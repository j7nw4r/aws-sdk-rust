// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_multi_region_access_point::_delete_multi_region_access_point_output::DeleteMultiRegionAccessPointOutputBuilder;

pub use crate::operation::delete_multi_region_access_point::_delete_multi_region_access_point_input::DeleteMultiRegionAccessPointInputBuilder;

impl DeleteMultiRegionAccessPointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_multi_region_access_point();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteMultiRegionAccessPoint`.
///
/// <p>Deletes a Multi-Region Access Point. This action does not delete the buckets associated with the Multi-Region Access Point, only the Multi-Region Access Point itself.</p>
/// <p>This action will always be routed to the US West (Oregon) Region. For more information about the restrictions around managing Multi-Region Access Points, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ManagingMultiRegionAccessPoints.html">Managing Multi-Region Access Points</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>This request is asynchronous, meaning that you might receive a response before the command has completed. When this request provides a response, it provides a token that you can use to monitor the status of the request with <code>DescribeMultiRegionAccessPointOperation</code>.</p>
/// <p>The following actions are related to <code>DeleteMultiRegionAccessPoint</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_CreateMultiRegionAccessPoint.html">CreateMultiRegionAccessPoint</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DescribeMultiRegionAccessPointOperation.html">DescribeMultiRegionAccessPointOperation</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_GetMultiRegionAccessPoint.html">GetMultiRegionAccessPoint</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_ListMultiRegionAccessPoints.html">ListMultiRegionAccessPoints</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteMultiRegionAccessPointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput,
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError,
    > for DeleteMultiRegionAccessPointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput,
            crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteMultiRegionAccessPointFluentBuilder {
    /// Creates a new `DeleteMultiRegionAccessPoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteMultiRegionAccessPoint as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointInputBuilder {
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
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput,
        crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError,
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
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>A container element containing details about the Multi-Region Access Point.</p>
    pub fn details(mut self, input: crate::types::DeleteMultiRegionAccessPointInput) -> Self {
        self.inner = self.inner.details(input);
        self
    }
    /// <p>A container element containing details about the Multi-Region Access Point.</p>
    pub fn set_details(mut self, input: ::std::option::Option<crate::types::DeleteMultiRegionAccessPointInput>) -> Self {
        self.inner = self.inner.set_details(input);
        self
    }
    /// <p>A container element containing details about the Multi-Region Access Point.</p>
    pub fn get_details(&self) -> &::std::option::Option<crate::types::DeleteMultiRegionAccessPointInput> {
        self.inner.get_details()
    }
}
