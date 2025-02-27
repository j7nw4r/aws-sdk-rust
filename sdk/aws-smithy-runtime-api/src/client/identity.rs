/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::box_error::BoxError;
use crate::client::auth::AuthSchemeId;
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::{RuntimeComponents, RuntimeComponentsBuilder};
use crate::impl_shared_conversions;
use aws_smithy_types::config_bag::ConfigBag;
use std::any::Any;
use std::fmt;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

#[cfg(feature = "http-auth")]
pub mod http;

new_type_future! {
    #[doc = "Future for [`IdentityResolver::resolve_identity`]."]
    pub struct IdentityFuture<'a, Identity, BoxError>;
}

static NEXT_CACHE_PARTITION: AtomicUsize = AtomicUsize::new(0);

/// Cache partition key for identity caching.
///
/// Identities need cache partitioning because a single identity cache is used across
/// multiple identity providers across multiple auth schemes. In addition, a single auth scheme
/// may have many different identity providers due to operation-level config overrides.
///
/// This partition _must_ be respected when retrieving from the identity cache and _should_
/// be part of the cache key.
///
/// Calling [`IdentityCachePartition::new`] will create a new globally unique cache partition key,
/// and the [`SharedIdentityResolver`] will automatically create and store a partion on construction.
/// Thus, every configured identity resolver will be assigned a unique partition.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct IdentityCachePartition(usize);

impl IdentityCachePartition {
    /// Create a new globally unique cache partition key.
    pub fn new() -> Self {
        Self(NEXT_CACHE_PARTITION.fetch_add(1, Ordering::Relaxed))
    }

    /// Helper for unit tests to create an identity cache partition with a known value.
    #[cfg(feature = "test-util")]
    pub fn new_for_tests(value: usize) -> IdentityCachePartition {
        Self(value)
    }
}

/// Caching resolver for identities.
pub trait ResolveCachedIdentity: fmt::Debug + Send + Sync {
    /// Returns a cached identity, or resolves an identity and caches it if its not already cached.
    fn resolve_cached_identity<'a>(
        &'a self,
        resolver: SharedIdentityResolver,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a>;

    /// Validate the base client configuration for this implementation.
    ///
    /// This gets called upon client construction. The full config may not be available at
    /// this time (hence why it has [`RuntimeComponentsBuilder`] as an argument rather
    /// than [`RuntimeComponents`]). Any error returned here will become a panic
    /// in the client constructor.
    fn validate_base_client_config(
        &self,
        runtime_components: &RuntimeComponentsBuilder,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        let _ = (runtime_components, cfg);
        Ok(())
    }

    /// Validate the final client configuration for this implementation.
    ///
    /// This gets called immediately after the [`Intercept::read_before_execution`] trait hook
    /// when the final configuration has been resolved. Any error returned here will
    /// cause the operation to return that error.
    ///
    /// [`Intercept::read_before_execution`]: crate::client::interceptors::Intercept::read_before_execution
    fn validate_final_config(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        let _ = (runtime_components, cfg);
        Ok(())
    }
}

/// Shared identity cache.
#[derive(Clone, Debug)]
pub struct SharedIdentityCache(Arc<dyn ResolveCachedIdentity>);

impl SharedIdentityCache {
    /// Creates a new [`SharedIdentityCache`] from the given cache implementation.
    pub fn new(cache: impl ResolveCachedIdentity + 'static) -> Self {
        Self(Arc::new(cache))
    }
}

impl ResolveCachedIdentity for SharedIdentityCache {
    fn resolve_cached_identity<'a>(
        &'a self,
        resolver: SharedIdentityResolver,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        self.0
            .resolve_cached_identity(resolver, runtime_components, config_bag)
    }
}

impl ValidateConfig for SharedIdentityCache {
    fn validate_base_client_config(
        &self,
        runtime_components: &RuntimeComponentsBuilder,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        self.0.validate_base_client_config(runtime_components, cfg)
    }

    fn validate_final_config(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        self.0.validate_final_config(runtime_components, cfg)
    }
}

impl_shared_conversions!(convert SharedIdentityCache from ResolveCachedIdentity using SharedIdentityCache::new);

/// Resolver for identities.
///
/// Every [`AuthScheme`](crate::client::auth::AuthScheme) has one or more compatible
/// identity resolvers, which are selected from runtime components by the auth scheme
/// implementation itself.
///
/// The identity resolver must return an [`IdentityFuture`] with the resolved identity, or an error
/// if resolution failed. There is no optionality for identity resolvers. The identity either
/// resolves successfully, or it fails. The orchestrator will choose exactly one auth scheme
/// to use, and thus, its chosen identity resolver is the only identity resolver that runs.
/// There is no fallback to other auth schemes in the absence of an identity.
pub trait ResolveIdentity: Send + Sync + Debug {
    /// Asynchronously resolves an identity for a request using the given config.
    fn resolve_identity<'a>(
        &'a self,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a>;

    /// Returns a fallback identity.
    ///
    /// This method should be used as a fallback plan, i.e., when a call to `resolve_identity`
    /// is interrupted by a timeout and its future fails to complete.
    ///
    /// The fallback identity should be set aside and ready to be returned
    /// immediately. Therefore, a new identity should NOT be fetched
    /// within this method, which might cause a long-running operation.
    fn fallback_on_interrupt(&self) -> Option<Identity> {
        None
    }
}

/// Container for a shared identity resolver.
#[derive(Clone, Debug)]
pub struct SharedIdentityResolver {
    inner: Arc<dyn ResolveIdentity>,
    cache_partition: IdentityCachePartition,
}

impl SharedIdentityResolver {
    /// Creates a new [`SharedIdentityResolver`] from the given resolver.
    pub fn new(resolver: impl ResolveIdentity + 'static) -> Self {
        Self {
            inner: Arc::new(resolver),
            cache_partition: IdentityCachePartition::new(),
        }
    }

    /// Returns the globally unique cache partition key for this identity resolver.
    ///
    /// See the [`IdentityCachePartition`] docs for more information on what this is used for
    /// and why.
    pub fn cache_partition(&self) -> IdentityCachePartition {
        self.cache_partition
    }
}

impl ResolveIdentity for SharedIdentityResolver {
    fn resolve_identity<'a>(
        &'a self,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        self.inner.resolve_identity(runtime_components, config_bag)
    }
}

impl_shared_conversions!(convert SharedIdentityResolver from ResolveIdentity using SharedIdentityResolver::new);

/// An identity resolver paired with an auth scheme ID that it resolves for.
#[derive(Clone, Debug)]
pub(crate) struct ConfiguredIdentityResolver {
    auth_scheme: AuthSchemeId,
    identity_resolver: SharedIdentityResolver,
}

impl ConfiguredIdentityResolver {
    /// Creates a new [`ConfiguredIdentityResolver`] from the given auth scheme and identity resolver.
    pub(crate) fn new(
        auth_scheme: AuthSchemeId,
        identity_resolver: SharedIdentityResolver,
    ) -> Self {
        Self {
            auth_scheme,
            identity_resolver,
        }
    }

    /// Returns the auth scheme ID.
    pub(crate) fn scheme_id(&self) -> AuthSchemeId {
        self.auth_scheme
    }

    /// Returns the identity resolver.
    pub(crate) fn identity_resolver(&self) -> SharedIdentityResolver {
        self.identity_resolver.clone()
    }
}

impl ValidateConfig for ConfiguredIdentityResolver {}

/// An identity that can be used for authentication.
///
/// The [`Identity`] is a container for any arbitrary identity data that may be used
/// by a [`Sign`](crate::client::auth::Sign) implementation. Under the hood, it
/// has an `Arc<dyn Any>`, and it is the responsibility of the signer to downcast
/// to the appropriate data type using the `data()` function.
///
/// The `Identity` also holds an optional expiration time, which may duplicate
/// an expiration time on the identity data. This is because an `Arc<dyn Any>`
/// can't be downcast to any arbitrary trait, and expiring identities are
/// common enough to be built-in.
#[derive(Clone)]
pub struct Identity {
    data: Arc<dyn Any + Send + Sync>,
    #[allow(clippy::type_complexity)]
    data_debug: Arc<dyn (Fn(&Arc<dyn Any + Send + Sync>) -> &dyn Debug) + Send + Sync>,
    expiration: Option<SystemTime>,
}

impl Identity {
    /// Creates a new identity with the given data and expiration time.
    pub fn new<T>(data: T, expiration: Option<SystemTime>) -> Self
    where
        T: Any + Debug + Send + Sync,
    {
        Self {
            data: Arc::new(data),
            data_debug: Arc::new(|d| d.downcast_ref::<T>().expect("type-checked") as _),
            expiration,
        }
    }

    /// Returns the raw identity data.
    pub fn data<T: Any + Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.data.downcast_ref()
    }

    /// Returns the expiration time for this identity, if any.
    pub fn expiration(&self) -> Option<SystemTime> {
        self.expiration
    }
}

impl Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity")
            .field("data", (self.data_debug)(&self.data))
            .field("expiration", &self.expiration)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_async::time::{SystemTimeSource, TimeSource};

    #[test]
    fn check_send_sync() {
        fn is_send_sync<T: Send + Sync>(_: T) {}
        is_send_sync(Identity::new("foo", None));
    }

    #[test]
    fn create_retrieve_identity() {
        #[derive(Debug)]
        struct MyIdentityData {
            first: String,
            last: String,
        }

        let ts = SystemTimeSource::new();
        let expiration = ts.now();
        let identity = Identity::new(
            MyIdentityData {
                first: "foo".into(),
                last: "bar".into(),
            },
            Some(expiration),
        );

        assert_eq!("foo", identity.data::<MyIdentityData>().unwrap().first);
        assert_eq!("bar", identity.data::<MyIdentityData>().unwrap().last);
        assert_eq!(Some(expiration), identity.expiration());
    }
}
