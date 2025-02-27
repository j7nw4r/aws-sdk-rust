// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateMember`](crate::operation::create_member::builders::CreateMemberFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account(AccountDetail)`](crate::operation::create_member::builders::CreateMemberFluentBuilder::account) / [`set_account(Option<AccountDetail>)`](crate::operation::create_member::builders::CreateMemberFluentBuilder::set_account):<br>required: **true**<br><p>The details of the account to associate with the administrator account.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_member::builders::CreateMemberFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_member::builders::CreateMemberFluentBuilder::set_tags):<br>required: **false**<br><p>A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.</p>  <p>An account can have a maximum of 50 tags. Each tag consists of a tag key and an associated tag value. The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p><br>
    /// - On success, responds with [`CreateMemberOutput`](crate::operation::create_member::CreateMemberOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_member::CreateMemberOutput::arn): <p>The Amazon Resource Name (ARN) of the account that was associated with the administrator account.</p>
    /// - On failure, responds with [`SdkError<CreateMemberError>`](crate::operation::create_member::CreateMemberError)
    pub fn create_member(&self) -> crate::operation::create_member::builders::CreateMemberFluentBuilder {
        crate::operation::create_member::builders::CreateMemberFluentBuilder::new(self.handle.clone())
    }
}
