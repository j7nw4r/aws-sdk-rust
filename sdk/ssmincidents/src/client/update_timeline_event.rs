// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTimelineEvent`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_client_token):<br>required: **false**<br><p>A token that ensures that a client calls the operation only once with the specified details.</p><br>
    ///   - [`incident_record_arn(impl Into<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::incident_record_arn) / [`set_incident_record_arn(Option<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_incident_record_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the incident that includes the timeline event.</p><br>
    ///   - [`event_id(impl Into<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::event_id) / [`set_event_id(Option<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_event_id):<br>required: **true**<br><p>The ID of the event to update. You can use <code>ListTimelineEvents</code> to find an event's ID.</p><br>
    ///   - [`event_time(DateTime)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::event_time) / [`set_event_time(Option<DateTime>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_event_time):<br>required: **false**<br><p>The timestamp for when the event occurred.</p><br>
    ///   - [`event_type(impl Into<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::event_type) / [`set_event_type(Option<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_event_type):<br>required: **false**<br><p>The type of event. You can update events of type <code>Custom Event</code> and <code>Note</code>.</p><br>
    ///   - [`event_data(impl Into<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::event_data) / [`set_event_data(Option<String>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_event_data):<br>required: **false**<br><p>A short description of the event.</p><br>
    ///   - [`event_references(EventReference)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::event_references) / [`set_event_references(Option<Vec::<EventReference>>)`](crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::set_event_references):<br>required: **false**<br><p>Updates all existing references in a <code>TimelineEvent</code>. A reference is an Amazon Web Services resource involved or associated with the incident. To specify a reference, enter its Amazon Resource Name (ARN). You can also specify a related item associated with that resource. For example, to specify an Amazon DynamoDB (DynamoDB) table as a resource, use its ARN. You can also specify an Amazon CloudWatch metric associated with the DynamoDB table as a related item.</p> <important>   <p>This update action overrides all existing references. If you want to keep existing references, you must specify them in the call. If you don't, this action removes any existing references and enters only new references.</p>  </important><br>
    /// - On success, responds with [`UpdateTimelineEventOutput`](crate::operation::update_timeline_event::UpdateTimelineEventOutput)
    /// - On failure, responds with [`SdkError<UpdateTimelineEventError>`](crate::operation::update_timeline_event::UpdateTimelineEventError)
    pub fn update_timeline_event(&self) -> crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder {
        crate::operation::update_timeline_event::builders::UpdateTimelineEventFluentBuilder::new(self.handle.clone())
    }
}
