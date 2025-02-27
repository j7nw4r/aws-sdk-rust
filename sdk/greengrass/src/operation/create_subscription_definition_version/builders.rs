// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_subscription_definition_version::_create_subscription_definition_version_output::CreateSubscriptionDefinitionVersionOutputBuilder;

pub use crate::operation::create_subscription_definition_version::_create_subscription_definition_version_input::CreateSubscriptionDefinitionVersionInputBuilder;

impl CreateSubscriptionDefinitionVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_subscription_definition_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSubscriptionDefinitionVersion`.
///
/// Creates a version of a subscription definition which has already been defined.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSubscriptionDefinitionVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_subscription_definition_version::builders::CreateSubscriptionDefinitionVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionOutput,
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionError,
    > for CreateSubscriptionDefinitionVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionOutput,
            crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSubscriptionDefinitionVersionFluentBuilder {
    /// Creates a new `CreateSubscriptionDefinitionVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSubscriptionDefinitionVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::create_subscription_definition_version::builders::CreateSubscriptionDefinitionVersionInputBuilder {
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
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersion::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionOutput,
        crate::operation::create_subscription_definition_version::CreateSubscriptionDefinitionVersionError,
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
    /// A client token used to correlate requests and responses.
    pub fn amzn_client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.amzn_client_token(input.into());
        self
    }
    /// A client token used to correlate requests and responses.
    pub fn set_amzn_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_amzn_client_token(input);
        self
    }
    /// A client token used to correlate requests and responses.
    pub fn get_amzn_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_amzn_client_token()
    }
    /// The ID of the subscription definition.
    pub fn subscription_definition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subscription_definition_id(input.into());
        self
    }
    /// The ID of the subscription definition.
    pub fn set_subscription_definition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subscription_definition_id(input);
        self
    }
    /// The ID of the subscription definition.
    pub fn get_subscription_definition_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subscription_definition_id()
    }
    /// Appends an item to `Subscriptions`.
    ///
    /// To override the contents of this collection use [`set_subscriptions`](Self::set_subscriptions).
    ///
    /// A list of subscriptions.
    pub fn subscriptions(mut self, input: crate::types::Subscription) -> Self {
        self.inner = self.inner.subscriptions(input);
        self
    }
    /// A list of subscriptions.
    pub fn set_subscriptions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Subscription>>) -> Self {
        self.inner = self.inner.set_subscriptions(input);
        self
    }
    /// A list of subscriptions.
    pub fn get_subscriptions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Subscription>> {
        self.inner.get_subscriptions()
    }
}
