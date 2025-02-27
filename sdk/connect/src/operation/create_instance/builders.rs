// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_instance::_create_instance_output::CreateInstanceOutputBuilder;

pub use crate::operation::create_instance::_create_instance_input::CreateInstanceInputBuilder;

impl CreateInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_instance::CreateInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_instance::CreateInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateInstance`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Initiates an Amazon Connect instance with all the supported channels enabled. It does not attach any storage, such as Amazon Simple Storage Service (Amazon S3) or Amazon Kinesis. It also does not allow for any configurations on features, such as Contact Lens for Amazon Connect. </p>
/// <p>Amazon Connect enforces a limit on the total number of instances that you can create or delete in 30 days. If you exceed this limit, you will get an error message indicating there has been an excessive number of attempts at creating or deleting instances. You must wait 30 days before you can restart creating and deleting instances in your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_instance::builders::CreateInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_instance::CreateInstanceOutput,
        crate::operation::create_instance::CreateInstanceError,
    > for CreateInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_instance::CreateInstanceOutput,
            crate::operation::create_instance::CreateInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateInstanceFluentBuilder {
    /// Creates a new `CreateInstance`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::create_instance::builders::CreateInstanceInputBuilder {
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
        crate::operation::create_instance::CreateInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_instance::CreateInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_instance::CreateInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_instance::CreateInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_instance::CreateInstanceOutput,
        crate::operation::create_instance::CreateInstanceError,
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
    /// <p>The idempotency token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The idempotency token.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The type of identity management for your Amazon Connect users.</p>
    pub fn identity_management_type(mut self, input: crate::types::DirectoryType) -> Self {
        self.inner = self.inner.identity_management_type(input);
        self
    }
    /// <p>The type of identity management for your Amazon Connect users.</p>
    pub fn set_identity_management_type(mut self, input: ::std::option::Option<crate::types::DirectoryType>) -> Self {
        self.inner = self.inner.set_identity_management_type(input);
        self
    }
    /// <p>The type of identity management for your Amazon Connect users.</p>
    pub fn get_identity_management_type(&self) -> &::std::option::Option<crate::types::DirectoryType> {
        self.inner.get_identity_management_type()
    }
    /// <p>The name for your instance.</p>
    pub fn instance_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_alias(input.into());
        self
    }
    /// <p>The name for your instance.</p>
    pub fn set_instance_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_alias(input);
        self
    }
    /// <p>The name for your instance.</p>
    pub fn get_instance_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_alias()
    }
    /// <p>The identifier for the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier for the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier for the directory.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>Your contact center handles incoming contacts.</p>
    pub fn inbound_calls_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.inbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center handles incoming contacts.</p>
    pub fn set_inbound_calls_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_inbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center handles incoming contacts.</p>
    pub fn get_inbound_calls_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_inbound_calls_enabled()
    }
    /// <p>Your contact center allows outbound calls.</p>
    pub fn outbound_calls_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.outbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center allows outbound calls.</p>
    pub fn set_outbound_calls_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_outbound_calls_enabled(input);
        self
    }
    /// <p>Your contact center allows outbound calls.</p>
    pub fn get_outbound_calls_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_outbound_calls_enabled()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource. For example, <code>{ "tags": {"key1":"value1", "key2":"value2"} }</code>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource. For example, <code>{ "tags": {"key1":"value1", "key2":"value2"} }</code>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource. For example, <code>{ "tags": {"key1":"value1", "key2":"value2"} }</code>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
