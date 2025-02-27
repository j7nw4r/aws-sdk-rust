// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorkflow`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_name):<br>required: **false**<br><p>A name for the workflow.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_description):<br>required: **false**<br><p>A description for the workflow.</p><br>
    ///   - [`engine(WorkflowEngine)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::engine) / [`set_engine(Option<WorkflowEngine>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_engine):<br>required: **false**<br><p>An engine for the workflow.</p><br>
    ///   - [`definition_zip(Blob)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::definition_zip) / [`set_definition_zip(Option<Blob>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_definition_zip):<br>required: **false**<br><p>A ZIP archive for the workflow.</p><br>
    ///   - [`definition_uri(impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::definition_uri) / [`set_definition_uri(Option<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_definition_uri):<br>required: **false**<br><p>The URI of a definition for the workflow.</p><br>
    ///   - [`main(impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::main) / [`set_main(Option<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_main):<br>required: **false**<br><p>The path of the main definition file for the workflow.</p><br>
    ///   - [`parameter_template(impl Into<String>, WorkflowParameter)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::parameter_template) / [`set_parameter_template(Option<HashMap::<String, WorkflowParameter>>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_parameter_template):<br>required: **false**<br><p>A parameter template for the workflow.</p><br>
    ///   - [`storage_capacity(i32)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::storage_capacity) / [`set_storage_capacity(Option<i32>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_storage_capacity):<br>required: **false**<br><p>A storage capacity for the workflow in gigabytes.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_tags):<br>required: **false**<br><p>Tags for the workflow.</p><br>
    ///   - [`request_id(impl Into<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::request_id) / [`set_request_id(Option<String>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_request_id):<br>required: **true**<br><p>To ensure that requests don't run multiple times, specify a unique ID for each request.</p><br>
    ///   - [`accelerators(Accelerators)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::accelerators) / [`set_accelerators(Option<Accelerators>)`](crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::set_accelerators):<br>required: **false**<br><p> The computational accelerator specified to run the workflow. </p><br>
    /// - On success, responds with [`CreateWorkflowOutput`](crate::operation::create_workflow::CreateWorkflowOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_workflow::CreateWorkflowOutput::arn): <p>The workflow's ARN.</p>
    ///   - [`id(Option<String>)`](crate::operation::create_workflow::CreateWorkflowOutput::id): <p>The workflow's ID.</p>
    ///   - [`status(Option<WorkflowStatus>)`](crate::operation::create_workflow::CreateWorkflowOutput::status): <p>The workflow's status.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::create_workflow::CreateWorkflowOutput::tags): <p>The workflow's tags.</p>
    /// - On failure, responds with [`SdkError<CreateWorkflowError>`](crate::operation::create_workflow::CreateWorkflowError)
    pub fn create_workflow(&self) -> crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder {
        crate::operation::create_workflow::builders::CreateWorkflowFluentBuilder::new(self.handle.clone())
    }
}
