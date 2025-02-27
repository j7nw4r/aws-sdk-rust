// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configuration_policies::_list_configuration_policies_output::ListConfigurationPoliciesOutputBuilder;

pub use crate::operation::list_configuration_policies::_list_configuration_policies_input::ListConfigurationPoliciesInputBuilder;

impl ListConfigurationPoliciesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_configuration_policies::ListConfigurationPoliciesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_configuration_policies::ListConfigurationPoliciesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_configuration_policies();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListConfigurationPolicies`.
///
/// <p> Lists the configuration policies that the Security Hub delegated administrator has created for your organization. Only the delegated administrator can invoke this operation from the home Region. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListConfigurationPoliciesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configuration_policies::builders::ListConfigurationPoliciesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_configuration_policies::ListConfigurationPoliciesOutput,
        crate::operation::list_configuration_policies::ListConfigurationPoliciesError,
    > for ListConfigurationPoliciesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_configuration_policies::ListConfigurationPoliciesOutput,
            crate::operation::list_configuration_policies::ListConfigurationPoliciesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListConfigurationPoliciesFluentBuilder {
    /// Creates a new `ListConfigurationPolicies`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListConfigurationPolicies as a reference.
    pub fn as_input(&self) -> &crate::operation::list_configuration_policies::builders::ListConfigurationPoliciesInputBuilder {
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
        crate::operation::list_configuration_policies::ListConfigurationPoliciesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_configuration_policies::ListConfigurationPoliciesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_configuration_policies::ListConfigurationPolicies::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_configuration_policies::ListConfigurationPolicies::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_configuration_policies::ListConfigurationPoliciesOutput,
        crate::operation::list_configuration_policies::ListConfigurationPoliciesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_configuration_policies::paginator::ListConfigurationPoliciesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_configuration_policies::paginator::ListConfigurationPoliciesPaginator {
        crate::operation::list_configuration_policies::paginator::ListConfigurationPoliciesPaginator::new(self.handle, self.inner)
    }
    /// <p> The NextToken value that's returned from a previous paginated <code>ListConfigurationPolicies</code> request where <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the end of the previous response that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p> The NextToken value that's returned from a previous paginated <code>ListConfigurationPolicies</code> request where <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the end of the previous response that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p> The NextToken value that's returned from a previous paginated <code>ListConfigurationPolicies</code> request where <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the <code>MaxResults</code> was used but the results exceeded the value of that parameter. Pagination continues from the end of the previous response that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p> The maximum number of results that's returned by <code>ListConfigurationPolicies</code> in each page of the response. When this parameter is used, <code>ListConfigurationPolicies</code> returns the specified number of results in a single page and a <code>NextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListConfigurationPolicies</code> request with the returned <code>NextToken</code> value. A valid range for <code>MaxResults</code> is between 1 and 100. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p> The maximum number of results that's returned by <code>ListConfigurationPolicies</code> in each page of the response. When this parameter is used, <code>ListConfigurationPolicies</code> returns the specified number of results in a single page and a <code>NextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListConfigurationPolicies</code> request with the returned <code>NextToken</code> value. A valid range for <code>MaxResults</code> is between 1 and 100. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p> The maximum number of results that's returned by <code>ListConfigurationPolicies</code> in each page of the response. When this parameter is used, <code>ListConfigurationPolicies</code> returns the specified number of results in a single page and a <code>NextToken</code> response element. You can see the remaining results of the initial request by sending another <code>ListConfigurationPolicies</code> request with the returned <code>NextToken</code> value. A valid range for <code>MaxResults</code> is between 1 and 100. </p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
