// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetUserPolicy`](crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder::set_user_name):<br>required: **true**<br><p>The name of the user who the policy is associated with.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
    ///   - [`policy_name(impl Into<String>)`](crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<String>)`](crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder::set_policy_name):<br>required: **true**<br><p>The name of the policy document to get.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
    /// - On success, responds with [`GetUserPolicyOutput`](crate::operation::get_user_policy::GetUserPolicyOutput) with field(s):
    ///   - [`user_name(String)`](crate::operation::get_user_policy::GetUserPolicyOutput::user_name): <p>The user the policy is associated with.</p>
    ///   - [`policy_name(String)`](crate::operation::get_user_policy::GetUserPolicyOutput::policy_name): <p>The name of the policy.</p>
    ///   - [`policy_document(String)`](crate::operation::get_user_policy::GetUserPolicyOutput::policy_document): <p>The policy document.</p>  <p>IAM stores policies in JSON format. However, resources that were created using CloudFormation templates can be formatted in YAML. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// - On failure, responds with [`SdkError<GetUserPolicyError>`](crate::operation::get_user_policy::GetUserPolicyError)
    pub fn get_user_policy(&self) -> crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder {
        crate::operation::get_user_policy::builders::GetUserPolicyFluentBuilder::new(self.handle.clone())
    }
}
