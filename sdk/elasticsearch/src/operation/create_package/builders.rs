// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_package::_create_package_output::CreatePackageOutputBuilder;

pub use crate::operation::create_package::_create_package_input::CreatePackageInputBuilder;

impl CreatePackageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_package::CreatePackageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_package::CreatePackageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_package();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePackage`.
///
/// <p>Create a package for use with Amazon ES domains.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePackageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_package::builders::CreatePackageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_package::CreatePackageOutput,
        crate::operation::create_package::CreatePackageError,
    > for CreatePackageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_package::CreatePackageOutput,
            crate::operation::create_package::CreatePackageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePackageFluentBuilder {
    /// Creates a new `CreatePackage`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePackage as a reference.
    pub fn as_input(&self) -> &crate::operation::create_package::builders::CreatePackageInputBuilder {
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
        crate::operation::create_package::CreatePackageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_package::CreatePackageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_package::CreatePackage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_package::CreatePackage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_package::CreatePackageOutput,
        crate::operation::create_package::CreatePackageError,
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
    /// <p>Unique identifier for the package.</p>
    pub fn package_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_name(input.into());
        self
    }
    /// <p>Unique identifier for the package.</p>
    pub fn set_package_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_name(input);
        self
    }
    /// <p>Unique identifier for the package.</p>
    pub fn get_package_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_name()
    }
    /// <p>Type of package. Currently supports only TXT-DICTIONARY.</p>
    pub fn package_type(mut self, input: crate::types::PackageType) -> Self {
        self.inner = self.inner.package_type(input);
        self
    }
    /// <p>Type of package. Currently supports only TXT-DICTIONARY.</p>
    pub fn set_package_type(mut self, input: ::std::option::Option<crate::types::PackageType>) -> Self {
        self.inner = self.inner.set_package_type(input);
        self
    }
    /// <p>Type of package. Currently supports only TXT-DICTIONARY.</p>
    pub fn get_package_type(&self) -> &::std::option::Option<crate::types::PackageType> {
        self.inner.get_package_type()
    }
    /// <p>Description of the package.</p>
    pub fn package_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.package_description(input.into());
        self
    }
    /// <p>Description of the package.</p>
    pub fn set_package_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_package_description(input);
        self
    }
    /// <p>Description of the package.</p>
    pub fn get_package_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_package_description()
    }
    /// <p>The customer S3 location <code>PackageSource</code> for importing the package.</p>
    pub fn package_source(mut self, input: crate::types::PackageSource) -> Self {
        self.inner = self.inner.package_source(input);
        self
    }
    /// <p>The customer S3 location <code>PackageSource</code> for importing the package.</p>
    pub fn set_package_source(mut self, input: ::std::option::Option<crate::types::PackageSource>) -> Self {
        self.inner = self.inner.set_package_source(input);
        self
    }
    /// <p>The customer S3 location <code>PackageSource</code> for importing the package.</p>
    pub fn get_package_source(&self) -> &::std::option::Option<crate::types::PackageSource> {
        self.inner.get_package_source()
    }
}
