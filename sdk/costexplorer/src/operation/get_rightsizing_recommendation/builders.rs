// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_rightsizing_recommendation::_get_rightsizing_recommendation_output::GetRightsizingRecommendationOutputBuilder;

pub use crate::operation::get_rightsizing_recommendation::_get_rightsizing_recommendation_input::GetRightsizingRecommendationInputBuilder;

impl GetRightsizingRecommendationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_rightsizing_recommendation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRightsizingRecommendation`.
///
/// <p>Creates recommendations that help you save cost by identifying idle and underutilized Amazon EC2 instances.</p>
/// <p>Recommendations are generated to either downsize or terminate instances, along with providing savings detail and metrics. For more information about calculation and function, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/ce-rightsizing.html">Optimizing Your Cost with Rightsizing Recommendations</a> in the <i>Billing and Cost Management User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRightsizingRecommendationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_rightsizing_recommendation::builders::GetRightsizingRecommendationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationOutput,
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationError,
    > for GetRightsizingRecommendationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationOutput,
            crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRightsizingRecommendationFluentBuilder {
    /// Creates a new `GetRightsizingRecommendation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRightsizingRecommendation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_rightsizing_recommendation::builders::GetRightsizingRecommendationInputBuilder {
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
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationOutput,
        crate::operation::get_rightsizing_recommendation::GetRightsizingRecommendationError,
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
    /// <p>Use <code>Expression</code> to filter in various Cost Explorer APIs.</p>
    /// <p>Not all <code>Expression</code> types are supported in each API. Refer to the documentation for each specific API to see what is supported.</p>
    /// <p>There are two patterns:</p>
    /// <ul>
    /// <li> <p>Simple dimension values.</p>
    /// <ul>
    /// <li> <p>There are three types of simple dimension values: <code>CostCategories</code>, <code>Tags</code>, and <code>Dimensions</code>.</p>
    /// <ul>
    /// <li> <p>Specify the <code>CostCategories</code> field to define a filter that acts on Cost Categories.</p> </li>
    /// <li> <p>Specify the <code>Tags</code> field to define a filter that acts on Cost Allocation Tags.</p> </li>
    /// <li> <p>Specify the <code>Dimensions</code> field to define a filter that acts on the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_DimensionValues.html"> <code>DimensionValues</code> </a>.</p> </li>
    /// </ul> </li>
    /// <li> <p>For each filter type, you can set the dimension name and values for the filters that you plan to use.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for <code>REGION==us-east-1 OR REGION==us-west-1</code>. For <code>GetRightsizingRecommendation</code>, the Region is a full name (for example, <code>REGION==US East (N. Virginia)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] } }</code> </p> </li>
    /// <li> <p>As shown in the previous example, lists of dimension values are combined with <code>OR</code> when applying the filter.</p> </li>
    /// </ul> </li>
    /// <li> <p>You can also set different match options to further control how the filter behaves. Not all APIs support match options. Refer to the documentation for each specific API to see what is supported.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for linked account names that start with "a".</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "LINKED_ACCOUNT_NAME", "MatchOptions": [ "STARTS_WITH" ], "Values": [ "a" ] } }</code> </p> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// <li> <p>Compound <code>Expression</code> types with logical operations.</p>
    /// <ul>
    /// <li> <p>You can use multiple <code>Expression</code> types and the logical operators <code>AND/OR/NOT</code> to create a list of one or more <code>Expression</code> objects. By doing this, you can filter by more advanced options.</p> </li>
    /// <li> <p>For example, you can filter by <code>((REGION == us-east-1 OR REGION == us-west-1) OR (TAG.Type == Type1)) AND (USAGE_TYPE != DataTransfer)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "And": [ {"Or": [ {"Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] }}, {"Tags": { "Key": "TagName", "Values": ["Value1"] } } ]}, {"Not": {"Dimensions": { "Key": "USAGE_TYPE", "Values": ["DataTransfer"] }}} ] } </code> </p> </li>
    /// </ul> <note>
    /// <p>Because each <code>Expression</code> can have only one operator, the service returns an error if more than one is specified. The following example shows an <code>Expression</code> object that creates an error: <code> { "And": [ ... ], "Dimensions": { "Key": "USAGE_TYPE", "Values": [ "DataTransfer" ] } } </code> </p>
    /// <p>The following is an example of the corresponding error message: <code>"Expression has more than one roots. Only one root operator is allowed for each expression: And, Or, Not, Dimensions, Tags, CostCategories"</code> </p>
    /// </note> </li>
    /// </ul> <note>
    /// <p>For the <code>GetRightsizingRecommendation</code> action, a combination of OR and NOT isn't supported. OR isn't supported between different dimensions, or dimensions and tags. NOT operators aren't supported. Dimensions are also limited to <code>LINKED_ACCOUNT</code>, <code>REGION</code>, or <code>RIGHTSIZING_TYPE</code>.</p>
    /// <p>For the <code>GetReservationPurchaseRecommendation</code> action, only NOT is supported. AND and OR aren't supported. Dimensions are limited to <code>LINKED_ACCOUNT</code>.</p>
    /// </note>
    pub fn filter(mut self, input: crate::types::Expression) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>Use <code>Expression</code> to filter in various Cost Explorer APIs.</p>
    /// <p>Not all <code>Expression</code> types are supported in each API. Refer to the documentation for each specific API to see what is supported.</p>
    /// <p>There are two patterns:</p>
    /// <ul>
    /// <li> <p>Simple dimension values.</p>
    /// <ul>
    /// <li> <p>There are three types of simple dimension values: <code>CostCategories</code>, <code>Tags</code>, and <code>Dimensions</code>.</p>
    /// <ul>
    /// <li> <p>Specify the <code>CostCategories</code> field to define a filter that acts on Cost Categories.</p> </li>
    /// <li> <p>Specify the <code>Tags</code> field to define a filter that acts on Cost Allocation Tags.</p> </li>
    /// <li> <p>Specify the <code>Dimensions</code> field to define a filter that acts on the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_DimensionValues.html"> <code>DimensionValues</code> </a>.</p> </li>
    /// </ul> </li>
    /// <li> <p>For each filter type, you can set the dimension name and values for the filters that you plan to use.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for <code>REGION==us-east-1 OR REGION==us-west-1</code>. For <code>GetRightsizingRecommendation</code>, the Region is a full name (for example, <code>REGION==US East (N. Virginia)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] } }</code> </p> </li>
    /// <li> <p>As shown in the previous example, lists of dimension values are combined with <code>OR</code> when applying the filter.</p> </li>
    /// </ul> </li>
    /// <li> <p>You can also set different match options to further control how the filter behaves. Not all APIs support match options. Refer to the documentation for each specific API to see what is supported.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for linked account names that start with "a".</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "LINKED_ACCOUNT_NAME", "MatchOptions": [ "STARTS_WITH" ], "Values": [ "a" ] } }</code> </p> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// <li> <p>Compound <code>Expression</code> types with logical operations.</p>
    /// <ul>
    /// <li> <p>You can use multiple <code>Expression</code> types and the logical operators <code>AND/OR/NOT</code> to create a list of one or more <code>Expression</code> objects. By doing this, you can filter by more advanced options.</p> </li>
    /// <li> <p>For example, you can filter by <code>((REGION == us-east-1 OR REGION == us-west-1) OR (TAG.Type == Type1)) AND (USAGE_TYPE != DataTransfer)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "And": [ {"Or": [ {"Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] }}, {"Tags": { "Key": "TagName", "Values": ["Value1"] } } ]}, {"Not": {"Dimensions": { "Key": "USAGE_TYPE", "Values": ["DataTransfer"] }}} ] } </code> </p> </li>
    /// </ul> <note>
    /// <p>Because each <code>Expression</code> can have only one operator, the service returns an error if more than one is specified. The following example shows an <code>Expression</code> object that creates an error: <code> { "And": [ ... ], "Dimensions": { "Key": "USAGE_TYPE", "Values": [ "DataTransfer" ] } } </code> </p>
    /// <p>The following is an example of the corresponding error message: <code>"Expression has more than one roots. Only one root operator is allowed for each expression: And, Or, Not, Dimensions, Tags, CostCategories"</code> </p>
    /// </note> </li>
    /// </ul> <note>
    /// <p>For the <code>GetRightsizingRecommendation</code> action, a combination of OR and NOT isn't supported. OR isn't supported between different dimensions, or dimensions and tags. NOT operators aren't supported. Dimensions are also limited to <code>LINKED_ACCOUNT</code>, <code>REGION</code>, or <code>RIGHTSIZING_TYPE</code>.</p>
    /// <p>For the <code>GetReservationPurchaseRecommendation</code> action, only NOT is supported. AND and OR aren't supported. Dimensions are limited to <code>LINKED_ACCOUNT</code>.</p>
    /// </note>
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::Expression>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Use <code>Expression</code> to filter in various Cost Explorer APIs.</p>
    /// <p>Not all <code>Expression</code> types are supported in each API. Refer to the documentation for each specific API to see what is supported.</p>
    /// <p>There are two patterns:</p>
    /// <ul>
    /// <li> <p>Simple dimension values.</p>
    /// <ul>
    /// <li> <p>There are three types of simple dimension values: <code>CostCategories</code>, <code>Tags</code>, and <code>Dimensions</code>.</p>
    /// <ul>
    /// <li> <p>Specify the <code>CostCategories</code> field to define a filter that acts on Cost Categories.</p> </li>
    /// <li> <p>Specify the <code>Tags</code> field to define a filter that acts on Cost Allocation Tags.</p> </li>
    /// <li> <p>Specify the <code>Dimensions</code> field to define a filter that acts on the <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_DimensionValues.html"> <code>DimensionValues</code> </a>.</p> </li>
    /// </ul> </li>
    /// <li> <p>For each filter type, you can set the dimension name and values for the filters that you plan to use.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for <code>REGION==us-east-1 OR REGION==us-west-1</code>. For <code>GetRightsizingRecommendation</code>, the Region is a full name (for example, <code>REGION==US East (N. Virginia)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] } }</code> </p> </li>
    /// <li> <p>As shown in the previous example, lists of dimension values are combined with <code>OR</code> when applying the filter.</p> </li>
    /// </ul> </li>
    /// <li> <p>You can also set different match options to further control how the filter behaves. Not all APIs support match options. Refer to the documentation for each specific API to see what is supported.</p>
    /// <ul>
    /// <li> <p>For example, you can filter for linked account names that start with "a".</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "Dimensions": { "Key": "LINKED_ACCOUNT_NAME", "MatchOptions": [ "STARTS_WITH" ], "Values": [ "a" ] } }</code> </p> </li>
    /// </ul> </li>
    /// </ul> </li>
    /// <li> <p>Compound <code>Expression</code> types with logical operations.</p>
    /// <ul>
    /// <li> <p>You can use multiple <code>Expression</code> types and the logical operators <code>AND/OR/NOT</code> to create a list of one or more <code>Expression</code> objects. By doing this, you can filter by more advanced options.</p> </li>
    /// <li> <p>For example, you can filter by <code>((REGION == us-east-1 OR REGION == us-west-1) OR (TAG.Type == Type1)) AND (USAGE_TYPE != DataTransfer)</code>.</p> </li>
    /// <li> <p>The corresponding <code>Expression</code> for this example is as follows: <code>{ "And": [ {"Or": [ {"Dimensions": { "Key": "REGION", "Values": [ "us-east-1", "us-west-1" ] }}, {"Tags": { "Key": "TagName", "Values": ["Value1"] } } ]}, {"Not": {"Dimensions": { "Key": "USAGE_TYPE", "Values": ["DataTransfer"] }}} ] } </code> </p> </li>
    /// </ul> <note>
    /// <p>Because each <code>Expression</code> can have only one operator, the service returns an error if more than one is specified. The following example shows an <code>Expression</code> object that creates an error: <code> { "And": [ ... ], "Dimensions": { "Key": "USAGE_TYPE", "Values": [ "DataTransfer" ] } } </code> </p>
    /// <p>The following is an example of the corresponding error message: <code>"Expression has more than one roots. Only one root operator is allowed for each expression: And, Or, Not, Dimensions, Tags, CostCategories"</code> </p>
    /// </note> </li>
    /// </ul> <note>
    /// <p>For the <code>GetRightsizingRecommendation</code> action, a combination of OR and NOT isn't supported. OR isn't supported between different dimensions, or dimensions and tags. NOT operators aren't supported. Dimensions are also limited to <code>LINKED_ACCOUNT</code>, <code>REGION</code>, or <code>RIGHTSIZING_TYPE</code>.</p>
    /// <p>For the <code>GetReservationPurchaseRecommendation</code> action, only NOT is supported. AND and OR aren't supported. Dimensions are limited to <code>LINKED_ACCOUNT</code>.</p>
    /// </note>
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::Expression> {
        self.inner.get_filter()
    }
    /// <p>You can use Configuration to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings that are associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
    pub fn configuration(mut self, input: crate::types::RightsizingRecommendationConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>You can use Configuration to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings that are associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::RightsizingRecommendationConfiguration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>You can use Configuration to customize recommendations across two attributes. You can choose to view recommendations for instances within the same instance families or across different instance families. You can also choose to view your estimated savings that are associated with recommendations with consideration of existing Savings Plans or RI benefits, or neither. </p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::RightsizingRecommendationConfiguration> {
        self.inner.get_configuration()
    }
    /// <p>The specific service that you want recommendations for. The only valid value for <code>GetRightsizingRecommendation</code> is "<code>AmazonEC2</code>".</p>
    pub fn service(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service(input.into());
        self
    }
    /// <p>The specific service that you want recommendations for. The only valid value for <code>GetRightsizingRecommendation</code> is "<code>AmazonEC2</code>".</p>
    pub fn set_service(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service(input);
        self
    }
    /// <p>The specific service that you want recommendations for. The only valid value for <code>GetRightsizingRecommendation</code> is "<code>AmazonEC2</code>".</p>
    pub fn get_service(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service()
    }
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The number of recommendations that you want returned in a single response object.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub fn next_page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_page_token(input.into());
        self
    }
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub fn set_next_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_page_token(input);
        self
    }
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    pub fn get_next_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_page_token()
    }
}
