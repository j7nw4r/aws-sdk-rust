// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_component::_import_component_output::ImportComponentOutputBuilder;

pub use crate::operation::import_component::_import_component_input::ImportComponentInputBuilder;

impl ImportComponentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_component::ImportComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_component::ImportComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_component();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportComponent`.
///
/// <p>Imports a component and transforms its data into a component document.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportComponentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_component::builders::ImportComponentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_component::ImportComponentOutput,
        crate::operation::import_component::ImportComponentError,
    > for ImportComponentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_component::ImportComponentOutput,
            crate::operation::import_component::ImportComponentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportComponentFluentBuilder {
    /// Creates a new `ImportComponent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportComponent as a reference.
    pub fn as_input(&self) -> &crate::operation::import_component::builders::ImportComponentInputBuilder {
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
        crate::operation::import_component::ImportComponentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_component::ImportComponentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_component::ImportComponent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_component::ImportComponent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_component::ImportComponentOutput,
        crate::operation::import_component::ImportComponentError,
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
    /// <p>The name of the component.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the component.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the component.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The semantic version of the component. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Filtering:</b> With semantic versioning, you have the flexibility to use wildcards (x) to specify the most recent versions or nodes when selecting the base image or components for your recipe. When you use a wildcard in any node, all nodes to the right of the first wildcard must also be wildcards.</p>
    /// </note>
    pub fn semantic_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.semantic_version(input.into());
        self
    }
    /// <p>The semantic version of the component. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Filtering:</b> With semantic versioning, you have the flexibility to use wildcards (x) to specify the most recent versions or nodes when selecting the base image or components for your recipe. When you use a wildcard in any node, all nodes to the right of the first wildcard must also be wildcards.</p>
    /// </note>
    pub fn set_semantic_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_semantic_version(input);
        self
    }
    /// <p>The semantic version of the component. This version follows the semantic version syntax.</p> <note>
    /// <p>The semantic version has four nodes: <major>
    /// .
    /// <minor>
    /// .
    /// <patch>
    /// /
    /// <build>
    /// . You can assign values for the first three, and can filter on all of them.
    /// </build>
    /// </patch>
    /// </minor>
    /// </major></p>
    /// <p> <b>Filtering:</b> With semantic versioning, you have the flexibility to use wildcards (x) to specify the most recent versions or nodes when selecting the base image or components for your recipe. When you use a wildcard in any node, all nodes to the right of the first wildcard must also be wildcards.</p>
    /// </note>
    pub fn get_semantic_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_semantic_version()
    }
    /// <p>The description of the component. Describes the contents of the component.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the component. Describes the contents of the component.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the component. Describes the contents of the component.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The change description of the component. This description indicates the change that has been made in this version, or what makes this version different from other versions of this component.</p>
    pub fn change_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.change_description(input.into());
        self
    }
    /// <p>The change description of the component. This description indicates the change that has been made in this version, or what makes this version different from other versions of this component.</p>
    pub fn set_change_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_change_description(input);
        self
    }
    /// <p>The change description of the component. This description indicates the change that has been made in this version, or what makes this version different from other versions of this component.</p>
    pub fn get_change_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_change_description()
    }
    /// <p>The type of the component denotes whether the component is used to build the image, or only to test it.</p>
    pub fn r#type(mut self, input: crate::types::ComponentType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of the component denotes whether the component is used to build the image, or only to test it.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ComponentType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of the component denotes whether the component is used to build the image, or only to test it.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ComponentType> {
        self.inner.get_type()
    }
    /// <p>The format of the resource that you want to import as a component.</p>
    pub fn format(mut self, input: crate::types::ComponentFormat) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>The format of the resource that you want to import as a component.</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::ComponentFormat>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>The format of the resource that you want to import as a component.</p>
    pub fn get_format(&self) -> &::std::option::Option<crate::types::ComponentFormat> {
        self.inner.get_format()
    }
    /// <p>The platform of the component.</p>
    pub fn platform(mut self, input: crate::types::Platform) -> Self {
        self.inner = self.inner.platform(input);
        self
    }
    /// <p>The platform of the component.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<crate::types::Platform>) -> Self {
        self.inner = self.inner.set_platform(input);
        self
    }
    /// <p>The platform of the component.</p>
    pub fn get_platform(&self) -> &::std::option::Option<crate::types::Platform> {
        self.inner.get_platform()
    }
    /// <p>The data of the component. Used to specify the data inline. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data(input.into());
        self
    }
    /// <p>The data of the component. Used to specify the data inline. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data(input);
        self
    }
    /// <p>The data of the component. Used to specify the data inline. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn get_data(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data()
    }
    /// <p>The uri of the component. Must be an Amazon S3 URL and the requester must have permission to access the Amazon S3 bucket. If you use Amazon S3, you can specify component content up to your service quota. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.uri(input.into());
        self
    }
    /// <p>The uri of the component. Must be an Amazon S3 URL and the requester must have permission to access the Amazon S3 bucket. If you use Amazon S3, you can specify component content up to your service quota. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn set_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_uri(input);
        self
    }
    /// <p>The uri of the component. Must be an Amazon S3 URL and the requester must have permission to access the Amazon S3 bucket. If you use Amazon S3, you can specify component content up to your service quota. Either <code>data</code> or <code>uri</code> can be used to specify the data within the component.</p>
    pub fn get_uri(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_uri()
    }
    /// <p>The ID of the KMS key that should be used to encrypt this component.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The ID of the KMS key that should be used to encrypt this component.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The ID of the KMS key that should be used to encrypt this component.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags of the component.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags of the component.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags of the component.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a> in the <i>Amazon EC2 API Reference</i>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
