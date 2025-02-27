// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::notify_provision_product_engine_workflow_result::_notify_provision_product_engine_workflow_result_output::NotifyProvisionProductEngineWorkflowResultOutputBuilder;

pub use crate::operation::notify_provision_product_engine_workflow_result::_notify_provision_product_engine_workflow_result_input::NotifyProvisionProductEngineWorkflowResultInputBuilder;

impl NotifyProvisionProductEngineWorkflowResultInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.notify_provision_product_engine_workflow_result();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `NotifyProvisionProductEngineWorkflowResult`.
///
/// <p> Notifies the result of the provisioning engine execution. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct NotifyProvisionProductEngineWorkflowResultFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::notify_provision_product_engine_workflow_result::builders::NotifyProvisionProductEngineWorkflowResultInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultOutput,
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultError,
    > for NotifyProvisionProductEngineWorkflowResultFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultOutput,
            crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl NotifyProvisionProductEngineWorkflowResultFluentBuilder {
    /// Creates a new `NotifyProvisionProductEngineWorkflowResult`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the NotifyProvisionProductEngineWorkflowResult as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::notify_provision_product_engine_workflow_result::builders::NotifyProvisionProductEngineWorkflowResultInputBuilder {
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
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResult::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResult::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultOutput,
        crate::operation::notify_provision_product_engine_workflow_result::NotifyProvisionProductEngineWorkflowResultError,
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
    /// <p> The encrypted contents of the provisioning engine execution payload that Service Catalog sends after the Terraform product provisioning workflow starts. </p>
    pub fn workflow_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_token(input.into());
        self
    }
    /// <p> The encrypted contents of the provisioning engine execution payload that Service Catalog sends after the Terraform product provisioning workflow starts. </p>
    pub fn set_workflow_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_token(input);
        self
    }
    /// <p> The encrypted contents of the provisioning engine execution payload that Service Catalog sends after the Terraform product provisioning workflow starts. </p>
    pub fn get_workflow_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_token()
    }
    /// <p> The identifier of the record. </p>
    pub fn record_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.record_id(input.into());
        self
    }
    /// <p> The identifier of the record. </p>
    pub fn set_record_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_record_id(input);
        self
    }
    /// <p> The identifier of the record. </p>
    pub fn get_record_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_record_id()
    }
    /// <p> The status of the provisioning engine execution. </p>
    pub fn status(mut self, input: crate::types::EngineWorkflowStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p> The status of the provisioning engine execution. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::EngineWorkflowStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p> The status of the provisioning engine execution. </p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::EngineWorkflowStatus> {
        self.inner.get_status()
    }
    /// <p> The reason why the provisioning engine execution failed. </p>
    pub fn failure_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.failure_reason(input.into());
        self
    }
    /// <p> The reason why the provisioning engine execution failed. </p>
    pub fn set_failure_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_failure_reason(input);
        self
    }
    /// <p> The reason why the provisioning engine execution failed. </p>
    pub fn get_failure_reason(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_failure_reason()
    }
    /// <p> The ID for the provisioned product resources that are part of a resource group. </p>
    pub fn resource_identifier(mut self, input: crate::types::EngineWorkflowResourceIdentifier) -> Self {
        self.inner = self.inner.resource_identifier(input);
        self
    }
    /// <p> The ID for the provisioned product resources that are part of a resource group. </p>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<crate::types::EngineWorkflowResourceIdentifier>) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// <p> The ID for the provisioned product resources that are part of a resource group. </p>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<crate::types::EngineWorkflowResourceIdentifier> {
        self.inner.get_resource_identifier()
    }
    /// Appends an item to `Outputs`.
    ///
    /// To override the contents of this collection use [`set_outputs`](Self::set_outputs).
    ///
    /// <p> The output of the provisioning engine execution. </p>
    pub fn outputs(mut self, input: crate::types::RecordOutput) -> Self {
        self.inner = self.inner.outputs(input);
        self
    }
    /// <p> The output of the provisioning engine execution. </p>
    pub fn set_outputs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RecordOutput>>) -> Self {
        self.inner = self.inner.set_outputs(input);
        self
    }
    /// <p> The output of the provisioning engine execution. </p>
    pub fn get_outputs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RecordOutput>> {
        self.inner.get_outputs()
    }
    /// <p> The idempotency token that identifies the provisioning engine execution. </p>
    pub fn idempotency_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p> The idempotency token that identifies the provisioning engine execution. </p>
    pub fn set_idempotency_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p> The idempotency token that identifies the provisioning engine execution. </p>
    pub fn get_idempotency_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_idempotency_token()
    }
}
