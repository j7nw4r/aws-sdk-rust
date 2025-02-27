// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_workspaces::_stop_workspaces_output::StopWorkspacesOutputBuilder;

pub use crate::operation::stop_workspaces::_stop_workspaces_input::StopWorkspacesInputBuilder;

impl StopWorkspacesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_workspaces::StopWorkspacesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_workspaces::StopWorkspacesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_workspaces();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopWorkspaces`.
///
/// <p> Stops the specified WorkSpaces.</p>
/// <p>You cannot stop a WorkSpace unless it has a running mode of <code>AutoStop</code> and a state of <code>AVAILABLE</code>, <code>IMPAIRED</code>, <code>UNHEALTHY</code>, or <code>ERROR</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopWorkspacesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_workspaces::builders::StopWorkspacesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_workspaces::StopWorkspacesOutput,
        crate::operation::stop_workspaces::StopWorkspacesError,
    > for StopWorkspacesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_workspaces::StopWorkspacesOutput,
            crate::operation::stop_workspaces::StopWorkspacesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopWorkspacesFluentBuilder {
    /// Creates a new `StopWorkspaces`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopWorkspaces as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_workspaces::builders::StopWorkspacesInputBuilder {
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
        crate::operation::stop_workspaces::StopWorkspacesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_workspaces::StopWorkspacesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_workspaces::StopWorkspaces::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_workspaces::StopWorkspaces::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_workspaces::StopWorkspacesOutput,
        crate::operation::stop_workspaces::StopWorkspacesError,
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
    /// Appends an item to `StopWorkspaceRequests`.
    ///
    /// To override the contents of this collection use [`set_stop_workspace_requests`](Self::set_stop_workspace_requests).
    ///
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    pub fn stop_workspace_requests(mut self, input: crate::types::StopRequest) -> Self {
        self.inner = self.inner.stop_workspace_requests(input);
        self
    }
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    pub fn set_stop_workspace_requests(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::StopRequest>>) -> Self {
        self.inner = self.inner.set_stop_workspace_requests(input);
        self
    }
    /// <p>The WorkSpaces to stop. You can specify up to 25 WorkSpaces.</p>
    pub fn get_stop_workspace_requests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::StopRequest>> {
        self.inner.get_stop_workspace_requests()
    }
}
