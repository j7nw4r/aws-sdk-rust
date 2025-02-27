// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_table_data_import_job::_describe_table_data_import_job_output::DescribeTableDataImportJobOutputBuilder;

pub use crate::operation::describe_table_data_import_job::_describe_table_data_import_job_input::DescribeTableDataImportJobInputBuilder;

impl DescribeTableDataImportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_table_data_import_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTableDataImportJob`.
///
/// <p> The DescribeTableDataImportJob API allows you to retrieve the status and details of a table data import job. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTableDataImportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput,
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError,
    > for DescribeTableDataImportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput,
            crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTableDataImportJobFluentBuilder {
    /// Creates a new `DescribeTableDataImportJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTableDataImportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobInputBuilder {
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
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_table_data_import_job::DescribeTableDataImportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobOutput,
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobError,
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
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn workbook_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workbook_id(input.into());
        self
    }
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_workbook_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workbook_id(input);
        self
    }
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_workbook_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workbook_id()
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_id(input.into());
        self
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_id(input);
        self
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_table_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_id()
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
}
