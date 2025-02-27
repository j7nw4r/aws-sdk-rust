// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWhatIfForecastExport`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`what_if_forecast_export_name(impl Into<String>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::what_if_forecast_export_name) / [`set_what_if_forecast_export_name(Option<String>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::set_what_if_forecast_export_name):<br>required: **true**<br><p>The name of the what-if forecast to export.</p><br>
    ///   - [`what_if_forecast_arns(impl Into<String>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::what_if_forecast_arns) / [`set_what_if_forecast_arns(Option<Vec::<String>>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::set_what_if_forecast_arns):<br>required: **true**<br><p>The list of what-if forecast Amazon Resource Names (ARNs) to export.</p><br>
    ///   - [`destination(DataDestination)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::destination) / [`set_destination(Option<DataDestination>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::set_destination):<br>required: **true**<br><p>The location where you want to save the forecast and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the location. The forecast must be exported to an Amazon S3 bucket.</p>  <p>If encryption is used, <code>Destination</code> must include an Key Management Service (KMS) key. The IAM role must allow Amazon Forecast permission to access the key.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::set_tags):<br>required: **false**<br><p>A list of <a href="https://docs.aws.amazon.com/forecast/latest/dg/tagging-forecast-resources.html">tags</a> to apply to the what if forecast.</p><br>
    ///   - [`format(impl Into<String>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::format) / [`set_format(Option<String>)`](crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::set_format):<br>required: **false**<br><p>The format of the exported data, CSV or PARQUET.</p><br>
    /// - On success, responds with [`CreateWhatIfForecastExportOutput`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportOutput) with field(s):
    ///   - [`what_if_forecast_export_arn(Option<String>)`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportOutput::what_if_forecast_export_arn): <p>The Amazon Resource Name (ARN) of the what-if forecast.</p>
    /// - On failure, responds with [`SdkError<CreateWhatIfForecastExportError>`](crate::operation::create_what_if_forecast_export::CreateWhatIfForecastExportError)
    pub fn create_what_if_forecast_export(
        &self,
    ) -> crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder {
        crate::operation::create_what_if_forecast_export::builders::CreateWhatIfForecastExportFluentBuilder::new(self.handle.clone())
    }
}
