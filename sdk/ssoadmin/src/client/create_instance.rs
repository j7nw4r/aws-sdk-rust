// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateInstance`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::set_name):<br>required: **false**<br><p>The name of the instance of IAM Identity Center.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::set_client_token):<br>required: **false**<br><p>Specifies a unique, case-sensitive ID that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value</a>.</p>  <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>  <p>If you retry the operation with the same <code>ClientToken</code>, but with different parameters, the retry fails with an <code>IdempotentParameterMismatch</code> error.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_instance::builders::CreateInstanceFluentBuilder::set_tags):<br>required: **false**<br><p>Specifies tags to be attached to the instance of IAM Identity Center.</p><br>
    /// - On success, responds with [`CreateInstanceOutput`](crate::operation::create_instance::CreateInstanceOutput) with field(s):
    ///   - [`instance_arn(Option<String>)`](crate::operation::create_instance::CreateInstanceOutput::instance_arn): <p>The ARN of the instance of IAM Identity Center under which the operation will run. </p>  <p>For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    /// - On failure, responds with [`SdkError<CreateInstanceError>`](crate::operation::create_instance::CreateInstanceError)
    pub fn create_instance(&self) -> crate::operation::create_instance::builders::CreateInstanceFluentBuilder {
        crate::operation::create_instance::builders::CreateInstanceFluentBuilder::new(self.handle.clone())
    }
}
