// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_risk_configuration::_set_risk_configuration_output::SetRiskConfigurationOutputBuilder;

pub use crate::operation::set_risk_configuration::_set_risk_configuration_input::SetRiskConfigurationInputBuilder;

impl SetRiskConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_risk_configuration::SetRiskConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_risk_configuration::SetRiskConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_risk_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetRiskConfiguration`.
///
/// <p>Configures actions on detected risks. To delete the risk configuration for <code>UserPoolId</code> or <code>ClientId</code>, pass null values for all four configuration types.</p>
/// <p>To activate Amazon Cognito advanced security features, update the user pool to include the <code>UserPoolAddOns</code> key<code>AdvancedSecurityMode</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetRiskConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_risk_configuration::builders::SetRiskConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_risk_configuration::SetRiskConfigurationOutput,
        crate::operation::set_risk_configuration::SetRiskConfigurationError,
    > for SetRiskConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_risk_configuration::SetRiskConfigurationOutput,
            crate::operation::set_risk_configuration::SetRiskConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetRiskConfigurationFluentBuilder {
    /// Creates a new `SetRiskConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetRiskConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::set_risk_configuration::builders::SetRiskConfigurationInputBuilder {
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
        crate::operation::set_risk_configuration::SetRiskConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_risk_configuration::SetRiskConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_risk_configuration::SetRiskConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_risk_configuration::SetRiskConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_risk_configuration::SetRiskConfigurationOutput,
        crate::operation::set_risk_configuration::SetRiskConfigurationError,
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
    /// <p>The user pool ID. </p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID. </p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user pool ID. </p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The app client ID. If <code>ClientId</code> is null, then the risk configuration is mapped to <code>userPoolId</code>. When the client ID is null, the same risk configuration is applied to all the clients in the userPool.</p>
    /// <p>Otherwise, <code>ClientId</code> is mapped to the client. When the client ID isn't null, the user pool configuration is overridden and the risk configuration for the client is used instead.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The app client ID. If <code>ClientId</code> is null, then the risk configuration is mapped to <code>userPoolId</code>. When the client ID is null, the same risk configuration is applied to all the clients in the userPool.</p>
    /// <p>Otherwise, <code>ClientId</code> is mapped to the client. When the client ID isn't null, the user pool configuration is overridden and the risk configuration for the client is used instead.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
    /// <p>The app client ID. If <code>ClientId</code> is null, then the risk configuration is mapped to <code>userPoolId</code>. When the client ID is null, the same risk configuration is applied to all the clients in the userPool.</p>
    /// <p>Otherwise, <code>ClientId</code> is mapped to the client. When the client ID isn't null, the user pool configuration is overridden and the risk configuration for the client is used instead.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_id()
    }
    /// <p>The compromised credentials risk configuration.</p>
    pub fn compromised_credentials_risk_configuration(mut self, input: crate::types::CompromisedCredentialsRiskConfigurationType) -> Self {
        self.inner = self.inner.compromised_credentials_risk_configuration(input);
        self
    }
    /// <p>The compromised credentials risk configuration.</p>
    pub fn set_compromised_credentials_risk_configuration(
        mut self,
        input: ::std::option::Option<crate::types::CompromisedCredentialsRiskConfigurationType>,
    ) -> Self {
        self.inner = self.inner.set_compromised_credentials_risk_configuration(input);
        self
    }
    /// <p>The compromised credentials risk configuration.</p>
    pub fn get_compromised_credentials_risk_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::CompromisedCredentialsRiskConfigurationType> {
        self.inner.get_compromised_credentials_risk_configuration()
    }
    /// <p>The account takeover risk configuration.</p>
    pub fn account_takeover_risk_configuration(mut self, input: crate::types::AccountTakeoverRiskConfigurationType) -> Self {
        self.inner = self.inner.account_takeover_risk_configuration(input);
        self
    }
    /// <p>The account takeover risk configuration.</p>
    pub fn set_account_takeover_risk_configuration(
        mut self,
        input: ::std::option::Option<crate::types::AccountTakeoverRiskConfigurationType>,
    ) -> Self {
        self.inner = self.inner.set_account_takeover_risk_configuration(input);
        self
    }
    /// <p>The account takeover risk configuration.</p>
    pub fn get_account_takeover_risk_configuration(&self) -> &::std::option::Option<crate::types::AccountTakeoverRiskConfigurationType> {
        self.inner.get_account_takeover_risk_configuration()
    }
    /// <p>The configuration to override the risk decision.</p>
    pub fn risk_exception_configuration(mut self, input: crate::types::RiskExceptionConfigurationType) -> Self {
        self.inner = self.inner.risk_exception_configuration(input);
        self
    }
    /// <p>The configuration to override the risk decision.</p>
    pub fn set_risk_exception_configuration(mut self, input: ::std::option::Option<crate::types::RiskExceptionConfigurationType>) -> Self {
        self.inner = self.inner.set_risk_exception_configuration(input);
        self
    }
    /// <p>The configuration to override the risk decision.</p>
    pub fn get_risk_exception_configuration(&self) -> &::std::option::Option<crate::types::RiskExceptionConfigurationType> {
        self.inner.get_risk_exception_configuration()
    }
}
