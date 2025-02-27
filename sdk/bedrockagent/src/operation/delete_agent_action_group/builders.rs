// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_agent_action_group::_delete_agent_action_group_output::DeleteAgentActionGroupOutputBuilder;

pub use crate::operation::delete_agent_action_group::_delete_agent_action_group_input::DeleteAgentActionGroupInputBuilder;

impl DeleteAgentActionGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_agent_action_group::DeleteAgentActionGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_agent_action_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteAgentActionGroup`.
///
/// Deletes an Action Group for existing Amazon Bedrock Agent.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteAgentActionGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_agent_action_group::builders::DeleteAgentActionGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupOutput,
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupError,
    > for DeleteAgentActionGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_agent_action_group::DeleteAgentActionGroupOutput,
            crate::operation::delete_agent_action_group::DeleteAgentActionGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteAgentActionGroupFluentBuilder {
    /// Creates a new `DeleteAgentActionGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteAgentActionGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_agent_action_group::builders::DeleteAgentActionGroupInputBuilder {
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
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_agent_action_group::DeleteAgentActionGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_agent_action_group::DeleteAgentActionGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_agent_action_group::DeleteAgentActionGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupOutput,
        crate::operation::delete_agent_action_group::DeleteAgentActionGroupError,
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
    /// Id generated at the server side when an Agent is created
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_id(input.into());
        self
    }
    /// Id generated at the server side when an Agent is created
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_id(input);
        self
    }
    /// Id generated at the server side when an Agent is created
    pub fn get_agent_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_id()
    }
    /// Draft Version of the Agent.
    pub fn agent_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_version(input.into());
        self
    }
    /// Draft Version of the Agent.
    pub fn set_agent_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_version(input);
        self
    }
    /// Draft Version of the Agent.
    pub fn get_agent_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_version()
    }
    /// Id generated at the server side when an Agent ActionGroup is created
    pub fn action_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action_group_id(input.into());
        self
    }
    /// Id generated at the server side when an Agent ActionGroup is created
    pub fn set_action_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_action_group_id(input);
        self
    }
    /// Id generated at the server side when an Agent ActionGroup is created
    pub fn get_action_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_action_group_id()
    }
    /// Skips checking if resource is in use when set to true. Defaults to false
    pub fn skip_resource_in_use_check(mut self, input: bool) -> Self {
        self.inner = self.inner.skip_resource_in_use_check(input);
        self
    }
    /// Skips checking if resource is in use when set to true. Defaults to false
    pub fn set_skip_resource_in_use_check(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_skip_resource_in_use_check(input);
        self
    }
    /// Skips checking if resource is in use when set to true. Defaults to false
    pub fn get_skip_resource_in_use_check(&self) -> &::std::option::Option<bool> {
        self.inner.get_skip_resource_in_use_check()
    }
}
