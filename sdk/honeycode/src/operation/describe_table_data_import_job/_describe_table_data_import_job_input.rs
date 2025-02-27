// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTableDataImportJobInput {
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub workbook_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeTableDataImportJobInput {
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn workbook_id(&self) -> ::std::option::Option<&str> {
        self.workbook_id.as_deref()
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn table_id(&self) -> ::std::option::Option<&str> {
        self.table_id.as_deref()
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DescribeTableDataImportJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeTableDataImportJobInput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobInput).
    pub fn builder() -> crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobInputBuilder {
        crate::operation::describe_table_data_import_job::builders::DescribeTableDataImportJobInputBuilder::default()
    }
}

/// A builder for [`DescribeTableDataImportJobInput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeTableDataImportJobInputBuilder {
    pub(crate) workbook_id: ::std::option::Option<::std::string::String>,
    pub(crate) table_id: ::std::option::Option<::std::string::String>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeTableDataImportJobInputBuilder {
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    /// This field is required.
    pub fn workbook_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workbook_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_workbook_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workbook_id = input;
        self
    }
    /// <p>The ID of the workbook into which data was imported.</p>
    /// <p> If a workbook with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_workbook_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.workbook_id
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    /// This field is required.
    pub fn table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_id = input;
        self
    }
    /// <p>The ID of the table into which data was imported.</p>
    /// <p> If a table with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_id
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    /// This field is required.
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>The ID of the job that was returned by the StartTableDataImportJob request.</p>
    /// <p> If a job with the specified id could not be found, this API throws ResourceNotFoundException. </p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_id
    }
    /// Consumes the builder and constructs a [`DescribeTableDataImportJobInput`](crate::operation::describe_table_data_import_job::DescribeTableDataImportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_table_data_import_job::DescribeTableDataImportJobInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_table_data_import_job::DescribeTableDataImportJobInput {
            workbook_id: self.workbook_id,
            table_id: self.table_id,
            job_id: self.job_id,
        })
    }
}
