// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateFleetAdvisorCollector`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`collector_name(impl Into<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::collector_name) / [`set_collector_name(Option<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::set_collector_name):<br>required: **true**<br><p>The name of your Fleet Advisor collector (for example, <code>sample-collector</code>).</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::set_description):<br>required: **false**<br><p>A summary description of your Fleet Advisor collector.</p><br>
    ///   - [`service_access_role_arn(impl Into<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::service_access_role_arn) / [`set_service_access_role_arn(Option<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::set_service_access_role_arn):<br>required: **true**<br><p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p><br>
    ///   - [`s3_bucket_name(impl Into<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::s3_bucket_name) / [`set_s3_bucket_name(Option<String>)`](crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::set_s3_bucket_name):<br>required: **true**<br><p>The Amazon S3 bucket that the Fleet Advisor collector uses to store inventory metadata.</p><br>
    /// - On success, responds with [`CreateFleetAdvisorCollectorOutput`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput) with field(s):
    ///   - [`collector_referenced_id(Option<String>)`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput::collector_referenced_id): <p>The unique ID of the new Fleet Advisor collector, for example: <code>22fda70c-40d5-4acf-b233-a495bd8eb7f5</code> </p>
    ///   - [`collector_name(Option<String>)`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput::collector_name): <p>The name of the new Fleet Advisor collector.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput::description): <p>A summary description of the Fleet Advisor collector.</p>
    ///   - [`service_access_role_arn(Option<String>)`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput::service_access_role_arn): <p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p>
    ///   - [`s3_bucket_name(Option<String>)`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput::s3_bucket_name): <p>The Amazon S3 bucket that the collector uses to store inventory metadata.</p>
    /// - On failure, responds with [`SdkError<CreateFleetAdvisorCollectorError>`](crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorError)
    pub fn create_fleet_advisor_collector(
        &self,
    ) -> crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder {
        crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorFluentBuilder::new(self.handle.clone())
    }
}
