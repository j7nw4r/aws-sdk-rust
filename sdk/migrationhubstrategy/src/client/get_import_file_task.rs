// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetImportFileTask`](crate::operation::get_import_file_task::builders::GetImportFileTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::get_import_file_task::builders::GetImportFileTaskFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_import_file_task::builders::GetImportFileTaskFluentBuilder::set_id):<br>required: **true**<br><p> The ID of the import file task. This ID is returned in the response of <code>StartImportFileTask</code>. </p><br>
    /// - On success, responds with [`GetImportFileTaskOutput`](crate::operation::get_import_file_task::GetImportFileTaskOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::id): <p> The import file task <code>id</code> returned in the response of <code>StartImportFileTask</code>. </p>
    ///   - [`status(Option<ImportFileTaskStatus>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::status): <p> Status of import file task. </p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::start_time): <p> Start time of the import task. </p>
    ///   - [`input_s3_bucket(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::input_s3_bucket): <p> The S3 bucket where import file is located. </p>
    ///   - [`input_s3_key(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::input_s3_key): <p> The Amazon S3 key name of the import file. </p>
    ///   - [`status_report_s3_bucket(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::status_report_s3_bucket): <p> The S3 bucket name for status report of import task. </p>
    ///   - [`status_report_s3_key(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::status_report_s3_key): <p> The Amazon S3 key name for status report of import task. The report contains details about whether each record imported successfully or why it did not.</p>
    ///   - [`completion_time(Option<DateTime>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::completion_time): <p> The time that the import task completed. </p>
    ///   - [`number_of_records_success(Option<i32>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::number_of_records_success): <p> The number of records successfully imported. </p>
    ///   - [`number_of_records_failed(Option<i32>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::number_of_records_failed): <p> The number of records that failed to be imported. </p>
    ///   - [`import_name(Option<String>)`](crate::operation::get_import_file_task::GetImportFileTaskOutput::import_name): <p> The name of the import task given in <code>StartImportFileTask</code>. </p>
    /// - On failure, responds with [`SdkError<GetImportFileTaskError>`](crate::operation::get_import_file_task::GetImportFileTaskError)
    pub fn get_import_file_task(&self) -> crate::operation::get_import_file_task::builders::GetImportFileTaskFluentBuilder {
        crate::operation::get_import_file_task::builders::GetImportFileTaskFluentBuilder::new(self.handle.clone())
    }
}
