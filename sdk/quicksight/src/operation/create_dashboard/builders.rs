// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dashboard::_create_dashboard_output::CreateDashboardOutputBuilder;

pub use crate::operation::create_dashboard::_create_dashboard_input::CreateDashboardInputBuilder;

impl CreateDashboardInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_dashboard::CreateDashboardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dashboard::CreateDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_dashboard();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDashboard`.
///
/// <p>Creates a dashboard from either a template or directly with a <code>DashboardDefinition</code>. To first create a template, see the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateTemplate.html">CreateTemplate</a> </code> API operation.</p>
/// <p>A dashboard is an entity in Amazon QuickSight that identifies Amazon QuickSight reports, created from analyses. You can share Amazon QuickSight dashboards. With the right permissions, you can create scheduled email reports from them. If you have the correct permissions, you can create a dashboard from a template that exists in a different Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDashboardFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dashboard::builders::CreateDashboardInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_dashboard::CreateDashboardOutput,
        crate::operation::create_dashboard::CreateDashboardError,
    > for CreateDashboardFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_dashboard::CreateDashboardOutput,
            crate::operation::create_dashboard::CreateDashboardError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDashboardFluentBuilder {
    /// Creates a new `CreateDashboard`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDashboard as a reference.
    pub fn as_input(&self) -> &crate::operation::create_dashboard::builders::CreateDashboardInputBuilder {
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
        crate::operation::create_dashboard::CreateDashboardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_dashboard::CreateDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_dashboard::CreateDashboard::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_dashboard::CreateDashboard::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_dashboard::CreateDashboardOutput,
        crate::operation::create_dashboard::CreateDashboardError,
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
    /// <p>The ID of the Amazon Web Services account where you want to create the dashboard.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account where you want to create the dashboard.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account where you want to create the dashboard.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    pub fn dashboard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dashboard_id(input.into());
        self
    }
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    pub fn set_dashboard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dashboard_id(input);
        self
    }
    /// <p>The ID for the dashboard, also added to the IAM policy.</p>
    pub fn get_dashboard_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dashboard_id()
    }
    /// <p>The display name of the dashboard.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The display name of the dashboard.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The display name of the dashboard.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. </p>
    pub fn parameters(mut self, input: crate::types::Parameters) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. </p>
    pub fn set_parameters(mut self, input: ::std::option::Option<crate::types::Parameters>) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>The parameters for the creation of the dashboard, which you want to use to override the default settings. A dashboard can have any type of parameters, and some parameters might accept multiple values. </p>
    pub fn get_parameters(&self) -> &::std::option::Option<crate::types::Parameters> {
        self.inner.get_parameters()
    }
    /// Appends an item to `Permissions`.
    ///
    /// To override the contents of this collection use [`set_permissions`](Self::set_permissions).
    ///
    /// <p>A structure that contains the permissions of the dashboard. You can use this structure for granting permissions by providing a list of IAM action information for each principal ARN. </p>
    /// <p>To specify no permissions, omit the permissions list.</p>
    pub fn permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        self.inner = self.inner.permissions(input);
        self
    }
    /// <p>A structure that contains the permissions of the dashboard. You can use this structure for granting permissions by providing a list of IAM action information for each principal ARN. </p>
    /// <p>To specify no permissions, omit the permissions list.</p>
    pub fn set_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>) -> Self {
        self.inner = self.inner.set_permissions(input);
        self
    }
    /// <p>A structure that contains the permissions of the dashboard. You can use this structure for granting permissions by providing a list of IAM action information for each principal ARN. </p>
    /// <p>To specify no permissions, omit the permissions list.</p>
    pub fn get_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>> {
        self.inner.get_permissions()
    }
    /// <p>The entity that you are using as a source when you create the dashboard. In <code>SourceEntity</code>, you specify the type of object you're using as source. You can only create a dashboard from a template, so you use a <code>SourceTemplate</code> entity. If you need to create a dashboard from an analysis, first convert the analysis to a template by using the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateTemplate.html">CreateTemplate</a> </code> API operation. For <code>SourceTemplate</code>, specify the Amazon Resource Name (ARN) of the source template. The <code>SourceTemplate</code>ARN can contain any Amazon Web Services account and any Amazon QuickSight-supported Amazon Web Services Region. </p>
    /// <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn source_entity(mut self, input: crate::types::DashboardSourceEntity) -> Self {
        self.inner = self.inner.source_entity(input);
        self
    }
    /// <p>The entity that you are using as a source when you create the dashboard. In <code>SourceEntity</code>, you specify the type of object you're using as source. You can only create a dashboard from a template, so you use a <code>SourceTemplate</code> entity. If you need to create a dashboard from an analysis, first convert the analysis to a template by using the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateTemplate.html">CreateTemplate</a> </code> API operation. For <code>SourceTemplate</code>, specify the Amazon Resource Name (ARN) of the source template. The <code>SourceTemplate</code>ARN can contain any Amazon Web Services account and any Amazon QuickSight-supported Amazon Web Services Region. </p>
    /// <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn set_source_entity(mut self, input: ::std::option::Option<crate::types::DashboardSourceEntity>) -> Self {
        self.inner = self.inner.set_source_entity(input);
        self
    }
    /// <p>The entity that you are using as a source when you create the dashboard. In <code>SourceEntity</code>, you specify the type of object you're using as source. You can only create a dashboard from a template, so you use a <code>SourceTemplate</code> entity. If you need to create a dashboard from an analysis, first convert the analysis to a template by using the <code> <a href="https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateTemplate.html">CreateTemplate</a> </code> API operation. For <code>SourceTemplate</code>, specify the Amazon Resource Name (ARN) of the source template. The <code>SourceTemplate</code>ARN can contain any Amazon Web Services account and any Amazon QuickSight-supported Amazon Web Services Region. </p>
    /// <p>Use the <code>DataSetReferences</code> entity within <code>SourceTemplate</code> to list the replacement datasets for the placeholders listed in the original. The schema in each dataset must match its placeholder. </p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn get_source_entity(&self) -> &::std::option::Option<crate::types::DashboardSourceEntity> {
        self.inner.get_source_entity()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dashboard.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dashboard.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Contains a map of the key-value pairs for the resource tag or tags assigned to the dashboard.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>A description for the first version of the dashboard being created.</p>
    pub fn version_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_description(input.into());
        self
    }
    /// <p>A description for the first version of the dashboard being created.</p>
    pub fn set_version_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_description(input);
        self
    }
    /// <p>A description for the first version of the dashboard being created.</p>
    pub fn get_version_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_description()
    }
    /// <p>Options for publishing the dashboard when you create it:</p>
    /// <ul>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, Amazon QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .CSV format isn't enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. This option is <code>COLLAPSED</code> by default. </p> </li>
    /// </ul>
    pub fn dashboard_publish_options(mut self, input: crate::types::DashboardPublishOptions) -> Self {
        self.inner = self.inner.dashboard_publish_options(input);
        self
    }
    /// <p>Options for publishing the dashboard when you create it:</p>
    /// <ul>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, Amazon QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .CSV format isn't enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. This option is <code>COLLAPSED</code> by default. </p> </li>
    /// </ul>
    pub fn set_dashboard_publish_options(mut self, input: ::std::option::Option<crate::types::DashboardPublishOptions>) -> Self {
        self.inner = self.inner.set_dashboard_publish_options(input);
        self
    }
    /// <p>Options for publishing the dashboard when you create it:</p>
    /// <ul>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>AdHocFilteringOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. When this is set to <code>DISABLED</code>, Amazon QuickSight disables the left filter pane on the published dashboard, which can be used for ad hoc (one-time) filtering. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>AvailabilityStatus</code> for <code>ExportToCSVOption</code> - This status can be either <code>ENABLED</code> or <code>DISABLED</code>. The visual option to export data to .CSV format isn't enabled when this is set to <code>DISABLED</code>. This option is <code>ENABLED</code> by default. </p> </li>
    /// <li> <p> <code>VisibilityState</code> for <code>SheetControlsOption</code> - This visibility state can be either <code>COLLAPSED</code> or <code>EXPANDED</code>. This option is <code>COLLAPSED</code> by default. </p> </li>
    /// </ul>
    pub fn get_dashboard_publish_options(&self) -> &::std::option::Option<crate::types::DashboardPublishOptions> {
        self.inner.get_dashboard_publish_options()
    }
    /// <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If you add a value for this field, it overrides the value that is used in the source entity. The theme ARN must exist in the same Amazon Web Services account where you create the dashboard.</p>
    pub fn theme_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.theme_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If you add a value for this field, it overrides the value that is used in the source entity. The theme ARN must exist in the same Amazon Web Services account where you create the dashboard.</p>
    pub fn set_theme_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_theme_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the theme that is being used for this dashboard. If you add a value for this field, it overrides the value that is used in the source entity. The theme ARN must exist in the same Amazon Web Services account where you create the dashboard.</p>
    pub fn get_theme_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_theme_arn()
    }
    /// <p>The definition of a dashboard.</p>
    /// <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn definition(mut self, input: crate::types::DashboardVersionDefinition) -> Self {
        self.inner = self.inner.definition(input);
        self
    }
    /// <p>The definition of a dashboard.</p>
    /// <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<crate::types::DashboardVersionDefinition>) -> Self {
        self.inner = self.inner.set_definition(input);
        self
    }
    /// <p>The definition of a dashboard.</p>
    /// <p>A definition is the data model of all features in a Dashboard, Template, or Analysis.</p>
    /// <p>Either a <code>SourceEntity</code> or a <code>Definition</code> must be provided in order for the request to be valid.</p>
    pub fn get_definition(&self) -> &::std::option::Option<crate::types::DashboardVersionDefinition> {
        self.inner.get_definition()
    }
    /// <p>The option to relax the validation needed to create a dashboard with definition objects. This option skips the validation step for specific errors.</p>
    pub fn validation_strategy(mut self, input: crate::types::ValidationStrategy) -> Self {
        self.inner = self.inner.validation_strategy(input);
        self
    }
    /// <p>The option to relax the validation needed to create a dashboard with definition objects. This option skips the validation step for specific errors.</p>
    pub fn set_validation_strategy(mut self, input: ::std::option::Option<crate::types::ValidationStrategy>) -> Self {
        self.inner = self.inner.set_validation_strategy(input);
        self
    }
    /// <p>The option to relax the validation needed to create a dashboard with definition objects. This option skips the validation step for specific errors.</p>
    pub fn get_validation_strategy(&self) -> &::std::option::Option<crate::types::ValidationStrategy> {
        self.inner.get_validation_strategy()
    }
    /// Appends an item to `FolderArns`.
    ///
    /// To override the contents of this collection use [`set_folder_arns`](Self::set_folder_arns).
    ///
    /// <p>When you create the dashboard, Amazon QuickSight adds the dashboard to these folders.</p>
    pub fn folder_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.folder_arns(input.into());
        self
    }
    /// <p>When you create the dashboard, Amazon QuickSight adds the dashboard to these folders.</p>
    pub fn set_folder_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_folder_arns(input);
        self
    }
    /// <p>When you create the dashboard, Amazon QuickSight adds the dashboard to these folders.</p>
    pub fn get_folder_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_folder_arns()
    }
    /// <p>A structure that contains the permissions of a shareable link to the dashboard.</p>
    pub fn link_sharing_configuration(mut self, input: crate::types::LinkSharingConfiguration) -> Self {
        self.inner = self.inner.link_sharing_configuration(input);
        self
    }
    /// <p>A structure that contains the permissions of a shareable link to the dashboard.</p>
    pub fn set_link_sharing_configuration(mut self, input: ::std::option::Option<crate::types::LinkSharingConfiguration>) -> Self {
        self.inner = self.inner.set_link_sharing_configuration(input);
        self
    }
    /// <p>A structure that contains the permissions of a shareable link to the dashboard.</p>
    pub fn get_link_sharing_configuration(&self) -> &::std::option::Option<crate::types::LinkSharingConfiguration> {
        self.inner.get_link_sharing_configuration()
    }
}
