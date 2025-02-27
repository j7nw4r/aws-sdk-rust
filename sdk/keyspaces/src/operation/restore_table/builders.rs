// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_table::_restore_table_output::RestoreTableOutputBuilder;

pub use crate::operation::restore_table::_restore_table_input::RestoreTableInputBuilder;

impl RestoreTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_table::RestoreTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_table::RestoreTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restore_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreTable`.
///
/// <p>Restores the specified table to the specified point in time within the <code>earliest_restorable_timestamp</code> and the current time. For more information about restore points, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery_HowItWorks.html#howitworks_backup_window"> Time window for PITR continuous backups</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
/// <p>Any number of users can execute up to 4 concurrent restores (any type of restore) in a given account.</p>
/// <p>When you restore using point in time recovery, Amazon Keyspaces restores your source table's schema and data to the state based on the selected timestamp <code>(day:hour:minute:second)</code> to a new table. The Time to Live (TTL) settings are also restored to the state based on the selected timestamp.</p>
/// <p>In addition to the table's schema, data, and TTL settings, <code>RestoreTable</code> restores the capacity mode, encryption, and point-in-time recovery settings from the source table. Unlike the table's schema data and TTL settings, which are restored based on the selected timestamp, these settings are always restored based on the table's settings as of the current time or when the table was deleted.</p>
/// <p>You can also overwrite these settings during restore:</p>
/// <ul>
/// <li> <p>Read/write capacity mode</p> </li>
/// <li> <p>Provisioned throughput capacity settings</p> </li>
/// <li> <p>Point-in-time (PITR) settings</p> </li>
/// <li> <p>Tags</p> </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery_HowItWorks.html#howitworks_backup_settings">PITR restore settings</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
/// <p>Note that the following settings are not restored, and you must configure them manually for the new table:</p>
/// <ul>
/// <li> <p>Automatic scaling policies (for tables that use provisioned capacity mode)</p> </li>
/// <li> <p>Identity and Access Management (IAM) policies</p> </li>
/// <li> <p>Amazon CloudWatch metrics and alarms</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restore_table::builders::RestoreTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::restore_table::RestoreTableOutput,
        crate::operation::restore_table::RestoreTableError,
    > for RestoreTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::restore_table::RestoreTableOutput,
            crate::operation::restore_table::RestoreTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RestoreTableFluentBuilder {
    /// Creates a new `RestoreTable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestoreTable as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_table::builders::RestoreTableInputBuilder {
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
        crate::operation::restore_table::RestoreTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_table::RestoreTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restore_table::RestoreTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restore_table::RestoreTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::restore_table::RestoreTableOutput,
        crate::operation::restore_table::RestoreTableError,
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
    /// <p>The keyspace name of the source table.</p>
    pub fn source_keyspace_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_keyspace_name(input.into());
        self
    }
    /// <p>The keyspace name of the source table.</p>
    pub fn set_source_keyspace_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_keyspace_name(input);
        self
    }
    /// <p>The keyspace name of the source table.</p>
    pub fn get_source_keyspace_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_keyspace_name()
    }
    /// <p>The name of the source table.</p>
    pub fn source_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_table_name(input.into());
        self
    }
    /// <p>The name of the source table.</p>
    pub fn set_source_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_table_name(input);
        self
    }
    /// <p>The name of the source table.</p>
    pub fn get_source_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_table_name()
    }
    /// <p>The name of the target keyspace.</p>
    pub fn target_keyspace_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_keyspace_name(input.into());
        self
    }
    /// <p>The name of the target keyspace.</p>
    pub fn set_target_keyspace_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_keyspace_name(input);
        self
    }
    /// <p>The name of the target keyspace.</p>
    pub fn get_target_keyspace_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_keyspace_name()
    }
    /// <p>The name of the target table.</p>
    pub fn target_table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_table_name(input.into());
        self
    }
    /// <p>The name of the target table.</p>
    pub fn set_target_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_table_name(input);
        self
    }
    /// <p>The name of the target table.</p>
    pub fn get_target_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_table_name()
    }
    /// <p>The restore timestamp in ISO 8601 format.</p>
    pub fn restore_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.restore_timestamp(input);
        self
    }
    /// <p>The restore timestamp in ISO 8601 format.</p>
    pub fn set_restore_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_restore_timestamp(input);
        self
    }
    /// <p>The restore timestamp in ISO 8601 format.</p>
    pub fn get_restore_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_restore_timestamp()
    }
    /// <p>Specifies the read/write throughput capacity mode for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn capacity_specification_override(mut self, input: crate::types::CapacitySpecification) -> Self {
        self.inner = self.inner.capacity_specification_override(input);
        self
    }
    /// <p>Specifies the read/write throughput capacity mode for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_capacity_specification_override(mut self, input: ::std::option::Option<crate::types::CapacitySpecification>) -> Self {
        self.inner = self.inner.set_capacity_specification_override(input);
        self
    }
    /// <p>Specifies the read/write throughput capacity mode for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn get_capacity_specification_override(&self) -> &::std::option::Option<crate::types::CapacitySpecification> {
        self.inner.get_capacity_specification_override()
    }
    /// <p>Specifies the encryption settings for the target table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input. </p> </li>
    /// </ul>
    /// <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn encryption_specification_override(mut self, input: crate::types::EncryptionSpecification) -> Self {
        self.inner = self.inner.encryption_specification_override(input);
        self
    }
    /// <p>Specifies the encryption settings for the target table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input. </p> </li>
    /// </ul>
    /// <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_encryption_specification_override(mut self, input: ::std::option::Option<crate::types::EncryptionSpecification>) -> Self {
        self.inner = self.inner.set_encryption_specification_override(input);
        self
    }
    /// <p>Specifies the encryption settings for the target table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input. </p> </li>
    /// </ul>
    /// <p>The default is <code>type:AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn get_encryption_specification_override(&self) -> &::std::option::Option<crate::types::EncryptionSpecification> {
        self.inner.get_encryption_specification_override()
    }
    /// <p>Specifies the <code>pointInTimeRecovery</code> settings for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn point_in_time_recovery_override(mut self, input: crate::types::PointInTimeRecovery) -> Self {
        self.inner = self.inner.point_in_time_recovery_override(input);
        self
    }
    /// <p>Specifies the <code>pointInTimeRecovery</code> settings for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_point_in_time_recovery_override(mut self, input: ::std::option::Option<crate::types::PointInTimeRecovery>) -> Self {
        self.inner = self.inner.set_point_in_time_recovery_override(input);
        self
    }
    /// <p>Specifies the <code>pointInTimeRecovery</code> settings for the target table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn get_point_in_time_recovery_override(&self) -> &::std::option::Option<crate::types::PointInTimeRecovery> {
        self.inner.get_point_in_time_recovery_override()
    }
    /// Appends an item to `tagsOverride`.
    ///
    /// To override the contents of this collection use [`set_tags_override`](Self::set_tags_override).
    ///
    /// <p>A list of key-value pair tags to be attached to the restored table. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn tags_override(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags_override(input);
        self
    }
    /// <p>A list of key-value pair tags to be attached to the restored table. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_tags_override(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags_override(input);
        self
    }
    /// <p>A list of key-value pair tags to be attached to the restored table. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html">Adding tags and labels to Amazon Keyspaces resources</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn get_tags_override(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags_override()
    }
}
