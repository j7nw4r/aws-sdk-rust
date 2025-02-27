// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_instance_profile::_tag_instance_profile_output::TagInstanceProfileOutputBuilder;

pub use crate::operation::tag_instance_profile::_tag_instance_profile_input::TagInstanceProfileInputBuilder;

impl TagInstanceProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::tag_instance_profile::TagInstanceProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_instance_profile::TagInstanceProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.tag_instance_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TagInstanceProfile`.
///
/// <p>Adds one or more tags to an IAM instance profile. If a tag with the same key name already exists, then that tag is overwritten with the new value.</p>
/// <p>Each tag consists of a key name and an associated value. By assigning tags to your resources, you can do the following:</p>
/// <ul>
/// <li> <p> <b>Administrative grouping and discovery</b> - Attach tags to resources to aid in organization and search. For example, you could search for all resources with the key name <i>Project</i> and the value <i>MyImportantProject</i>. Or search for all resources with the key name <i>Cost Center</i> and the value <i>41200</i>. </p> </li>
/// <li> <p> <b>Access control</b> - Include tags in IAM user-based and resource-based policies. You can use tags to restrict access to only an IAM instance profile that has a specified tag attached. For examples of policies that show how to use tags to control access, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html">Control access using IAM tags</a> in the <i>IAM User Guide</i>.</p> </li>
/// </ul> <note>
/// <ul>
/// <li> <p>If any one of the tags is invalid or if you exceed the allowed maximum number of tags, then the entire request fails and the resource is not created. For more information about tagging, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_tags.html">Tagging IAM resources</a> in the <i>IAM User Guide</i>.</p> </li>
/// <li> <p>Amazon Web Services always interprets the tag <code>Value</code> as a single string. If you need to store an array, you can store comma-separated values in the string. However, you must interpret the value in your code.</p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TagInstanceProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_instance_profile::builders::TagInstanceProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::tag_instance_profile::TagInstanceProfileOutput,
        crate::operation::tag_instance_profile::TagInstanceProfileError,
    > for TagInstanceProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::tag_instance_profile::TagInstanceProfileOutput,
            crate::operation::tag_instance_profile::TagInstanceProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TagInstanceProfileFluentBuilder {
    /// Creates a new `TagInstanceProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TagInstanceProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::tag_instance_profile::builders::TagInstanceProfileInputBuilder {
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
        crate::operation::tag_instance_profile::TagInstanceProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_instance_profile::TagInstanceProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::tag_instance_profile::TagInstanceProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::tag_instance_profile::TagInstanceProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::tag_instance_profile::TagInstanceProfileOutput,
        crate::operation::tag_instance_profile::TagInstanceProfileError,
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
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_profile_name(input.into());
        self
    }
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_profile_name(input);
        self
    }
    /// <p>The name of the IAM instance profile to which you want to add tags.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn get_instance_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_profile_name()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The list of tags that you want to attach to the IAM instance profile. Each tag consists of a key name and an associated value.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
