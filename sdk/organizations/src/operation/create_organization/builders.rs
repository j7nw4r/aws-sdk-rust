// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_organization::_create_organization_output::CreateOrganizationOutputBuilder;

pub use crate::operation::create_organization::_create_organization_input::CreateOrganizationInputBuilder;

impl CreateOrganizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_organization::CreateOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_organization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateOrganization`.
///
/// <p>Creates an Amazon Web Services organization. The account whose user is calling the <code>CreateOrganization</code> operation automatically becomes the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">management account</a> of the new organization.</p>
/// <p>This operation must be called using credentials from the account that is to become the new organization's management account. The principal must also have the relevant IAM permissions.</p>
/// <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING</code>, no policy types are enabled by default and you can't use organization policies.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_organization::builders::CreateOrganizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_organization::CreateOrganizationOutput,
        crate::operation::create_organization::CreateOrganizationError,
    > for CreateOrganizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_organization::CreateOrganizationOutput,
            crate::operation::create_organization::CreateOrganizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateOrganizationFluentBuilder {
    /// Creates a new `CreateOrganization`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateOrganization as a reference.
    pub fn as_input(&self) -> &crate::operation::create_organization::builders::CreateOrganizationInputBuilder {
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
        crate::operation::create_organization::CreateOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_organization::CreateOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_organization::CreateOrganization::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_organization::CreateOrganization::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_organization::CreateOrganizationOutput,
        crate::operation::create_organization::CreateOrganizationError,
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
    /// <p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p>
    /// <ul>
    /// <li> <p> <code>CONSOLIDATED_BILLING</code>: All member accounts have their bills consolidated to and paid by the management account. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-cb-only">Consolidated billing</a> in the <i>Organizations User Guide</i>.</p> <p> The consolidated billing feature subset isn't available for organizations in the Amazon Web Services GovCloud (US) Region.</p> </li>
    /// <li> <p> <code>ALL</code>: In addition to all the features supported by the consolidated billing feature set, the management account can also apply any policy type to any member account in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-all">All features</a> in the <i>Organizations User Guide</i>.</p> </li>
    /// </ul>
    pub fn feature_set(mut self, input: crate::types::OrganizationFeatureSet) -> Self {
        self.inner = self.inner.feature_set(input);
        self
    }
    /// <p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p>
    /// <ul>
    /// <li> <p> <code>CONSOLIDATED_BILLING</code>: All member accounts have their bills consolidated to and paid by the management account. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-cb-only">Consolidated billing</a> in the <i>Organizations User Guide</i>.</p> <p> The consolidated billing feature subset isn't available for organizations in the Amazon Web Services GovCloud (US) Region.</p> </li>
    /// <li> <p> <code>ALL</code>: In addition to all the features supported by the consolidated billing feature set, the management account can also apply any policy type to any member account in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-all">All features</a> in the <i>Organizations User Guide</i>.</p> </li>
    /// </ul>
    pub fn set_feature_set(mut self, input: ::std::option::Option<crate::types::OrganizationFeatureSet>) -> Self {
        self.inner = self.inner.set_feature_set(input);
        self
    }
    /// <p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p>
    /// <ul>
    /// <li> <p> <code>CONSOLIDATED_BILLING</code>: All member accounts have their bills consolidated to and paid by the management account. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-cb-only">Consolidated billing</a> in the <i>Organizations User Guide</i>.</p> <p> The consolidated billing feature subset isn't available for organizations in the Amazon Web Services GovCloud (US) Region.</p> </li>
    /// <li> <p> <code>ALL</code>: In addition to all the features supported by the consolidated billing feature set, the management account can also apply any policy type to any member account in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#feature-set-all">All features</a> in the <i>Organizations User Guide</i>.</p> </li>
    /// </ul>
    pub fn get_feature_set(&self) -> &::std::option::Option<crate::types::OrganizationFeatureSet> {
        self.inner.get_feature_set()
    }
}
