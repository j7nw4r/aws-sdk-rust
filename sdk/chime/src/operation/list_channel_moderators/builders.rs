// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_channel_moderators::_list_channel_moderators_output::ListChannelModeratorsOutputBuilder;

pub use crate::operation::list_channel_moderators::_list_channel_moderators_input::ListChannelModeratorsInputBuilder;

impl ListChannelModeratorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_channel_moderators::ListChannelModeratorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channel_moderators::ListChannelModeratorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_channel_moderators();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListChannelModerators`.
///
/// <p>Lists all the moderators for a channel.</p> <note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the <code>AppInstanceUserArn</code> of the user that makes the API call as the value in the header.</p>
/// </note> <important>
/// <p> <b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_messaging-chime_ListChannelModerators.html">ListChannelModerators</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
#[deprecated(note = "Replaced by ListChannelModerators in the Amazon Chime SDK Messaging Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListChannelModeratorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_channel_moderators::builders::ListChannelModeratorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_channel_moderators::ListChannelModeratorsOutput,
        crate::operation::list_channel_moderators::ListChannelModeratorsError,
    > for ListChannelModeratorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_channel_moderators::ListChannelModeratorsOutput,
            crate::operation::list_channel_moderators::ListChannelModeratorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListChannelModeratorsFluentBuilder {
    /// Creates a new `ListChannelModerators`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListChannelModerators as a reference.
    pub fn as_input(&self) -> &crate::operation::list_channel_moderators::builders::ListChannelModeratorsInputBuilder {
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
        crate::operation::list_channel_moderators::ListChannelModeratorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_channel_moderators::ListChannelModeratorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_channel_moderators::ListChannelModerators::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_channel_moderators::ListChannelModerators::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_channel_moderators::ListChannelModeratorsOutput,
        crate::operation::list_channel_moderators::ListChannelModeratorsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_channel_moderators::paginator::ListChannelModeratorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_channel_moderators::paginator::ListChannelModeratorsPaginator {
        crate::operation::list_channel_moderators::paginator::ListChannelModeratorsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_arn()
    }
    /// <p>The maximum number of moderators that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of moderators that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of moderators that you want returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token passed by previous API calls until all requested moderators are returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token passed by previous API calls until all requested moderators are returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token passed by previous API calls until all requested moderators are returned.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_chime_bearer()
    }
}
