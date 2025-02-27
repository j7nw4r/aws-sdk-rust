// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetManagedResource`](crate::operation::get_managed_resource::builders::GetManagedResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_identifier(impl Into<String>)`](crate::operation::get_managed_resource::builders::GetManagedResourceFluentBuilder::resource_identifier) / [`set_resource_identifier(Option<String>)`](crate::operation::get_managed_resource::builders::GetManagedResourceFluentBuilder::set_resource_identifier):<br>required: **true**<br><p>The identifier for the resource to shift away traffic for. The identifier is the Amazon Resource Name (ARN) for the resource.</p>  <p>At this time, supported resources are Network Load Balancers and Application Load Balancers with cross-zone load balancing turned off.</p><br>
    /// - On success, responds with [`GetManagedResourceOutput`](crate::operation::get_managed_resource::GetManagedResourceOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::arn): <p>The Amazon Resource Name (ARN) for the resource.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::name): <p>The name of the resource.</p>
    ///   - [`applied_weights(HashMap::<String, f32>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::applied_weights): <p>A collection of key-value pairs that indicate whether resources are active in Availability Zones or not. The key name is the Availability Zone where the resource is deployed. The value is 1 or 0.</p>
    ///   - [`zonal_shifts(Vec::<ZonalShiftInResource>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::zonal_shifts): <p>The zonal shifts that are currently active for a resource. </p>
    ///   - [`autoshifts(Option<Vec::<AutoshiftInResource>>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::autoshifts): <p>An array of the autoshifts that are active for the resource.</p>
    ///   - [`practice_run_configuration(Option<PracticeRunConfiguration>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::practice_run_configuration): <p>The practice run configuration for zonal autoshift that's associated with the resource.</p>
    ///   - [`zonal_autoshift_status(Option<ZonalAutoshiftStatus>)`](crate::operation::get_managed_resource::GetManagedResourceOutput::zonal_autoshift_status): <p>The status for zonal autoshift for a resource. When the autoshift status is <code>ENABLED</code>, Amazon Web Services shifts traffic for a resource away from an Availability Zone, on your behalf, when Amazon Web Services determines that there's an issue in the Availability Zone that could potentially affect customers.</p>
    /// - On failure, responds with [`SdkError<GetManagedResourceError>`](crate::operation::get_managed_resource::GetManagedResourceError)
    pub fn get_managed_resource(&self) -> crate::operation::get_managed_resource::builders::GetManagedResourceFluentBuilder {
        crate::operation::get_managed_resource::builders::GetManagedResourceFluentBuilder::new(self.handle.clone())
    }
}
