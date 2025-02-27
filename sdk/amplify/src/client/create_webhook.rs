// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWebhook`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::set_app_id):<br>required: **true**<br><p>The unique ID for an Amplify app. </p><br>
    ///   - [`branch_name(impl Into<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::branch_name) / [`set_branch_name(Option<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::set_branch_name):<br>required: **true**<br><p>The name for a branch that is part of an Amplify app. </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::set_description):<br>required: **false**<br><p>The description for a webhook. </p><br>
    /// - On success, responds with [`CreateWebhookOutput`](crate::operation::create_webhook::CreateWebhookOutput) with field(s):
    ///   - [`webhook(Option<Webhook>)`](crate::operation::create_webhook::CreateWebhookOutput::webhook): <p>Describes a webhook that connects repository events to an Amplify app. </p>
    /// - On failure, responds with [`SdkError<CreateWebhookError>`](crate::operation::create_webhook::CreateWebhookError)
    pub fn create_webhook(&self) -> crate::operation::create_webhook::builders::CreateWebhookFluentBuilder {
        crate::operation::create_webhook::builders::CreateWebhookFluentBuilder::new(self.handle.clone())
    }
}
