// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_maintenance_window_executions::_describe_maintenance_window_executions_output::DescribeMaintenanceWindowExecutionsOutputBuilder;

pub use crate::operation::describe_maintenance_window_executions::_describe_maintenance_window_executions_input::DescribeMaintenanceWindowExecutionsInputBuilder;

impl DescribeMaintenanceWindowExecutionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_maintenance_window_executions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeMaintenanceWindowExecutions`.
///
/// <p>Lists the executions of a maintenance window. This includes information about when the maintenance window was scheduled to be active, and information about tasks registered and run with the maintenance window.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeMaintenanceWindowExecutionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_maintenance_window_executions::builders::DescribeMaintenanceWindowExecutionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput,
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError,
    > for DescribeMaintenanceWindowExecutionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput,
            crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeMaintenanceWindowExecutionsFluentBuilder {
    /// Creates a new `DescribeMaintenanceWindowExecutions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeMaintenanceWindowExecutions as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_maintenance_window_executions::builders::DescribeMaintenanceWindowExecutionsInputBuilder {
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
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutions::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput,
        crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator {
        crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    pub fn window_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.window_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    pub fn set_window_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_window_id(input);
        self
    }
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    pub fn get_window_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_window_id()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Each entry in the array is a structure containing:</p>
    /// <ul>
    /// <li> <p>Key. A string between 1 and 128 characters. Supported keys include <code>ExecutedBefore</code> and <code>ExecutedAfter</code>.</p> </li>
    /// <li> <p>Values. An array of strings, each between 1 and 256 characters. Supported values are date/time strings in a valid ISO 8601 date/time format, such as <code>2021-11-04T05:00:00Z</code>.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::MaintenanceWindowFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Each entry in the array is a structure containing:</p>
    /// <ul>
    /// <li> <p>Key. A string between 1 and 128 characters. Supported keys include <code>ExecutedBefore</code> and <code>ExecutedAfter</code>.</p> </li>
    /// <li> <p>Values. An array of strings, each between 1 and 256 characters. Supported values are date/time strings in a valid ISO 8601 date/time format, such as <code>2021-11-04T05:00:00Z</code>.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MaintenanceWindowFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Each entry in the array is a structure containing:</p>
    /// <ul>
    /// <li> <p>Key. A string between 1 and 128 characters. Supported keys include <code>ExecutedBefore</code> and <code>ExecutedAfter</code>.</p> </li>
    /// <li> <p>Values. An array of strings, each between 1 and 256 characters. Supported values are date/time strings in a valid ISO 8601 date/time format, such as <code>2021-11-04T05:00:00Z</code>.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MaintenanceWindowFilter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
