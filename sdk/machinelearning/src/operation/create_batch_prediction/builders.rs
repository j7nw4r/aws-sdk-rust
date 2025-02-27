// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_batch_prediction::_create_batch_prediction_output::CreateBatchPredictionOutputBuilder;

pub use crate::operation::create_batch_prediction::_create_batch_prediction_input::CreateBatchPredictionInputBuilder;

impl CreateBatchPredictionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_batch_prediction::CreateBatchPredictionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_batch_prediction::CreateBatchPredictionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_batch_prediction();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateBatchPrediction`.
///
/// <p>Generates predictions for a group of observations. The observations to process exist in one or more data files referenced by a <code>DataSource</code>. This operation creates a new <code>BatchPrediction</code>, and uses an <code>MLModel</code> and the data files referenced by the <code>DataSource</code> as information sources. </p>
/// <p> <code>CreateBatchPrediction</code> is an asynchronous operation. In response to <code>CreateBatchPrediction</code>, Amazon Machine Learning (Amazon ML) immediately returns and sets the <code>BatchPrediction</code> status to <code>PENDING</code>. After the <code>BatchPrediction</code> completes, Amazon ML sets the status to <code>COMPLETED</code>. </p>
/// <p>You can poll for status updates by using the <code>GetBatchPrediction</code> operation and checking the <code>Status</code> parameter of the result. After the <code>COMPLETED</code> status appears, the results are available in the location specified by the <code>OutputUri</code> parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateBatchPredictionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_batch_prediction::builders::CreateBatchPredictionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_batch_prediction::CreateBatchPredictionOutput,
        crate::operation::create_batch_prediction::CreateBatchPredictionError,
    > for CreateBatchPredictionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_batch_prediction::CreateBatchPredictionOutput,
            crate::operation::create_batch_prediction::CreateBatchPredictionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateBatchPredictionFluentBuilder {
    /// Creates a new `CreateBatchPrediction`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateBatchPrediction as a reference.
    pub fn as_input(&self) -> &crate::operation::create_batch_prediction::builders::CreateBatchPredictionInputBuilder {
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
        crate::operation::create_batch_prediction::CreateBatchPredictionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_batch_prediction::CreateBatchPredictionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_batch_prediction::CreateBatchPrediction::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_batch_prediction::CreateBatchPrediction::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_batch_prediction::CreateBatchPredictionOutput,
        crate::operation::create_batch_prediction::CreateBatchPredictionError,
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
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    pub fn batch_prediction_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.batch_prediction_id(input.into());
        self
    }
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    pub fn set_batch_prediction_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_batch_prediction_id(input);
        self
    }
    /// <p>A user-supplied ID that uniquely identifies the <code>BatchPrediction</code>.</p>
    pub fn get_batch_prediction_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_batch_prediction_id()
    }
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>. <code>BatchPredictionName</code> can only use the UTF-8 character set.</p>
    pub fn batch_prediction_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.batch_prediction_name(input.into());
        self
    }
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>. <code>BatchPredictionName</code> can only use the UTF-8 character set.</p>
    pub fn set_batch_prediction_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_batch_prediction_name(input);
        self
    }
    /// <p>A user-supplied name or description of the <code>BatchPrediction</code>. <code>BatchPredictionName</code> can only use the UTF-8 character set.</p>
    pub fn get_batch_prediction_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_batch_prediction_name()
    }
    /// <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p>
    pub fn ml_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ml_model_id(input.into());
        self
    }
    /// <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p>
    pub fn set_ml_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ml_model_id(input);
        self
    }
    /// <p>The ID of the <code>MLModel</code> that will generate predictions for the group of observations. </p>
    pub fn get_ml_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ml_model_id()
    }
    /// <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
    pub fn batch_prediction_data_source_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.batch_prediction_data_source_id(input.into());
        self
    }
    /// <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
    pub fn set_batch_prediction_data_source_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_batch_prediction_data_source_id(input);
        self
    }
    /// <p>The ID of the <code>DataSource</code> that points to the group of observations to predict.</p>
    pub fn get_batch_prediction_data_source_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_batch_prediction_data_source_id()
    }
    /// <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p>
    /// <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    pub fn output_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.output_uri(input.into());
        self
    }
    /// <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p>
    /// <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    pub fn set_output_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_output_uri(input);
        self
    }
    /// <p>The location of an Amazon Simple Storage Service (Amazon S3) bucket or directory to store the batch prediction results. The following substrings are not allowed in the <code>s3 key</code> portion of the <code>outputURI</code> field: ':', '//', '/./', '/../'.</p>
    /// <p>Amazon ML needs permissions to store and retrieve the logs on your behalf. For information about how to set permissions, see the <a href="https://docs.aws.amazon.com/machine-learning/latest/dg">Amazon Machine Learning Developer Guide</a>.</p>
    pub fn get_output_uri(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_output_uri()
    }
}
