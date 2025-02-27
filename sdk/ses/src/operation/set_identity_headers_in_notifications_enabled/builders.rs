// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_identity_headers_in_notifications_enabled::_set_identity_headers_in_notifications_enabled_output::SetIdentityHeadersInNotificationsEnabledOutputBuilder;

pub use crate::operation::set_identity_headers_in_notifications_enabled::_set_identity_headers_in_notifications_enabled_input::SetIdentityHeadersInNotificationsEnabledInputBuilder;

impl SetIdentityHeadersInNotificationsEnabledInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_identity_headers_in_notifications_enabled();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetIdentityHeadersInNotificationsEnabled`.
///
/// <p>Given an identity (an email address or a domain), sets whether Amazon SES includes the original email headers in the Amazon Simple Notification Service (Amazon SNS) notifications of a specified type.</p>
/// <p>You can execute this operation no more than once per second.</p>
/// <p>For more information about using notifications with Amazon SES, see the <a href="https://docs.aws.amazon.com/ses/latest/dg/monitor-sending-activity-using-notifications.html">Amazon SES Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetIdentityHeadersInNotificationsEnabledFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_identity_headers_in_notifications_enabled::builders::SetIdentityHeadersInNotificationsEnabledInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledOutput,
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledError,
    > for SetIdentityHeadersInNotificationsEnabledFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledOutput,
            crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetIdentityHeadersInNotificationsEnabledFluentBuilder {
    /// Creates a new `SetIdentityHeadersInNotificationsEnabled`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetIdentityHeadersInNotificationsEnabled as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::set_identity_headers_in_notifications_enabled::builders::SetIdentityHeadersInNotificationsEnabledInputBuilder {
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
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabled::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabled::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledOutput,
        crate::operation::set_identity_headers_in_notifications_enabled::SetIdentityHeadersInNotificationsEnabledError,
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
    /// <p>The identity for which to enable or disable headers in notifications. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    pub fn identity(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity(input.into());
        self
    }
    /// <p>The identity for which to enable or disable headers in notifications. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    pub fn set_identity(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity(input);
        self
    }
    /// <p>The identity for which to enable or disable headers in notifications. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    pub fn get_identity(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity()
    }
    /// <p>The notification type for which to enable or disable headers in notifications. </p>
    pub fn notification_type(mut self, input: crate::types::NotificationType) -> Self {
        self.inner = self.inner.notification_type(input);
        self
    }
    /// <p>The notification type for which to enable or disable headers in notifications. </p>
    pub fn set_notification_type(mut self, input: ::std::option::Option<crate::types::NotificationType>) -> Self {
        self.inner = self.inner.set_notification_type(input);
        self
    }
    /// <p>The notification type for which to enable or disable headers in notifications. </p>
    pub fn get_notification_type(&self) -> &::std::option::Option<crate::types::NotificationType> {
        self.inner.get_notification_type()
    }
    /// <p>Sets whether Amazon SES includes the original email headers in Amazon SNS notifications of the specified notification type. A value of <code>true</code> specifies that Amazon SES includes headers in notifications, and a value of <code>false</code> specifies that Amazon SES does not include headers in notifications.</p>
    /// <p>This value can only be set when <code>NotificationType</code> is already set to use a particular Amazon SNS topic.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Sets whether Amazon SES includes the original email headers in Amazon SNS notifications of the specified notification type. A value of <code>true</code> specifies that Amazon SES includes headers in notifications, and a value of <code>false</code> specifies that Amazon SES does not include headers in notifications.</p>
    /// <p>This value can only be set when <code>NotificationType</code> is already set to use a particular Amazon SNS topic.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// <p>Sets whether Amazon SES includes the original email headers in Amazon SNS notifications of the specified notification type. A value of <code>true</code> specifies that Amazon SES includes headers in notifications, and a value of <code>false</code> specifies that Amazon SES does not include headers in notifications.</p>
    /// <p>This value can only be set when <code>NotificationType</code> is already set to use a particular Amazon SNS topic.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_enabled()
    }
}
