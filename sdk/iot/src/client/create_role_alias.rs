// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRoleAlias`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_alias(impl Into<String>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::role_alias) / [`set_role_alias(Option<String>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::set_role_alias):<br>required: **true**<br><p>The role alias that points to a role ARN. This allows you to change the role without having to update the device.</p><br>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::set_role_arn):<br>required: **true**<br><p>The role ARN.</p><br>
    ///   - [`credential_duration_seconds(i32)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::credential_duration_seconds) / [`set_credential_duration_seconds(Option<i32>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::set_credential_duration_seconds):<br>required: **false**<br><p>How long (in seconds) the credentials will be valid. The default value is 3,600 seconds.</p>  <p>This value must be less than or equal to the maximum session duration of the IAM role that the role alias references.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::set_tags):<br>required: **false**<br><p>Metadata which can be used to manage the role alias.</p> <note>   <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>   <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>   <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>  </note><br>
    /// - On success, responds with [`CreateRoleAliasOutput`](crate::operation::create_role_alias::CreateRoleAliasOutput) with field(s):
    ///   - [`role_alias(Option<String>)`](crate::operation::create_role_alias::CreateRoleAliasOutput::role_alias): <p>The role alias.</p>
    ///   - [`role_alias_arn(Option<String>)`](crate::operation::create_role_alias::CreateRoleAliasOutput::role_alias_arn): <p>The role alias ARN.</p>
    /// - On failure, responds with [`SdkError<CreateRoleAliasError>`](crate::operation::create_role_alias::CreateRoleAliasError)
    pub fn create_role_alias(&self) -> crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder {
        crate::operation::create_role_alias::builders::CreateRoleAliasFluentBuilder::new(self.handle.clone())
    }
}
