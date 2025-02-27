// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateBackendEnvironment`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::set_app_id):<br>required: **true**<br><p>The unique ID for an Amplify app. </p><br>
    ///   - [`environment_name(impl Into<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::environment_name) / [`set_environment_name(Option<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::set_environment_name):<br>required: **true**<br><p>The name for the backend environment. </p><br>
    ///   - [`stack_name(impl Into<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::stack_name) / [`set_stack_name(Option<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::set_stack_name):<br>required: **false**<br><p>The AWS CloudFormation stack name of a backend environment. </p><br>
    ///   - [`deployment_artifacts(impl Into<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::deployment_artifacts) / [`set_deployment_artifacts(Option<String>)`](crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::set_deployment_artifacts):<br>required: **false**<br><p>The name of deployment artifacts. </p><br>
    /// - On success, responds with [`CreateBackendEnvironmentOutput`](crate::operation::create_backend_environment::CreateBackendEnvironmentOutput) with field(s):
    ///   - [`backend_environment(Option<BackendEnvironment>)`](crate::operation::create_backend_environment::CreateBackendEnvironmentOutput::backend_environment): <p>Describes the backend environment for an Amplify app. </p>
    /// - On failure, responds with [`SdkError<CreateBackendEnvironmentError>`](crate::operation::create_backend_environment::CreateBackendEnvironmentError)
    pub fn create_backend_environment(&self) -> crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder {
        crate::operation::create_backend_environment::builders::CreateBackendEnvironmentFluentBuilder::new(self.handle.clone())
    }
}
