// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_policy_generation::_start_policy_generation_output::StartPolicyGenerationOutputBuilder;

pub use crate::operation::start_policy_generation::_start_policy_generation_input::StartPolicyGenerationInputBuilder;

impl StartPolicyGenerationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_policy_generation::StartPolicyGenerationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_policy_generation::StartPolicyGenerationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_policy_generation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartPolicyGeneration`.
///
/// <p>Starts the policy generation request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartPolicyGenerationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_policy_generation::builders::StartPolicyGenerationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_policy_generation::StartPolicyGenerationOutput,
        crate::operation::start_policy_generation::StartPolicyGenerationError,
    > for StartPolicyGenerationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_policy_generation::StartPolicyGenerationOutput,
            crate::operation::start_policy_generation::StartPolicyGenerationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartPolicyGenerationFluentBuilder {
    /// Creates a new `StartPolicyGeneration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartPolicyGeneration as a reference.
    pub fn as_input(&self) -> &crate::operation::start_policy_generation::builders::StartPolicyGenerationInputBuilder {
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
        crate::operation::start_policy_generation::StartPolicyGenerationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_policy_generation::StartPolicyGenerationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_policy_generation::StartPolicyGeneration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_policy_generation::StartPolicyGeneration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_policy_generation::StartPolicyGenerationOutput,
        crate::operation::start_policy_generation::StartPolicyGenerationError,
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
    /// <p>Contains the ARN of the IAM entity (user or role) for which you are generating a policy.</p>
    pub fn policy_generation_details(mut self, input: crate::types::PolicyGenerationDetails) -> Self {
        self.inner = self.inner.policy_generation_details(input);
        self
    }
    /// <p>Contains the ARN of the IAM entity (user or role) for which you are generating a policy.</p>
    pub fn set_policy_generation_details(mut self, input: ::std::option::Option<crate::types::PolicyGenerationDetails>) -> Self {
        self.inner = self.inner.set_policy_generation_details(input);
        self
    }
    /// <p>Contains the ARN of the IAM entity (user or role) for which you are generating a policy.</p>
    pub fn get_policy_generation_details(&self) -> &::std::option::Option<crate::types::PolicyGenerationDetails> {
        self.inner.get_policy_generation_details()
    }
    /// <p>A <code>CloudTrailDetails</code> object that contains details about a <code>Trail</code> that you want to analyze to generate policies.</p>
    pub fn cloud_trail_details(mut self, input: crate::types::CloudTrailDetails) -> Self {
        self.inner = self.inner.cloud_trail_details(input);
        self
    }
    /// <p>A <code>CloudTrailDetails</code> object that contains details about a <code>Trail</code> that you want to analyze to generate policies.</p>
    pub fn set_cloud_trail_details(mut self, input: ::std::option::Option<crate::types::CloudTrailDetails>) -> Self {
        self.inner = self.inner.set_cloud_trail_details(input);
        self
    }
    /// <p>A <code>CloudTrailDetails</code> object that contains details about a <code>Trail</code> that you want to analyze to generate policies.</p>
    pub fn get_cloud_trail_details(&self) -> &::std::option::Option<crate::types::CloudTrailDetails> {
        self.inner.get_cloud_trail_details()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, the subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p>
    /// <p>If you do not specify a client token, one is automatically generated by the Amazon Web Services SDK.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, the subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p>
    /// <p>If you do not specify a client token, one is automatically generated by the Amazon Web Services SDK.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, the subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p>
    /// <p>If you do not specify a client token, one is automatically generated by the Amazon Web Services SDK.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
