// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetReportGroups`](crate::operation::batch_get_report_groups::builders::BatchGetReportGroupsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_group_arns(impl Into<String>)`](crate::operation::batch_get_report_groups::builders::BatchGetReportGroupsFluentBuilder::report_group_arns) / [`set_report_group_arns(Option<Vec::<String>>)`](crate::operation::batch_get_report_groups::builders::BatchGetReportGroupsFluentBuilder::set_report_group_arns):<br>required: **true**<br><p> An array of report group ARNs that identify the report groups to return. </p><br>
    /// - On success, responds with [`BatchGetReportGroupsOutput`](crate::operation::batch_get_report_groups::BatchGetReportGroupsOutput) with field(s):
    ///   - [`report_groups(Option<Vec::<ReportGroup>>)`](crate::operation::batch_get_report_groups::BatchGetReportGroupsOutput::report_groups): <p> The array of report groups returned by <code>BatchGetReportGroups</code>. </p>
    ///   - [`report_groups_not_found(Option<Vec::<String>>)`](crate::operation::batch_get_report_groups::BatchGetReportGroupsOutput::report_groups_not_found): <p> An array of ARNs passed to <code>BatchGetReportGroups</code> that are not associated with a <code>ReportGroup</code>. </p>
    /// - On failure, responds with [`SdkError<BatchGetReportGroupsError>`](crate::operation::batch_get_report_groups::BatchGetReportGroupsError)
    pub fn batch_get_report_groups(&self) -> crate::operation::batch_get_report_groups::builders::BatchGetReportGroupsFluentBuilder {
        crate::operation::batch_get_report_groups::builders::BatchGetReportGroupsFluentBuilder::new(self.handle.clone())
    }
}
