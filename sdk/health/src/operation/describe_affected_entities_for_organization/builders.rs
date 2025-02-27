// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_affected_entities_for_organization::_describe_affected_entities_for_organization_output::DescribeAffectedEntitiesForOrganizationOutputBuilder;

pub use crate::operation::describe_affected_entities_for_organization::_describe_affected_entities_for_organization_input::DescribeAffectedEntitiesForOrganizationInputBuilder;

impl DescribeAffectedEntitiesForOrganizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_affected_entities_for_organization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAffectedEntitiesForOrganization`.
///
/// <p>Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the Amazon Web Service.</p>
/// <p>At least one event Amazon Resource Name (ARN) and account ID are required.</p>
/// <p>Before you can call this operation, you must first enable Health to work with Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's management account.</p> <note>
/// <ul>
/// <li> <p>This API operation uses pagination. Specify the <code>nextToken</code> parameter in the next request to return more results.</p> </li>
/// <li> <p>This operation doesn't support resource-level permissions. You can't use this operation to allow or deny access to specific Health events. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>Health User Guide</i>.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAffectedEntitiesForOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_affected_entities_for_organization::builders::DescribeAffectedEntitiesForOrganizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationOutput,
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationError,
    > for DescribeAffectedEntitiesForOrganizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationOutput,
            crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAffectedEntitiesForOrganizationFluentBuilder {
    /// Creates a new `DescribeAffectedEntitiesForOrganization`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAffectedEntitiesForOrganization as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_affected_entities_for_organization::builders::DescribeAffectedEntitiesForOrganizationInputBuilder {
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
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganization::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganization::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationOutput,
        crate::operation::describe_affected_entities_for_organization::DescribeAffectedEntitiesForOrganizationError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_affected_entities_for_organization::paginator::DescribeAffectedEntitiesForOrganizationPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_affected_entities_for_organization::paginator::DescribeAffectedEntitiesForOrganizationPaginator {
        crate::operation::describe_affected_entities_for_organization::paginator::DescribeAffectedEntitiesForOrganizationPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `organizationEntityFilters`.
    ///
    /// To override the contents of this collection use [`set_organization_entity_filters`](Self::set_organization_entity_filters).
    ///
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    #[deprecated(note = "This property is deprecated, use organizationEntityAccountFilters instead.")]
    pub fn organization_entity_filters(mut self, input: crate::types::EventAccountFilter) -> Self {
        self.inner = self.inner.organization_entity_filters(input);
        self
    }
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    #[deprecated(note = "This property is deprecated, use organizationEntityAccountFilters instead.")]
    pub fn set_organization_entity_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EventAccountFilter>>) -> Self {
        self.inner = self.inner.set_organization_entity_filters(input);
        self
    }
    /// <p>A JSON set of elements including the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    #[deprecated(note = "This property is deprecated, use organizationEntityAccountFilters instead.")]
    pub fn get_organization_entity_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EventAccountFilter>> {
        self.inner.get_organization_entity_filters()
    }
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale(input.into());
        self
    }
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale(input);
        self
    }
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    pub fn get_locale(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale()
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return in one batch, between 10 and 100, inclusive.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// Appends an item to `organizationEntityAccountFilters`.
    ///
    /// To override the contents of this collection use [`set_organization_entity_account_filters`](Self::set_organization_entity_account_filters).
    ///
    /// <p>A JSON set of elements including the <code>awsAccountId</code>, <code>eventArn</code> and a set of <code>statusCodes</code>.</p>
    pub fn organization_entity_account_filters(mut self, input: crate::types::EntityAccountFilter) -> Self {
        self.inner = self.inner.organization_entity_account_filters(input);
        self
    }
    /// <p>A JSON set of elements including the <code>awsAccountId</code>, <code>eventArn</code> and a set of <code>statusCodes</code>.</p>
    pub fn set_organization_entity_account_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EntityAccountFilter>>,
    ) -> Self {
        self.inner = self.inner.set_organization_entity_account_filters(input);
        self
    }
    /// <p>A JSON set of elements including the <code>awsAccountId</code>, <code>eventArn</code> and a set of <code>statusCodes</code>.</p>
    pub fn get_organization_entity_account_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EntityAccountFilter>> {
        self.inner.get_organization_entity_account_filters()
    }
}
