// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TagResource`](crate::operation::tag_resource::builders::TagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) for the Amazon SWF domain.</p><br>
    ///   - [`tags(ResourceTag)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::tags) / [`set_tags(Option<Vec::<ResourceTag>>)`](crate::operation::tag_resource::builders::TagResourceFluentBuilder::set_tags):<br>required: **true**<br><p>The list of tags to add to a domain. </p>  <p>Tags may only contain unicode letters, digits, whitespace, or these symbols: <code>_ . : / = + - @</code>.</p><br>
    /// - On success, responds with [`TagResourceOutput`](crate::operation::tag_resource::TagResourceOutput)
    /// - On failure, responds with [`SdkError<TagResourceError>`](crate::operation::tag_resource::TagResourceError)
    pub fn tag_resource(&self) -> crate::operation::tag_resource::builders::TagResourceFluentBuilder {
        crate::operation::tag_resource::builders::TagResourceFluentBuilder::new(self.handle.clone())
    }
}
