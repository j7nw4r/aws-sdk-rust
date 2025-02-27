// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_events_configuration::_put_events_configuration_output::PutEventsConfigurationOutputBuilder;

pub use crate::operation::put_events_configuration::_put_events_configuration_input::PutEventsConfigurationInputBuilder;

impl PutEventsConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_events_configuration::PutEventsConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_events_configuration::PutEventsConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_events_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutEventsConfiguration`.
///
/// <p>Creates an events configuration that allows a bot to receive outgoing events sent by Amazon Chime. Choose either an HTTPS endpoint or a Lambda function ARN. For more information, see <code>Bot</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutEventsConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_events_configuration::builders::PutEventsConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_events_configuration::PutEventsConfigurationOutput,
        crate::operation::put_events_configuration::PutEventsConfigurationError,
    > for PutEventsConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_events_configuration::PutEventsConfigurationOutput,
            crate::operation::put_events_configuration::PutEventsConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutEventsConfigurationFluentBuilder {
    /// Creates a new `PutEventsConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutEventsConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_events_configuration::builders::PutEventsConfigurationInputBuilder {
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
        crate::operation::put_events_configuration::PutEventsConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_events_configuration::PutEventsConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_events_configuration::PutEventsConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_events_configuration::PutEventsConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_events_configuration::PutEventsConfigurationOutput,
        crate::operation::put_events_configuration::PutEventsConfigurationError,
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
    /// <p>The Amazon Chime account ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>The bot ID.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The bot ID.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The bot ID.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>HTTPS endpoint that allows the bot to receive outgoing events.</p>
    pub fn outbound_events_https_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outbound_events_https_endpoint(input.into());
        self
    }
    /// <p>HTTPS endpoint that allows the bot to receive outgoing events.</p>
    pub fn set_outbound_events_https_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outbound_events_https_endpoint(input);
        self
    }
    /// <p>HTTPS endpoint that allows the bot to receive outgoing events.</p>
    pub fn get_outbound_events_https_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outbound_events_https_endpoint()
    }
    /// <p>Lambda function ARN that allows the bot to receive outgoing events.</p>
    pub fn lambda_function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lambda_function_arn(input.into());
        self
    }
    /// <p>Lambda function ARN that allows the bot to receive outgoing events.</p>
    pub fn set_lambda_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lambda_function_arn(input);
        self
    }
    /// <p>Lambda function ARN that allows the bot to receive outgoing events.</p>
    pub fn get_lambda_function_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lambda_function_arn()
    }
}
