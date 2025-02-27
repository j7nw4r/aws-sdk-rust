// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_event_tracker::_create_event_tracker_output::CreateEventTrackerOutputBuilder;

pub use crate::operation::create_event_tracker::_create_event_tracker_input::CreateEventTrackerInputBuilder;

impl CreateEventTrackerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_event_tracker::CreateEventTrackerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_event_tracker::CreateEventTrackerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_event_tracker();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateEventTracker`.
///
/// <p>Creates an event tracker that you use when adding event data to a specified dataset group using the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> API.</p> <note>
/// <p>Only one event tracker can be associated with a dataset group. You will get an error if you call <code>CreateEventTracker</code> using the same dataset group as an existing event tracker.</p>
/// </note>
/// <p>When you create an event tracker, the response includes a tracking ID, which you pass as a parameter when you use the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_UBS_PutEvents.html">PutEvents</a> operation. Amazon Personalize then appends the event data to the Item interactions dataset of the dataset group you specify in your event tracker. </p>
/// <p>The event tracker can be in one of the following states:</p>
/// <ul>
/// <li> <p>CREATE PENDING &gt; CREATE IN_PROGRESS &gt; ACTIVE -or- CREATE FAILED</p> </li>
/// <li> <p>DELETE PENDING &gt; DELETE IN_PROGRESS</p> </li>
/// </ul>
/// <p>To get the status of the event tracker, call <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_DescribeEventTracker.html">DescribeEventTracker</a>.</p> <note>
/// <p>The event tracker must be in the ACTIVE state before using the tracking ID.</p>
/// </note>
/// <p class="title"> <b>Related APIs</b> </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_ListEventTrackers.html">ListEventTrackers</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_DescribeEventTracker.html">DescribeEventTracker</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_DeleteEventTracker.html">DeleteEventTracker</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateEventTrackerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_event_tracker::builders::CreateEventTrackerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_event_tracker::CreateEventTrackerOutput,
        crate::operation::create_event_tracker::CreateEventTrackerError,
    > for CreateEventTrackerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_event_tracker::CreateEventTrackerOutput,
            crate::operation::create_event_tracker::CreateEventTrackerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateEventTrackerFluentBuilder {
    /// Creates a new `CreateEventTracker`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateEventTracker as a reference.
    pub fn as_input(&self) -> &crate::operation::create_event_tracker::builders::CreateEventTrackerInputBuilder {
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
        crate::operation::create_event_tracker::CreateEventTrackerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_event_tracker::CreateEventTrackerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_event_tracker::CreateEventTracker::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_event_tracker::CreateEventTracker::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_event_tracker::CreateEventTrackerOutput,
        crate::operation::create_event_tracker::CreateEventTrackerError,
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
    /// <p>The name for the event tracker.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name for the event tracker.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name for the event tracker.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
    pub fn dataset_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
    pub fn set_dataset_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_group_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the dataset group that receives the event data.</p>
    pub fn get_dataset_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_group_arn()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the event tracker.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the event tracker.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dg/tagging-resources.html">tags</a> to apply to the event tracker.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
