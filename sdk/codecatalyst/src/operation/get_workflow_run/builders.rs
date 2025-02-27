// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_workflow_run::_get_workflow_run_output::GetWorkflowRunOutputBuilder;

pub use crate::operation::get_workflow_run::_get_workflow_run_input::GetWorkflowRunInputBuilder;

impl GetWorkflowRunInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_workflow_run::GetWorkflowRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_run::GetWorkflowRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_workflow_run();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetWorkflowRun`.
///
/// <p>Returns information about a specified run of a workflow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetWorkflowRunFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_workflow_run::builders::GetWorkflowRunInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_workflow_run::GetWorkflowRunOutput,
        crate::operation::get_workflow_run::GetWorkflowRunError,
    > for GetWorkflowRunFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_workflow_run::GetWorkflowRunOutput,
            crate::operation::get_workflow_run::GetWorkflowRunError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetWorkflowRunFluentBuilder {
    /// Creates a new `GetWorkflowRun`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetWorkflowRun as a reference.
    pub fn as_input(&self) -> &crate::operation::get_workflow_run::builders::GetWorkflowRunInputBuilder {
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
        crate::operation::get_workflow_run::GetWorkflowRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_workflow_run::GetWorkflowRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_workflow_run::GetWorkflowRun::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_workflow_run::GetWorkflowRun::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_workflow_run::GetWorkflowRunOutput,
        crate::operation::get_workflow_run::GetWorkflowRunError,
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
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_name()
    }
    /// <p>The ID of the workflow run. To retrieve a list of workflow run IDs, use <code>ListWorkflowRuns</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the workflow run. To retrieve a list of workflow run IDs, use <code>ListWorkflowRuns</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the workflow run. To retrieve a list of workflow run IDs, use <code>ListWorkflowRuns</code>.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
}
