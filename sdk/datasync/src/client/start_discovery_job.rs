// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartDiscoveryJob`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`storage_system_arn(impl Into<String>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::storage_system_arn) / [`set_storage_system_arn(Option<String>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::set_storage_system_arn):<br>required: **true**<br><p>Specifies the Amazon Resource Name (ARN) of the on-premises storage system that you want to run the discovery job on.</p><br>
    ///   - [`collection_duration_minutes(i32)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::collection_duration_minutes) / [`set_collection_duration_minutes(Option<i32>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::set_collection_duration_minutes):<br>required: **true**<br><p>Specifies in minutes how long you want the discovery job to run.</p> <note>   <p>For more accurate recommendations, we recommend a duration of at least 14 days. Longer durations allow time to collect a sufficient number of data points and provide a realistic representation of storage performance and utilization.</p>  </note><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::set_client_token):<br>required: **true**<br><p>Specifies a client token to make sure requests with this API operation are idempotent. If you don't specify a client token, DataSync generates one for you automatically.</p><br>
    ///   - [`tags(TagListEntry)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::tags) / [`set_tags(Option<Vec::<TagListEntry>>)`](crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::set_tags):<br>required: **false**<br><p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources.</p><br>
    /// - On success, responds with [`StartDiscoveryJobOutput`](crate::operation::start_discovery_job::StartDiscoveryJobOutput) with field(s):
    ///   - [`discovery_job_arn(Option<String>)`](crate::operation::start_discovery_job::StartDiscoveryJobOutput::discovery_job_arn): <p>The ARN of the discovery job that you started.</p>
    /// - On failure, responds with [`SdkError<StartDiscoveryJobError>`](crate::operation::start_discovery_job::StartDiscoveryJobError)
    pub fn start_discovery_job(&self) -> crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder {
        crate::operation::start_discovery_job::builders::StartDiscoveryJobFluentBuilder::new(self.handle.clone())
    }
}
