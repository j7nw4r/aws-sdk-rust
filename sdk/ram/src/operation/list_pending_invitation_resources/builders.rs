// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_pending_invitation_resources::_list_pending_invitation_resources_output::ListPendingInvitationResourcesOutputBuilder;

pub use crate::operation::list_pending_invitation_resources::_list_pending_invitation_resources_input::ListPendingInvitationResourcesInputBuilder;

impl ListPendingInvitationResourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_pending_invitation_resources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPendingInvitationResources`.
///
/// <p>Lists the resources in a resource share that is shared with you but for which the invitation is still <code>PENDING</code>. That means that you haven't accepted or rejected the invitation and the invitation hasn't expired.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPendingInvitationResourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_pending_invitation_resources::builders::ListPendingInvitationResourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesOutput,
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesError,
    > for ListPendingInvitationResourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesOutput,
            crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPendingInvitationResourcesFluentBuilder {
    /// Creates a new `ListPendingInvitationResources`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPendingInvitationResources as a reference.
    pub fn as_input(&self) -> &crate::operation::list_pending_invitation_resources::builders::ListPendingInvitationResourcesInputBuilder {
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
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_pending_invitation_resources::ListPendingInvitationResources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesOutput,
        crate::operation::list_pending_invitation_resources::ListPendingInvitationResourcesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_pending_invitation_resources::paginator::ListPendingInvitationResourcesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_pending_invitation_resources::paginator::ListPendingInvitationResourcesPaginator {
        crate::operation::list_pending_invitation_resources::paginator::ListPendingInvitationResourcesPaginator::new(self.handle, self.inner)
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the invitation. You can use <code>GetResourceShareInvitations</code> to find the ARN of the invitation.</p>
    pub fn resource_share_invitation_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_share_invitation_arn(input.into());
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the invitation. You can use <code>GetResourceShareInvitations</code> to find the ARN of the invitation.</p>
    pub fn set_resource_share_invitation_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_share_invitation_arn(input);
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of the invitation. You can use <code>GetResourceShareInvitations</code> to find the ARN of the invitation.</p>
    pub fn get_resource_share_invitation_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_share_invitation_arn()
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>NextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>NextToken</code> response to request the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Specifies the total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the number you specify, the <code>NextToken</code> response element is returned with a value (not null). Include the specified value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Specifies that you want the results to include only resources that have the specified scope.</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – the results include both global and regional resources or resource types.</p> </li>
    /// <li> <p> <code>GLOBAL</code> – the results include only global resources or resource types.</p> </li>
    /// <li> <p> <code>REGIONAL</code> – the results include only regional resources or resource types.</p> </li>
    /// </ul>
    /// <p>The default value is <code>ALL</code>.</p>
    pub fn resource_region_scope(mut self, input: crate::types::ResourceRegionScopeFilter) -> Self {
        self.inner = self.inner.resource_region_scope(input);
        self
    }
    /// <p>Specifies that you want the results to include only resources that have the specified scope.</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – the results include both global and regional resources or resource types.</p> </li>
    /// <li> <p> <code>GLOBAL</code> – the results include only global resources or resource types.</p> </li>
    /// <li> <p> <code>REGIONAL</code> – the results include only regional resources or resource types.</p> </li>
    /// </ul>
    /// <p>The default value is <code>ALL</code>.</p>
    pub fn set_resource_region_scope(mut self, input: ::std::option::Option<crate::types::ResourceRegionScopeFilter>) -> Self {
        self.inner = self.inner.set_resource_region_scope(input);
        self
    }
    /// <p>Specifies that you want the results to include only resources that have the specified scope.</p>
    /// <ul>
    /// <li> <p> <code>ALL</code> – the results include both global and regional resources or resource types.</p> </li>
    /// <li> <p> <code>GLOBAL</code> – the results include only global resources or resource types.</p> </li>
    /// <li> <p> <code>REGIONAL</code> – the results include only regional resources or resource types.</p> </li>
    /// </ul>
    /// <p>The default value is <code>ALL</code>.</p>
    pub fn get_resource_region_scope(&self) -> &::std::option::Option<crate::types::ResourceRegionScopeFilter> {
        self.inner.get_resource_region_scope()
    }
}
