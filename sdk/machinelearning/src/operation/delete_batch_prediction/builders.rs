// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_batch_prediction::_delete_batch_prediction_output::DeleteBatchPredictionOutputBuilder;

pub use crate::operation::delete_batch_prediction::_delete_batch_prediction_input::DeleteBatchPredictionInputBuilder;

impl DeleteBatchPredictionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_batch_prediction::DeleteBatchPredictionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_batch_prediction();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteBatchPrediction`.
///
/// <p>Assigns the DELETED status to a <code>BatchPrediction</code>, rendering it unusable.</p>
/// <p>After using the <code>DeleteBatchPrediction</code> operation, you can use the <code>GetBatchPrediction</code> operation to verify that the status of the <code>BatchPrediction</code> changed to DELETED.</p>
/// <p> <b>Caution:</b> The result of the <code>DeleteBatchPrediction</code> operation is irreversible.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteBatchPredictionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput,
        crate::operation::delete_batch_prediction::DeleteBatchPredictionError,
    > for DeleteBatchPredictionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput,
            crate::operation::delete_batch_prediction::DeleteBatchPredictionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteBatchPredictionFluentBuilder {
    /// Creates a new `DeleteBatchPrediction`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteBatchPrediction as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_batch_prediction::builders::DeleteBatchPredictionInputBuilder {
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
        crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_batch_prediction::DeleteBatchPredictionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_batch_prediction::DeleteBatchPrediction::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_batch_prediction::DeleteBatchPrediction::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_batch_prediction::DeleteBatchPredictionOutput,
        crate::operation::delete_batch_prediction::DeleteBatchPredictionError,
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
}
