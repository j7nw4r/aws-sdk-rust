// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::export_source_network_cfn_template::_export_source_network_cfn_template_output::ExportSourceNetworkCfnTemplateOutputBuilder;

pub use crate::operation::export_source_network_cfn_template::_export_source_network_cfn_template_input::ExportSourceNetworkCfnTemplateInputBuilder;

impl ExportSourceNetworkCfnTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.export_source_network_cfn_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExportSourceNetworkCfnTemplate`.
///
/// <p>Export the Source Network CloudFormation template to an S3 bucket.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExportSourceNetworkCfnTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput,
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError,
    > for ExportSourceNetworkCfnTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput,
            crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ExportSourceNetworkCfnTemplateFluentBuilder {
    /// Creates a new `ExportSourceNetworkCfnTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ExportSourceNetworkCfnTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateInputBuilder {
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
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput,
        crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError,
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
    /// <p>The Source Network ID to export its CloudFormation template to an S3 bucket.</p>
    pub fn source_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_network_id(input.into());
        self
    }
    /// <p>The Source Network ID to export its CloudFormation template to an S3 bucket.</p>
    pub fn set_source_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_network_id(input);
        self
    }
    /// <p>The Source Network ID to export its CloudFormation template to an S3 bucket.</p>
    pub fn get_source_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_network_id()
    }
}
