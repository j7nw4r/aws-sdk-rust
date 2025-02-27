// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_utterances_view::_get_utterances_view_output::GetUtterancesViewOutputBuilder;

pub use crate::operation::get_utterances_view::_get_utterances_view_input::GetUtterancesViewInputBuilder;

impl GetUtterancesViewInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_utterances_view::GetUtterancesViewOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_utterances_view::GetUtterancesViewError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_utterances_view();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetUtterancesView`.
///
/// <p>Use the <code>GetUtterancesView</code> operation to get information about the utterances that your users have made to your bot. You can use this list to tune the utterances that your bot responds to.</p>
/// <p>For example, say that you have created a bot to order flowers. After your users have used your bot for a while, use the <code>GetUtterancesView</code> operation to see the requests that they have made and whether they have been successful. You might find that the utterance "I want flowers" is not being recognized. You could add this utterance to the <code>OrderFlowers</code> intent so that your bot recognizes that utterance.</p>
/// <p>After you publish a new version of a bot, you can get information about the old version and the new so that you can compare the performance across the two versions. </p>
/// <p>Utterance statistics are generated once a day. Data is available for the last 15 days. You can request information for up to 5 versions of your bot in each request. Amazon Lex returns the most frequent utterances received by the bot in the last 15 days. The response contains information about a maximum of 100 utterances for each version.</p>
/// <p>If you set <code>childDirected</code> field to true when you created your bot, if you are using slot obfuscation with one or more slots, or if you opted out of participating in improving Amazon Lex, utterances are not available.</p>
/// <p>This operation requires permissions for the <code>lex:GetUtterancesView</code> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetUtterancesViewFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_utterances_view::builders::GetUtterancesViewInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_utterances_view::GetUtterancesViewOutput,
        crate::operation::get_utterances_view::GetUtterancesViewError,
    > for GetUtterancesViewFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_utterances_view::GetUtterancesViewOutput,
            crate::operation::get_utterances_view::GetUtterancesViewError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetUtterancesViewFluentBuilder {
    /// Creates a new `GetUtterancesView`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetUtterancesView as a reference.
    pub fn as_input(&self) -> &crate::operation::get_utterances_view::builders::GetUtterancesViewInputBuilder {
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
        crate::operation::get_utterances_view::GetUtterancesViewOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_utterances_view::GetUtterancesViewError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_utterances_view::GetUtterancesView::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_utterances_view::GetUtterancesView::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_utterances_view::GetUtterancesViewOutput,
        crate::operation::get_utterances_view::GetUtterancesViewError,
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
    /// <p>The name of the bot for which utterance information should be returned.</p>
    pub fn bot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_name(input.into());
        self
    }
    /// <p>The name of the bot for which utterance information should be returned.</p>
    pub fn set_bot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_name(input);
        self
    }
    /// <p>The name of the bot for which utterance information should be returned.</p>
    pub fn get_bot_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_name()
    }
    /// Appends an item to `botVersions`.
    ///
    /// To override the contents of this collection use [`set_bot_versions`](Self::set_bot_versions).
    ///
    /// <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    pub fn bot_versions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_versions(input.into());
        self
    }
    /// <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    pub fn set_bot_versions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_bot_versions(input);
        self
    }
    /// <p>An array of bot versions for which utterance information should be returned. The limit is 5 versions per request.</p>
    pub fn get_bot_versions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_bot_versions()
    }
    /// <p>To return utterances that were recognized and handled, use <code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    pub fn status_type(mut self, input: crate::types::StatusType) -> Self {
        self.inner = self.inner.status_type(input);
        self
    }
    /// <p>To return utterances that were recognized and handled, use <code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    pub fn set_status_type(mut self, input: ::std::option::Option<crate::types::StatusType>) -> Self {
        self.inner = self.inner.set_status_type(input);
        self
    }
    /// <p>To return utterances that were recognized and handled, use <code>Detected</code>. To return utterances that were not recognized, use <code>Missed</code>.</p>
    pub fn get_status_type(&self) -> &::std::option::Option<crate::types::StatusType> {
        self.inner.get_status_type()
    }
}
