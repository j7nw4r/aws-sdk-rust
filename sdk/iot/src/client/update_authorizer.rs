// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAuthorizer`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`authorizer_name(impl Into<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::authorizer_name) / [`set_authorizer_name(Option<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_authorizer_name):<br>required: **true**<br><p>The authorizer name.</p><br>
    ///   - [`authorizer_function_arn(impl Into<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::authorizer_function_arn) / [`set_authorizer_function_arn(Option<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_authorizer_function_arn):<br>required: **false**<br><p>The ARN of the authorizer's Lambda function.</p><br>
    ///   - [`token_key_name(impl Into<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::token_key_name) / [`set_token_key_name(Option<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_token_key_name):<br>required: **false**<br><p>The key used to extract the token from the HTTP headers. </p><br>
    ///   - [`token_signing_public_keys(impl Into<String>, impl Into<String>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::token_signing_public_keys) / [`set_token_signing_public_keys(Option<HashMap::<String, String>>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_token_signing_public_keys):<br>required: **false**<br><p>The public keys used to verify the token signature.</p><br>
    ///   - [`status(AuthorizerStatus)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::status) / [`set_status(Option<AuthorizerStatus>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_status):<br>required: **false**<br><p>The status of the update authorizer request.</p><br>
    ///   - [`enable_caching_for_http(bool)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::enable_caching_for_http) / [`set_enable_caching_for_http(Option<bool>)`](crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::set_enable_caching_for_http):<br>required: **false**<br><p>When <code>true</code>, the result from the authorizer’s Lambda function is cached for the time specified in <code>refreshAfterInSeconds</code>. The cached result is used while the device reuses the same HTTP connection.</p><br>
    /// - On success, responds with [`UpdateAuthorizerOutput`](crate::operation::update_authorizer::UpdateAuthorizerOutput) with field(s):
    ///   - [`authorizer_name(Option<String>)`](crate::operation::update_authorizer::UpdateAuthorizerOutput::authorizer_name): <p>The authorizer name.</p>
    ///   - [`authorizer_arn(Option<String>)`](crate::operation::update_authorizer::UpdateAuthorizerOutput::authorizer_arn): <p>The authorizer ARN.</p>
    /// - On failure, responds with [`SdkError<UpdateAuthorizerError>`](crate::operation::update_authorizer::UpdateAuthorizerError)
    pub fn update_authorizer(&self) -> crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder {
        crate::operation::update_authorizer::builders::UpdateAuthorizerFluentBuilder::new(self.handle.clone())
    }
}
