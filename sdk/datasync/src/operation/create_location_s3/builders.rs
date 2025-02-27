// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_location_s3::_create_location_s3_output::CreateLocationS3OutputBuilder;

pub use crate::operation::create_location_s3::_create_location_s3_input::CreateLocationS3InputBuilder;

impl CreateLocationS3InputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_location_s3::CreateLocationS3Output,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_location_s3::CreateLocationS3Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_location_s3();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLocationS3`.
///
/// <p>A <i>location</i> is an endpoint for an Amazon S3 bucket. DataSync can use the location as a source or destination for copying data.</p> <important>
/// <p>Before you create your location, make sure that you read the following sections:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Storage class considerations with Amazon S3 locations</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#create-s3-location-s3-requests">Evaluating S3 request costs when using DataSync</a> </p> </li>
/// </ul>
/// </important>
/// <p> For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-locations-cli.html#create-location-s3-cli">Creating an Amazon S3 location</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLocationS3FluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_location_s3::builders::CreateLocationS3InputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_location_s3::CreateLocationS3Output,
        crate::operation::create_location_s3::CreateLocationS3Error,
    > for CreateLocationS3FluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_location_s3::CreateLocationS3Output,
            crate::operation::create_location_s3::CreateLocationS3Error,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLocationS3FluentBuilder {
    /// Creates a new `CreateLocationS3`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLocationS3 as a reference.
    pub fn as_input(&self) -> &crate::operation::create_location_s3::builders::CreateLocationS3InputBuilder {
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
        crate::operation::create_location_s3::CreateLocationS3Output,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_location_s3::CreateLocationS3Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_location_s3::CreateLocationS3::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_location_s3::CreateLocationS3::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_location_s3::CreateLocationS3Output,
        crate::operation::create_location_s3::CreateLocationS3Error,
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
    /// <p>A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.</p>
    pub fn subdirectory(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subdirectory(input.into());
        self
    }
    /// <p>A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.</p>
    pub fn set_subdirectory(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subdirectory(input);
        self
    }
    /// <p>A subdirectory in the Amazon S3 bucket. This subdirectory in Amazon S3 is used to read data from the S3 source location or write data to the S3 destination.</p>
    pub fn get_subdirectory(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subdirectory()
    }
    /// <p>The ARN of the Amazon S3 bucket. If the bucket is on an Amazon Web Services Outpost, this must be an access point ARN.</p>
    pub fn s3_bucket_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.s3_bucket_arn(input.into());
        self
    }
    /// <p>The ARN of the Amazon S3 bucket. If the bucket is on an Amazon Web Services Outpost, this must be an access point ARN.</p>
    pub fn set_s3_bucket_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_s3_bucket_arn(input);
        self
    }
    /// <p>The ARN of the Amazon S3 bucket. If the bucket is on an Amazon Web Services Outpost, this must be an access point ARN.</p>
    pub fn get_s3_bucket_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_s3_bucket_arn()
    }
    /// <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For buckets in Amazon Web Services Regions, the storage class defaults to Standard. For buckets on Outposts, the storage class defaults to Amazon Web Services S3 Outposts.</p>
    /// <p>For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Considerations when working with S3 storage classes in DataSync</a>.</p>
    pub fn s3_storage_class(mut self, input: crate::types::S3StorageClass) -> Self {
        self.inner = self.inner.s3_storage_class(input);
        self
    }
    /// <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For buckets in Amazon Web Services Regions, the storage class defaults to Standard. For buckets on Outposts, the storage class defaults to Amazon Web Services S3 Outposts.</p>
    /// <p>For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Considerations when working with S3 storage classes in DataSync</a>.</p>
    pub fn set_s3_storage_class(mut self, input: ::std::option::Option<crate::types::S3StorageClass>) -> Self {
        self.inner = self.inner.set_s3_storage_class(input);
        self
    }
    /// <p>The Amazon S3 storage class that you want to store your files in when this location is used as a task destination. For buckets in Amazon Web Services Regions, the storage class defaults to Standard. For buckets on Outposts, the storage class defaults to Amazon Web Services S3 Outposts.</p>
    /// <p>For more information about S3 storage classes, see <a href="http://aws.amazon.com/s3/storage-classes/">Amazon S3 Storage Classes</a>. Some storage classes have behaviors that can affect your S3 storage cost. For detailed information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-s3-location.html#using-storage-classes">Considerations when working with S3 storage classes in DataSync</a>.</p>
    pub fn get_s3_storage_class(&self) -> &::std::option::Option<crate::types::S3StorageClass> {
        self.inner.get_s3_storage_class()
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role used to access an Amazon S3 bucket.</p>
    /// <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>DataSync User Guide</i>.</p>
    pub fn s3_config(mut self, input: crate::types::S3Config) -> Self {
        self.inner = self.inner.s3_config(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role used to access an Amazon S3 bucket.</p>
    /// <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>DataSync User Guide</i>.</p>
    pub fn set_s3_config(mut self, input: ::std::option::Option<crate::types::S3Config>) -> Self {
        self.inner = self.inner.set_s3_config(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role used to access an Amazon S3 bucket.</p>
    /// <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>DataSync User Guide</i>.</p>
    pub fn get_s3_config(&self) -> &::std::option::Option<crate::types::S3Config> {
        self.inner.get_s3_config()
    }
    /// Appends an item to `AgentArns`.
    ///
    /// To override the contents of this collection use [`set_agent_arns`](Self::set_agent_arns).
    ///
    /// <p>If you're using DataSync on an Amazon Web Services Outpost, specify the Amazon Resource Names (ARNs) of the DataSync agents deployed on your Outpost. For more information about launching a DataSync agent on an Amazon Web Services Outpost, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent on Outposts</a>.</p>
    pub fn agent_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_arns(input.into());
        self
    }
    /// <p>If you're using DataSync on an Amazon Web Services Outpost, specify the Amazon Resource Names (ARNs) of the DataSync agents deployed on your Outpost. For more information about launching a DataSync agent on an Amazon Web Services Outpost, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent on Outposts</a>.</p>
    pub fn set_agent_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_agent_arns(input);
        self
    }
    /// <p>If you're using DataSync on an Amazon Web Services Outpost, specify the Amazon Resource Names (ARNs) of the DataSync agents deployed on your Outpost. For more information about launching a DataSync agent on an Amazon Web Services Outpost, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/deploy-agents.html#outposts-agent">Deploy your DataSync agent on Outposts</a>.</p>
    pub fn get_agent_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_agent_arns()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    pub fn tags(mut self, input: crate::types::TagListEntry) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The key-value pair that represents the tag that you want to add to the location. The value can be an empty string. We recommend using tags to name your resources.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>> {
        self.inner.get_tags()
    }
}
