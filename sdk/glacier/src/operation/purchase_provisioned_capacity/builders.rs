// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::purchase_provisioned_capacity::_purchase_provisioned_capacity_output::PurchaseProvisionedCapacityOutputBuilder;

pub use crate::operation::purchase_provisioned_capacity::_purchase_provisioned_capacity_input::PurchaseProvisionedCapacityInputBuilder;

impl PurchaseProvisionedCapacityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.purchase_provisioned_capacity();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PurchaseProvisionedCapacity`.
///
/// <p>This operation purchases a provisioned capacity unit for an AWS account. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PurchaseProvisionedCapacityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput,
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError,
    > for PurchaseProvisionedCapacityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput,
            crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PurchaseProvisionedCapacityFluentBuilder {
    /// Creates a new `PurchaseProvisionedCapacity`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PurchaseProvisionedCapacity as a reference.
    pub fn as_input(&self) -> &crate::operation::purchase_provisioned_capacity::builders::PurchaseProvisionedCapacityInputBuilder {
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
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacity::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacity::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityOutput,
        crate::operation::purchase_provisioned_capacity::PurchaseProvisionedCapacityError,
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
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '-' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, don't include any hyphens ('-') in the ID. </p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
