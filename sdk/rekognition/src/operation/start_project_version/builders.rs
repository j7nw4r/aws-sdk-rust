// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_project_version::_start_project_version_output::StartProjectVersionOutputBuilder;

pub use crate::operation::start_project_version::_start_project_version_input::StartProjectVersionInputBuilder;

impl StartProjectVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_project_version::StartProjectVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_project_version::StartProjectVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_project_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartProjectVersion`.
///
/// <note>
/// <p>This operation applies only to Amazon Rekognition Custom Labels.</p>
/// </note>
/// <p>Starts the running of the version of a model. Starting a model takes a while to complete. To check the current state of the model, use <code>DescribeProjectVersions</code>. </p>
/// <p>Once the model is running, you can detect custom labels in new images by calling <code>DetectCustomLabels</code>.</p> <note>
/// <p>You are charged for the amount of time that the model is running. To stop a running model, call <code>StopProjectVersion</code>.</p>
/// </note>
/// <p>This operation requires permissions to perform the <code>rekognition:StartProjectVersion</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartProjectVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_project_version::builders::StartProjectVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_project_version::StartProjectVersionOutput,
        crate::operation::start_project_version::StartProjectVersionError,
    > for StartProjectVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_project_version::StartProjectVersionOutput,
            crate::operation::start_project_version::StartProjectVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartProjectVersionFluentBuilder {
    /// Creates a new `StartProjectVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartProjectVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::start_project_version::builders::StartProjectVersionInputBuilder {
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
        crate::operation::start_project_version::StartProjectVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_project_version::StartProjectVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_project_version::StartProjectVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_project_version::StartProjectVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_project_version::StartProjectVersionOutput,
        crate::operation::start_project_version::StartProjectVersionError,
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
    /// <p>The Amazon Resource Name(ARN) of the model version that you want to start.</p>
    pub fn project_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_version_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name(ARN) of the model version that you want to start.</p>
    pub fn set_project_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_version_arn(input);
        self
    }
    /// <p>The Amazon Resource Name(ARN) of the model version that you want to start.</p>
    pub fn get_project_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_version_arn()
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. </p>
    /// <p>Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn min_inference_units(mut self, input: i32) -> Self {
        self.inner = self.inner.min_inference_units(input);
        self
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. </p>
    /// <p>Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn set_min_inference_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_min_inference_units(input);
        self
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. </p>
    /// <p>Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn get_min_inference_units(&self) -> &::std::option::Option<i32> {
        self.inner.get_min_inference_units()
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Rekognition Custom Labels doesn't auto-scale the model.</p>
    pub fn max_inference_units(mut self, input: i32) -> Self {
        self.inner = self.inner.max_inference_units(input);
        self
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Rekognition Custom Labels doesn't auto-scale the model.</p>
    pub fn set_max_inference_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_inference_units(input);
        self
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Rekognition Custom Labels doesn't auto-scale the model.</p>
    pub fn get_max_inference_units(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_inference_units()
    }
}
