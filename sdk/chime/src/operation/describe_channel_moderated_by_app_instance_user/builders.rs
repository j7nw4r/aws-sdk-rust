// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_channel_moderated_by_app_instance_user::_describe_channel_moderated_by_app_instance_user_output::DescribeChannelModeratedByAppInstanceUserOutputBuilder;

pub use crate::operation::describe_channel_moderated_by_app_instance_user::_describe_channel_moderated_by_app_instance_user_input::DescribeChannelModeratedByAppInstanceUserInputBuilder;

impl DescribeChannelModeratedByAppInstanceUserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_channel_moderated_by_app_instance_user();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeChannelModeratedByAppInstanceUser`.
///
/// <p>Returns the full details of a channel moderated by the specified <code>AppInstanceUser</code>.</p> <note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the <code>AppInstanceUserArn</code> of the user that makes the API call as the value in the header.</p>
/// </note> <important>
/// <p> <b>This API is is no longer supported and will not be updated.</b> We recommend using the latest version, <a href="https://docs.aws.amazon.com/chime-sdk/latest/APIReference/API_messaging-chime_DescribeChannelModeratedByAppInstanceUser.html">DescribeChannelModeratedByAppInstanceUser</a>, in the Amazon Chime SDK.</p>
/// <p>Using the latest version requires migrating to a dedicated namespace. For more information, refer to <a href="https://docs.aws.amazon.com/chime-sdk/latest/dg/migrate-from-chm-namespace.html">Migrating from the Amazon Chime namespace</a> in the <i>Amazon Chime SDK Developer Guide</i>.</p>
/// </important>
#[deprecated(note = "Replaced by DescribeChannelModeratedByAppInstanceUser in the Amazon Chime SDK Messaging Namespace")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeChannelModeratedByAppInstanceUserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_channel_moderated_by_app_instance_user::builders::DescribeChannelModeratedByAppInstanceUserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserOutput,
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserError,
    > for DescribeChannelModeratedByAppInstanceUserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserOutput,
            crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeChannelModeratedByAppInstanceUserFluentBuilder {
    /// Creates a new `DescribeChannelModeratedByAppInstanceUser`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeChannelModeratedByAppInstanceUser as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_channel_moderated_by_app_instance_user::builders::DescribeChannelModeratedByAppInstanceUserInputBuilder {
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
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUser::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUser::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserOutput,
        crate::operation::describe_channel_moderated_by_app_instance_user::DescribeChannelModeratedByAppInstanceUserError,
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
    /// <p>The ARN of the moderated channel.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the moderated channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the moderated channel.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_arn()
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> in the moderated channel.</p>
    pub fn app_instance_user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_user_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> in the moderated channel.</p>
    pub fn set_app_instance_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_user_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> in the moderated channel.</p>
    pub fn get_app_instance_user_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_user_arn()
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
