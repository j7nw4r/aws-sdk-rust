// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_dashboard_published_version::_update_dashboard_published_version_output::UpdateDashboardPublishedVersionOutputBuilder;

pub use crate::operation::update_dashboard_published_version::_update_dashboard_published_version_input::UpdateDashboardPublishedVersionInputBuilder;

impl UpdateDashboardPublishedVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_dashboard_published_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDashboardPublishedVersion`.
///
/// <p>Updates the published version of a dashboard.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDashboardPublishedVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_dashboard_published_version::builders::UpdateDashboardPublishedVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionOutput,
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionError,
    > for UpdateDashboardPublishedVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionOutput,
            crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDashboardPublishedVersionFluentBuilder {
    /// Creates a new `UpdateDashboardPublishedVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDashboardPublishedVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::update_dashboard_published_version::builders::UpdateDashboardPublishedVersionInputBuilder {
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
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionOutput,
        crate::operation::update_dashboard_published_version::UpdateDashboardPublishedVersionError,
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
    /// <p>The ID of the Amazon Web Services account that contains the dashboard that you're updating.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the dashboard that you're updating.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the dashboard that you're updating.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the dashboard.</p>
    pub fn dashboard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dashboard_id(input.into());
        self
    }
    /// <p>The ID for the dashboard.</p>
    pub fn set_dashboard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dashboard_id(input);
        self
    }
    /// <p>The ID for the dashboard.</p>
    pub fn get_dashboard_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dashboard_id()
    }
    /// <p>The version number of the dashboard.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.inner = self.inner.version_number(input);
        self
    }
    /// <p>The version number of the dashboard.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version_number(input);
        self
    }
    /// <p>The version number of the dashboard.</p>
    pub fn get_version_number(&self) -> &::std::option::Option<i64> {
        self.inner.get_version_number()
    }
}
