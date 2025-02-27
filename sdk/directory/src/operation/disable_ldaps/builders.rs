// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_ldaps::_disable_ldaps_output::DisableLdapsOutputBuilder;

pub use crate::operation::disable_ldaps::_disable_ldaps_input::DisableLdapsInputBuilder;

impl DisableLdapsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_ldaps::DisableLdapsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_ldaps::DisableLDAPSError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_ldaps();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableLDAPS`.
///
/// <p>Deactivates LDAP secure calls for the specified directory.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableLDAPSFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_ldaps::builders::DisableLdapsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_ldaps::DisableLdapsOutput,
        crate::operation::disable_ldaps::DisableLDAPSError,
    > for DisableLDAPSFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_ldaps::DisableLdapsOutput,
            crate::operation::disable_ldaps::DisableLDAPSError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableLDAPSFluentBuilder {
    /// Creates a new `DisableLDAPS`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableLDAPS as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_ldaps::builders::DisableLdapsInputBuilder {
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
        crate::operation::disable_ldaps::DisableLdapsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_ldaps::DisableLDAPSError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_ldaps::DisableLDAPS::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_ldaps::DisableLDAPS::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_ldaps::DisableLdapsOutput,
        crate::operation::disable_ldaps::DisableLDAPSError,
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
    /// <p>The identifier of the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    pub fn r#type(mut self, input: crate::types::LdapsType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::LdapsType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of LDAP security to enable. Currently only the value <code>Client</code> is supported.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::LdapsType> {
        self.inner.get_type()
    }
}
