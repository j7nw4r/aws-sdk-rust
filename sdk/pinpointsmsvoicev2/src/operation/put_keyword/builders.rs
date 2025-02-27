// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_keyword::_put_keyword_output::PutKeywordOutputBuilder;

pub use crate::operation::put_keyword::_put_keyword_input::PutKeywordInputBuilder;

impl PutKeywordInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_keyword::PutKeywordOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_keyword::PutKeywordError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_keyword();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutKeyword`.
///
/// <p>Creates or updates a keyword configuration on an origination phone number or pool.</p>
/// <p> A keyword is a word that you can search for on a particular phone number or pool. It is also a specific word or phrase that an end user can send to your number to elicit a response, such as an informational message or a special offer. When your number receives a message that begins with a keyword, Amazon Pinpoint responds with a customizable message.</p>
/// <p>If you specify a keyword that isn't valid, an error is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutKeywordFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_keyword::builders::PutKeywordInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_keyword::PutKeywordOutput,
        crate::operation::put_keyword::PutKeywordError,
    > for PutKeywordFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_keyword::PutKeywordOutput,
            crate::operation::put_keyword::PutKeywordError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutKeywordFluentBuilder {
    /// Creates a new `PutKeyword`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutKeyword as a reference.
    pub fn as_input(&self) -> &crate::operation::put_keyword::builders::PutKeywordInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_keyword::PutKeywordOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_keyword::PutKeywordError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_keyword::PutKeyword::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_keyword::PutKeyword::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_keyword::PutKeywordOutput,
        crate::operation::put_keyword::PutKeywordError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <code>DescribePhoneNumbers</code> get the values for PhoneNumberId and PhoneNumberArn while <code>DescribeSenderIds</code> can be used to get the values for SenderId and SenderIdArn.</p>
    pub fn origination_identity(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.origination_identity(input.into());
        self
    }
    /// <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <code>DescribePhoneNumbers</code> get the values for PhoneNumberId and PhoneNumberArn while <code>DescribeSenderIds</code> can be used to get the values for SenderId and SenderIdArn.</p>
    pub fn set_origination_identity(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_origination_identity(input);
        self
    }
    /// <p>The origination identity to use such as a PhoneNumberId, PhoneNumberArn, SenderId or SenderIdArn. You can use <code>DescribePhoneNumbers</code> get the values for PhoneNumberId and PhoneNumberArn while <code>DescribeSenderIds</code> can be used to get the values for SenderId and SenderIdArn.</p>
    pub fn get_origination_identity(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_origination_identity()
    }
    /// <p>The new keyword to add.</p>
    pub fn keyword(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.keyword(input.into());
        self
    }
    /// <p>The new keyword to add.</p>
    pub fn set_keyword(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_keyword(input);
        self
    }
    /// <p>The new keyword to add.</p>
    pub fn get_keyword(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_keyword()
    }
    /// <p>The message associated with the keyword.</p>
    pub fn keyword_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.keyword_message(input.into());
        self
    }
    /// <p>The message associated with the keyword.</p>
    pub fn set_keyword_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_keyword_message(input);
        self
    }
    /// <p>The message associated with the keyword.</p>
    pub fn get_keyword_message(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_keyword_message()
    }
    /// <p>The action to perform for the new keyword when it is received.</p>
    /// <ul>
    /// <li> <p>AUTOMATIC_RESPONSE: A message is sent to the recipient.</p> </li>
    /// <li> <p>OPT_OUT: Keeps the recipient from receiving future messages.</p> </li>
    /// <li> <p>OPT_IN: The recipient wants to receive future messages.</p> </li>
    /// </ul>
    pub fn keyword_action(mut self, input: crate::types::KeywordAction) -> Self {
        self.inner = self.inner.keyword_action(input);
        self
    }
    /// <p>The action to perform for the new keyword when it is received.</p>
    /// <ul>
    /// <li> <p>AUTOMATIC_RESPONSE: A message is sent to the recipient.</p> </li>
    /// <li> <p>OPT_OUT: Keeps the recipient from receiving future messages.</p> </li>
    /// <li> <p>OPT_IN: The recipient wants to receive future messages.</p> </li>
    /// </ul>
    pub fn set_keyword_action(mut self, input: ::std::option::Option<crate::types::KeywordAction>) -> Self {
        self.inner = self.inner.set_keyword_action(input);
        self
    }
    /// <p>The action to perform for the new keyword when it is received.</p>
    /// <ul>
    /// <li> <p>AUTOMATIC_RESPONSE: A message is sent to the recipient.</p> </li>
    /// <li> <p>OPT_OUT: Keeps the recipient from receiving future messages.</p> </li>
    /// <li> <p>OPT_IN: The recipient wants to receive future messages.</p> </li>
    /// </ul>
    pub fn get_keyword_action(&self) -> &::std::option::Option<crate::types::KeywordAction> {
        self.inner.get_keyword_action()
    }
}
