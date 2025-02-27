// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_lifecycle_policy::_create_lifecycle_policy_output::CreateLifecyclePolicyOutputBuilder;

pub use crate::operation::create_lifecycle_policy::_create_lifecycle_policy_input::CreateLifecyclePolicyInputBuilder;

impl CreateLifecyclePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_lifecycle_policy::CreateLifecyclePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_lifecycle_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLifecyclePolicy`.
///
/// <p>Creates an Amazon Data Lifecycle Manager lifecycle policy. Amazon Data Lifecycle Manager supports the following policy types:</p>
/// <ul>
/// <li> <p>Custom EBS snapshot policy</p> </li>
/// <li> <p>Custom EBS-backed AMI policy</p> </li>
/// <li> <p>Cross-account copy event policy</p> </li>
/// <li> <p>Default policy for EBS snapshots</p> </li>
/// <li> <p>Default policy for EBS-backed AMIs</p> </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/policy-differences.html"> Default policies vs custom policies</a>.</p> <important>
/// <p>If you create a default policy, you can specify the request parameters either in the request body, or in the PolicyDetails request structure, but not both.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLifecyclePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_lifecycle_policy::builders::CreateLifecyclePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyOutput,
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyError,
    > for CreateLifecyclePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_lifecycle_policy::CreateLifecyclePolicyOutput,
            crate::operation::create_lifecycle_policy::CreateLifecyclePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLifecyclePolicyFluentBuilder {
    /// Creates a new `CreateLifecyclePolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLifecyclePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::create_lifecycle_policy::builders::CreateLifecyclePolicyInputBuilder {
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
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_lifecycle_policy::CreateLifecyclePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_lifecycle_policy::CreateLifecyclePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyOutput,
        crate::operation::create_lifecycle_policy::CreateLifecyclePolicyError,
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
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    pub fn execution_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    pub fn set_execution_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role used to run the operations specified by the lifecycle policy.</p>
    pub fn get_execution_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_role_arn()
    }
    /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are supported.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are supported.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the lifecycle policy. The characters ^[0-9A-Za-z _-]+$ are supported.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The activation state of the lifecycle policy after creation.</p>
    pub fn state(mut self, input: crate::types::SettablePolicyStateValues) -> Self {
        self.inner = self.inner.state(input);
        self
    }
    /// <p>The activation state of the lifecycle policy after creation.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::SettablePolicyStateValues>) -> Self {
        self.inner = self.inner.set_state(input);
        self
    }
    /// <p>The activation state of the lifecycle policy after creation.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::SettablePolicyStateValues> {
        self.inner.get_state()
    }
    /// <p>The configuration details of the lifecycle policy.</p> <important>
    /// <p>If you create a default policy, you can specify the request parameters either in the request body, or in the PolicyDetails request structure, but not both.</p>
    /// </important>
    pub fn policy_details(mut self, input: crate::types::PolicyDetails) -> Self {
        self.inner = self.inner.policy_details(input);
        self
    }
    /// <p>The configuration details of the lifecycle policy.</p> <important>
    /// <p>If you create a default policy, you can specify the request parameters either in the request body, or in the PolicyDetails request structure, but not both.</p>
    /// </important>
    pub fn set_policy_details(mut self, input: ::std::option::Option<crate::types::PolicyDetails>) -> Self {
        self.inner = self.inner.set_policy_details(input);
        self
    }
    /// <p>The configuration details of the lifecycle policy.</p> <important>
    /// <p>If you create a default policy, you can specify the request parameters either in the request body, or in the PolicyDetails request structure, but not both.</p>
    /// </important>
    pub fn get_policy_details(&self) -> &::std::option::Option<crate::types::PolicyDetails> {
        self.inner.get_policy_details()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the lifecycle policy during creation.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to apply to the lifecycle policy during creation.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to apply to the lifecycle policy during creation.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p> <b>[Default policies only]</b> Specify the type of default policy to create.</p>
    /// <ul>
    /// <li> <p>To create a default policy for EBS snapshots, that creates snapshots of all volumes in the Region that do not have recent backups, specify <code>VOLUME</code>.</p> </li>
    /// <li> <p>To create a default policy for EBS-backed AMIs, that creates EBS-backed AMIs from all instances in the Region that do not have recent backups, specify <code>INSTANCE</code>.</p> </li>
    /// </ul>
    pub fn default_policy(mut self, input: crate::types::DefaultPolicyTypeValues) -> Self {
        self.inner = self.inner.default_policy(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specify the type of default policy to create.</p>
    /// <ul>
    /// <li> <p>To create a default policy for EBS snapshots, that creates snapshots of all volumes in the Region that do not have recent backups, specify <code>VOLUME</code>.</p> </li>
    /// <li> <p>To create a default policy for EBS-backed AMIs, that creates EBS-backed AMIs from all instances in the Region that do not have recent backups, specify <code>INSTANCE</code>.</p> </li>
    /// </ul>
    pub fn set_default_policy(mut self, input: ::std::option::Option<crate::types::DefaultPolicyTypeValues>) -> Self {
        self.inner = self.inner.set_default_policy(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specify the type of default policy to create.</p>
    /// <ul>
    /// <li> <p>To create a default policy for EBS snapshots, that creates snapshots of all volumes in the Region that do not have recent backups, specify <code>VOLUME</code>.</p> </li>
    /// <li> <p>To create a default policy for EBS-backed AMIs, that creates EBS-backed AMIs from all instances in the Region that do not have recent backups, specify <code>INSTANCE</code>.</p> </li>
    /// </ul>
    pub fn get_default_policy(&self) -> &::std::option::Option<crate::types::DefaultPolicyTypeValues> {
        self.inner.get_default_policy()
    }
    /// <p> <b>[Default policies only]</b> Specifies how often the policy should run and create snapshots or AMIs. The creation frequency can range from 1 to 7 days. If you do not specify a value, the default is 1.</p>
    /// <p>Default: 1</p>
    pub fn create_interval(mut self, input: i32) -> Self {
        self.inner = self.inner.create_interval(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies how often the policy should run and create snapshots or AMIs. The creation frequency can range from 1 to 7 days. If you do not specify a value, the default is 1.</p>
    /// <p>Default: 1</p>
    pub fn set_create_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_create_interval(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies how often the policy should run and create snapshots or AMIs. The creation frequency can range from 1 to 7 days. If you do not specify a value, the default is 1.</p>
    /// <p>Default: 1</p>
    pub fn get_create_interval(&self) -> &::std::option::Option<i32> {
        self.inner.get_create_interval()
    }
    /// <p> <b>[Default policies only]</b> Specifies how long the policy should retain snapshots or AMIs before deleting them. The retention period can range from 2 to 14 days, but it must be greater than the creation frequency to ensure that the policy retains at least 1 snapshot or AMI at any given time. If you do not specify a value, the default is 7.</p>
    /// <p>Default: 7</p>
    pub fn retain_interval(mut self, input: i32) -> Self {
        self.inner = self.inner.retain_interval(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies how long the policy should retain snapshots or AMIs before deleting them. The retention period can range from 2 to 14 days, but it must be greater than the creation frequency to ensure that the policy retains at least 1 snapshot or AMI at any given time. If you do not specify a value, the default is 7.</p>
    /// <p>Default: 7</p>
    pub fn set_retain_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_retain_interval(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies how long the policy should retain snapshots or AMIs before deleting them. The retention period can range from 2 to 14 days, but it must be greater than the creation frequency to ensure that the policy retains at least 1 snapshot or AMI at any given time. If you do not specify a value, the default is 7.</p>
    /// <p>Default: 7</p>
    pub fn get_retain_interval(&self) -> &::std::option::Option<i32> {
        self.inner.get_retain_interval()
    }
    /// <p> <b>[Default policies only]</b> Indicates whether the policy should copy tags from the source resource to the snapshot or AMI. If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn copy_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_tags(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Indicates whether the policy should copy tags from the source resource to the snapshot or AMI. If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn set_copy_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_tags(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Indicates whether the policy should copy tags from the source resource to the snapshot or AMI. If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn get_copy_tags(&self) -> &::std::option::Option<bool> {
        self.inner.get_copy_tags()
    }
    /// <p> <b>[Default policies only]</b> Defines the snapshot or AMI retention behavior for the policy if the source volume or instance is deleted, or if the policy enters the error, disabled, or deleted state.</p>
    /// <p>By default (<b>ExtendDeletion=false</b>):</p>
    /// <ul>
    /// <li> <p>If a source resource is deleted, Amazon Data Lifecycle Manager will continue to delete previously created snapshots or AMIs, up to but not including the last one, based on the specified retention period. If you want Amazon Data Lifecycle Manager to delete all snapshots or AMIs, including the last one, specify <code>true</code>.</p> </li>
    /// <li> <p>If a policy enters the error, disabled, or deleted state, Amazon Data Lifecycle Manager stops deleting snapshots and AMIs. If you want Amazon Data Lifecycle Manager to continue deleting snapshots or AMIs, including the last one, if the policy enters one of these states, specify <code>true</code>.</p> </li>
    /// </ul>
    /// <p>If you enable extended deletion (<b>ExtendDeletion=true</b>), you override both default behaviors simultaneously.</p>
    /// <p>If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn extend_deletion(mut self, input: bool) -> Self {
        self.inner = self.inner.extend_deletion(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Defines the snapshot or AMI retention behavior for the policy if the source volume or instance is deleted, or if the policy enters the error, disabled, or deleted state.</p>
    /// <p>By default (<b>ExtendDeletion=false</b>):</p>
    /// <ul>
    /// <li> <p>If a source resource is deleted, Amazon Data Lifecycle Manager will continue to delete previously created snapshots or AMIs, up to but not including the last one, based on the specified retention period. If you want Amazon Data Lifecycle Manager to delete all snapshots or AMIs, including the last one, specify <code>true</code>.</p> </li>
    /// <li> <p>If a policy enters the error, disabled, or deleted state, Amazon Data Lifecycle Manager stops deleting snapshots and AMIs. If you want Amazon Data Lifecycle Manager to continue deleting snapshots or AMIs, including the last one, if the policy enters one of these states, specify <code>true</code>.</p> </li>
    /// </ul>
    /// <p>If you enable extended deletion (<b>ExtendDeletion=true</b>), you override both default behaviors simultaneously.</p>
    /// <p>If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn set_extend_deletion(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_extend_deletion(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Defines the snapshot or AMI retention behavior for the policy if the source volume or instance is deleted, or if the policy enters the error, disabled, or deleted state.</p>
    /// <p>By default (<b>ExtendDeletion=false</b>):</p>
    /// <ul>
    /// <li> <p>If a source resource is deleted, Amazon Data Lifecycle Manager will continue to delete previously created snapshots or AMIs, up to but not including the last one, based on the specified retention period. If you want Amazon Data Lifecycle Manager to delete all snapshots or AMIs, including the last one, specify <code>true</code>.</p> </li>
    /// <li> <p>If a policy enters the error, disabled, or deleted state, Amazon Data Lifecycle Manager stops deleting snapshots and AMIs. If you want Amazon Data Lifecycle Manager to continue deleting snapshots or AMIs, including the last one, if the policy enters one of these states, specify <code>true</code>.</p> </li>
    /// </ul>
    /// <p>If you enable extended deletion (<b>ExtendDeletion=true</b>), you override both default behaviors simultaneously.</p>
    /// <p>If you do not specify a value, the default is <code>false</code>.</p>
    /// <p>Default: false</p>
    pub fn get_extend_deletion(&self) -> &::std::option::Option<bool> {
        self.inner.get_extend_deletion()
    }
    /// Appends an item to `CrossRegionCopyTargets`.
    ///
    /// To override the contents of this collection use [`set_cross_region_copy_targets`](Self::set_cross_region_copy_targets).
    ///
    /// <p> <b>[Default policies only]</b> Specifies destination Regions for snapshot or AMI copies. You can specify up to 3 destination Regions. If you do not want to create cross-Region copies, omit this parameter.</p>
    pub fn cross_region_copy_targets(mut self, input: crate::types::CrossRegionCopyTarget) -> Self {
        self.inner = self.inner.cross_region_copy_targets(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies destination Regions for snapshot or AMI copies. You can specify up to 3 destination Regions. If you do not want to create cross-Region copies, omit this parameter.</p>
    pub fn set_cross_region_copy_targets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CrossRegionCopyTarget>>) -> Self {
        self.inner = self.inner.set_cross_region_copy_targets(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies destination Regions for snapshot or AMI copies. You can specify up to 3 destination Regions. If you do not want to create cross-Region copies, omit this parameter.</p>
    pub fn get_cross_region_copy_targets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CrossRegionCopyTarget>> {
        self.inner.get_cross_region_copy_targets()
    }
    /// <p> <b>[Default policies only]</b> Specifies exclusion parameters for volumes or instances for which you do not want to create snapshots or AMIs. The policy will not create snapshots or AMIs for target resources that match any of the specified exclusion parameters.</p>
    pub fn exclusions(mut self, input: crate::types::Exclusions) -> Self {
        self.inner = self.inner.exclusions(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies exclusion parameters for volumes or instances for which you do not want to create snapshots or AMIs. The policy will not create snapshots or AMIs for target resources that match any of the specified exclusion parameters.</p>
    pub fn set_exclusions(mut self, input: ::std::option::Option<crate::types::Exclusions>) -> Self {
        self.inner = self.inner.set_exclusions(input);
        self
    }
    /// <p> <b>[Default policies only]</b> Specifies exclusion parameters for volumes or instances for which you do not want to create snapshots or AMIs. The policy will not create snapshots or AMIs for target resources that match any of the specified exclusion parameters.</p>
    pub fn get_exclusions(&self) -> &::std::option::Option<crate::types::Exclusions> {
        self.inner.get_exclusions()
    }
}
