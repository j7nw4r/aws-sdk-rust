// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBranch`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl Into<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::set_app_id):<br>required: **true**<br><p> The unique ID for an Amplify app. </p><br>
    ///   - [`branch_name(impl Into<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::branch_name) / [`set_branch_name(Option<String>)`](crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::set_branch_name):<br>required: **true**<br><p>The name of the branch. </p><br>
    /// - On success, responds with [`DeleteBranchOutput`](crate::operation::delete_branch::DeleteBranchOutput) with field(s):
    ///   - [`branch(Option<Branch>)`](crate::operation::delete_branch::DeleteBranchOutput::branch): <p>The branch for an Amplify app, which maps to a third-party repository branch. </p>
    /// - On failure, responds with [`SdkError<DeleteBranchError>`](crate::operation::delete_branch::DeleteBranchError)
    pub fn delete_branch(&self) -> crate::operation::delete_branch::builders::DeleteBranchFluentBuilder {
        crate::operation::delete_branch::builders::DeleteBranchFluentBuilder::new(self.handle.clone())
    }
}
