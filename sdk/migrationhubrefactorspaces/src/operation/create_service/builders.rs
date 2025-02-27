// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_service::_create_service_output::CreateServiceOutputBuilder;

pub use crate::operation::create_service::_create_service_input::CreateServiceInputBuilder;

impl CreateServiceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_service::CreateServiceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service::CreateServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_service();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateService`.
///
/// <p>Creates an Amazon Web Services Migration Hub Refactor Spaces service. The account owner of the service is always the environment owner, regardless of which account in the environment creates the service. Services have either a URL endpoint in a virtual private cloud (VPC), or a Lambda function endpoint.</p> <important>
/// <p>If an Amazon Web Services resource is launched in a service VPC, and you want it to be accessible to all of an environment’s services with VPCs and routes, apply the <code>RefactorSpacesSecurityGroup</code> to the resource. Alternatively, to add more cross-account constraints, apply your own security group.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateServiceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_service::builders::CreateServiceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_service::CreateServiceOutput,
        crate::operation::create_service::CreateServiceError,
    > for CreateServiceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_service::CreateServiceOutput,
            crate::operation::create_service::CreateServiceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateServiceFluentBuilder {
    /// Creates a new `CreateService`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateService as a reference.
    pub fn as_input(&self) -> &crate::operation::create_service::builders::CreateServiceInputBuilder {
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
        crate::operation::create_service::CreateServiceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service::CreateServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_service::CreateService::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_service::CreateService::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_service::CreateServiceOutput,
        crate::operation::create_service::CreateServiceError,
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
    /// <p>The name of the service.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the service.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the service.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description of the service.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the service.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the service.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The ID of the environment in which the service is created.</p>
    pub fn environment_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The ID of the environment in which the service is created.</p>
    pub fn set_environment_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The ID of the environment in which the service is created.</p>
    pub fn get_environment_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_identifier()
    }
    /// <p>The ID of the application which the service is created.</p>
    pub fn application_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_identifier(input.into());
        self
    }
    /// <p>The ID of the application which the service is created.</p>
    pub fn set_application_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_identifier(input);
        self
    }
    /// <p>The ID of the application which the service is created.</p>
    pub fn get_application_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_identifier()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_id()
    }
    /// <p>The type of endpoint to use for the service. The type can be a URL in a VPC or an Lambda function.</p>
    pub fn endpoint_type(mut self, input: crate::types::ServiceEndpointType) -> Self {
        self.inner = self.inner.endpoint_type(input);
        self
    }
    /// <p>The type of endpoint to use for the service. The type can be a URL in a VPC or an Lambda function.</p>
    pub fn set_endpoint_type(mut self, input: ::std::option::Option<crate::types::ServiceEndpointType>) -> Self {
        self.inner = self.inner.set_endpoint_type(input);
        self
    }
    /// <p>The type of endpoint to use for the service. The type can be a URL in a VPC or an Lambda function.</p>
    pub fn get_endpoint_type(&self) -> &::std::option::Option<crate::types::ServiceEndpointType> {
        self.inner.get_endpoint_type()
    }
    /// <p>The configuration for the URL endpoint type. When creating a route to a service, Refactor Spaces automatically resolves the address in the <code>UrlEndpointInput</code> object URL when the Domain Name System (DNS) time-to-live (TTL) expires, or every 60 seconds for TTLs less than 60 seconds.</p>
    pub fn url_endpoint(mut self, input: crate::types::UrlEndpointInput) -> Self {
        self.inner = self.inner.url_endpoint(input);
        self
    }
    /// <p>The configuration for the URL endpoint type. When creating a route to a service, Refactor Spaces automatically resolves the address in the <code>UrlEndpointInput</code> object URL when the Domain Name System (DNS) time-to-live (TTL) expires, or every 60 seconds for TTLs less than 60 seconds.</p>
    pub fn set_url_endpoint(mut self, input: ::std::option::Option<crate::types::UrlEndpointInput>) -> Self {
        self.inner = self.inner.set_url_endpoint(input);
        self
    }
    /// <p>The configuration for the URL endpoint type. When creating a route to a service, Refactor Spaces automatically resolves the address in the <code>UrlEndpointInput</code> object URL when the Domain Name System (DNS) time-to-live (TTL) expires, or every 60 seconds for TTLs less than 60 seconds.</p>
    pub fn get_url_endpoint(&self) -> &::std::option::Option<crate::types::UrlEndpointInput> {
        self.inner.get_url_endpoint()
    }
    /// <p>The configuration for the Lambda endpoint type.</p>
    pub fn lambda_endpoint(mut self, input: crate::types::LambdaEndpointInput) -> Self {
        self.inner = self.inner.lambda_endpoint(input);
        self
    }
    /// <p>The configuration for the Lambda endpoint type.</p>
    pub fn set_lambda_endpoint(mut self, input: ::std::option::Option<crate::types::LambdaEndpointInput>) -> Self {
        self.inner = self.inner.set_lambda_endpoint(input);
        self
    }
    /// <p>The configuration for the Lambda endpoint type.</p>
    pub fn get_lambda_endpoint(&self) -> &::std::option::Option<crate::types::LambdaEndpointInput> {
        self.inner.get_lambda_endpoint()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the service. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to assign to the service. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to assign to the service. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
