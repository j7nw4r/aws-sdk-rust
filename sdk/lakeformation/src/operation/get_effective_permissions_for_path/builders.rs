// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_effective_permissions_for_path::_get_effective_permissions_for_path_output::GetEffectivePermissionsForPathOutputBuilder;

pub use crate::operation::get_effective_permissions_for_path::_get_effective_permissions_for_path_input::GetEffectivePermissionsForPathInputBuilder;

impl GetEffectivePermissionsForPathInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_effective_permissions_for_path();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEffectivePermissionsForPath`.
///
/// <p>Returns the Lake Formation permissions for a specified table or database resource located at a path in Amazon S3. <code>GetEffectivePermissionsForPath</code> will not return databases and tables if the catalog is encrypted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEffectivePermissionsForPathFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_effective_permissions_for_path::builders::GetEffectivePermissionsForPathInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathOutput,
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathError,
    > for GetEffectivePermissionsForPathFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathOutput,
            crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetEffectivePermissionsForPathFluentBuilder {
    /// Creates a new `GetEffectivePermissionsForPath`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetEffectivePermissionsForPath as a reference.
    pub fn as_input(&self) -> &crate::operation::get_effective_permissions_for_path::builders::GetEffectivePermissionsForPathInputBuilder {
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
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPath::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPath::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathOutput,
        crate::operation::get_effective_permissions_for_path::GetEffectivePermissionsForPathError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_effective_permissions_for_path::paginator::GetEffectivePermissionsForPathPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_effective_permissions_for_path::paginator::GetEffectivePermissionsForPathPaginator {
        crate::operation::get_effective_permissions_for_path::paginator::GetEffectivePermissionsForPathPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment. </p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get permissions.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get permissions.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to get permissions.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A continuation token, if this is not the first call to retrieve this list.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
