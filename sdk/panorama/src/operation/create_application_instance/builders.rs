// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_application_instance::_create_application_instance_output::CreateApplicationInstanceOutputBuilder;

pub use crate::operation::create_application_instance::_create_application_instance_input::CreateApplicationInstanceInputBuilder;

impl CreateApplicationInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_application_instance::CreateApplicationInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_instance::CreateApplicationInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_application_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateApplicationInstance`.
///
/// <p>Creates an application instance and deploys it to a device.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateApplicationInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_application_instance::builders::CreateApplicationInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_application_instance::CreateApplicationInstanceOutput,
        crate::operation::create_application_instance::CreateApplicationInstanceError,
    > for CreateApplicationInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_application_instance::CreateApplicationInstanceOutput,
            crate::operation::create_application_instance::CreateApplicationInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateApplicationInstanceFluentBuilder {
    /// Creates a new `CreateApplicationInstance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateApplicationInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::create_application_instance::builders::CreateApplicationInstanceInputBuilder {
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
        crate::operation::create_application_instance::CreateApplicationInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_instance::CreateApplicationInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_application_instance::CreateApplicationInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_application_instance::CreateApplicationInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_application_instance::CreateApplicationInstanceOutput,
        crate::operation::create_application_instance::CreateApplicationInstanceError,
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
    /// <p>A name for the application instance.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the application instance.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the application instance.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description for the application instance.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the application instance.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the application instance.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The application's manifest document.</p>
    pub fn manifest_payload(mut self, input: crate::types::ManifestPayload) -> Self {
        self.inner = self.inner.manifest_payload(input);
        self
    }
    /// <p>The application's manifest document.</p>
    pub fn set_manifest_payload(mut self, input: ::std::option::Option<crate::types::ManifestPayload>) -> Self {
        self.inner = self.inner.set_manifest_payload(input);
        self
    }
    /// <p>The application's manifest document.</p>
    pub fn get_manifest_payload(&self) -> &::std::option::Option<crate::types::ManifestPayload> {
        self.inner.get_manifest_payload()
    }
    /// <p>Setting overrides for the application manifest.</p>
    pub fn manifest_overrides_payload(mut self, input: crate::types::ManifestOverridesPayload) -> Self {
        self.inner = self.inner.manifest_overrides_payload(input);
        self
    }
    /// <p>Setting overrides for the application manifest.</p>
    pub fn set_manifest_overrides_payload(mut self, input: ::std::option::Option<crate::types::ManifestOverridesPayload>) -> Self {
        self.inner = self.inner.set_manifest_overrides_payload(input);
        self
    }
    /// <p>Setting overrides for the application manifest.</p>
    pub fn get_manifest_overrides_payload(&self) -> &::std::option::Option<crate::types::ManifestOverridesPayload> {
        self.inner.get_manifest_overrides_payload()
    }
    /// <p>The ID of an application instance to replace with the new instance.</p>
    pub fn application_instance_id_to_replace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_instance_id_to_replace(input.into());
        self
    }
    /// <p>The ID of an application instance to replace with the new instance.</p>
    pub fn set_application_instance_id_to_replace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_instance_id_to_replace(input);
        self
    }
    /// <p>The ID of an application instance to replace with the new instance.</p>
    pub fn get_application_instance_id_to_replace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_instance_id_to_replace()
    }
    /// <p>The ARN of a runtime role for the application instance.</p>
    pub fn runtime_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.runtime_role_arn(input.into());
        self
    }
    /// <p>The ARN of a runtime role for the application instance.</p>
    pub fn set_runtime_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_runtime_role_arn(input);
        self
    }
    /// <p>The ARN of a runtime role for the application instance.</p>
    pub fn get_runtime_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_runtime_role_arn()
    }
    /// <p>A device's ID.</p>
    pub fn default_runtime_context_device(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.default_runtime_context_device(input.into());
        self
    }
    /// <p>A device's ID.</p>
    pub fn set_default_runtime_context_device(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_default_runtime_context_device(input);
        self
    }
    /// <p>A device's ID.</p>
    pub fn get_default_runtime_context_device(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_default_runtime_context_device()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags for the application instance.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Tags for the application instance.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags for the application instance.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
