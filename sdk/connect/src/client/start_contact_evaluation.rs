// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartContactEvaluation`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::set_instance_id):<br>required: **true**<br><p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p><br>
    ///   - [`contact_id(impl Into<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::contact_id) / [`set_contact_id(Option<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::set_contact_id):<br>required: **true**<br><p>The identifier of the contact in this instance of Amazon Connect. </p><br>
    ///   - [`evaluation_form_id(impl Into<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::evaluation_form_id) / [`set_evaluation_form_id(Option<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::set_evaluation_form_id):<br>required: **true**<br><p>The unique identifier for the evaluation form.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p><br>
    /// - On success, responds with [`StartContactEvaluationOutput`](crate::operation::start_contact_evaluation::StartContactEvaluationOutput) with field(s):
    ///   - [`evaluation_id(String)`](crate::operation::start_contact_evaluation::StartContactEvaluationOutput::evaluation_id): <p>A unique identifier for the contact evaluation.</p>
    ///   - [`evaluation_arn(String)`](crate::operation::start_contact_evaluation::StartContactEvaluationOutput::evaluation_arn): <p>The Amazon Resource Name (ARN) for the contact evaluation resource.</p>
    /// - On failure, responds with [`SdkError<StartContactEvaluationError>`](crate::operation::start_contact_evaluation::StartContactEvaluationError)
    pub fn start_contact_evaluation(&self) -> crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder {
        crate::operation::start_contact_evaluation::builders::StartContactEvaluationFluentBuilder::new(self.handle.clone())
    }
}
