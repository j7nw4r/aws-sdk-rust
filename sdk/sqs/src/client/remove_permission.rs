// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemovePermission`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`queue_url(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_queue_url):<br>required: **true**<br><p>The URL of the Amazon SQS queue from which permissions are removed.</p>  <p>Queue URLs and names are case-sensitive.</p><br>
    ///   - [`label(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::label) / [`set_label(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_label):<br>required: **true**<br><p>The identification of the permission to remove. This is the label added using the <code> <code>AddPermission</code> </code> action.</p><br>
    /// - On success, responds with [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput)
    /// - On failure, responds with [`SdkError<RemovePermissionError>`](crate::operation::remove_permission::RemovePermissionError)
    pub fn remove_permission(&self) -> crate::operation::remove_permission::builders::RemovePermissionFluentBuilder {
        crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::new(self.handle.clone())
    }
}
