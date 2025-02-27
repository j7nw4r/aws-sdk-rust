// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_distributions_by_cache_policy_id::_list_distributions_by_cache_policy_id_output::ListDistributionsByCachePolicyIdOutputBuilder;

pub use crate::operation::list_distributions_by_cache_policy_id::_list_distributions_by_cache_policy_id_input::ListDistributionsByCachePolicyIdInputBuilder;

impl ListDistributionsByCachePolicyIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_distributions_by_cache_policy_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDistributionsByCachePolicyId`.
///
/// <p>Gets a list of distribution IDs for distributions that have a cache behavior that's associated with the specified cache policy.</p>
/// <p>You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the response is paginated. To get the next page of items, send a subsequent request that specifies the <code>NextMarker</code> value from the current response as the <code>Marker</code> value in the subsequent request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDistributionsByCachePolicyIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_distributions_by_cache_policy_id::builders::ListDistributionsByCachePolicyIdInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdOutput,
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdError,
    > for ListDistributionsByCachePolicyIdFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdOutput,
            crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDistributionsByCachePolicyIdFluentBuilder {
    /// Creates a new `ListDistributionsByCachePolicyId`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDistributionsByCachePolicyId as a reference.
    pub fn as_input(&self) -> &crate::operation::list_distributions_by_cache_policy_id::builders::ListDistributionsByCachePolicyIdInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyId::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyId::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdOutput,
        crate::operation::list_distributions_by_cache_policy_id::ListDistributionsByCachePolicyIdError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list of distribution IDs. The response includes distribution IDs in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of distribution IDs that you want in the response.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
    /// <p>The ID of the cache policy whose associated distribution IDs you want to list.</p>
    pub fn cache_policy_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cache_policy_id(input.into());
        self
    }
    /// <p>The ID of the cache policy whose associated distribution IDs you want to list.</p>
    pub fn set_cache_policy_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cache_policy_id(input);
        self
    }
    /// <p>The ID of the cache policy whose associated distribution IDs you want to list.</p>
    pub fn get_cache_policy_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cache_policy_id()
    }
}
