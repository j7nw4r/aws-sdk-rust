// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFleet`](crate::operation::update_fleet::builders::UpdateFleetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::operation::update_fleet::builders::UpdateFleetFluentBuilder::fleet_id) / [`set_fleet_id(Option<String>)`](crate::operation::update_fleet::builders::UpdateFleetFluentBuilder::set_fleet_id):<br>required: **true**<br><p> The ID of the fleet to update. </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_fleet::builders::UpdateFleetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_fleet::builders::UpdateFleetFluentBuilder::set_description):<br>required: **false**<br><p> An updated description of the fleet. </p><br>
    /// - On success, responds with [`UpdateFleetOutput`](crate::operation::update_fleet::UpdateFleetOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::update_fleet::UpdateFleetOutput::id): <p>The ID of the updated fleet.</p>
    ///   - [`arn(Option<String>)`](crate::operation::update_fleet::UpdateFleetOutput::arn): <p>The Amazon Resource Name (ARN) of the updated fleet.</p>
    /// - On failure, responds with [`SdkError<UpdateFleetError>`](crate::operation::update_fleet::UpdateFleetError)
    pub fn update_fleet(&self) -> crate::operation::update_fleet::builders::UpdateFleetFluentBuilder {
        crate::operation::update_fleet::builders::UpdateFleetFluentBuilder::new(self.handle.clone())
    }
}
