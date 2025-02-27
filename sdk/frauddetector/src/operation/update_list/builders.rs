// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_list::_update_list_output::UpdateListOutputBuilder;

pub use crate::operation::update_list::_update_list_input::UpdateListInputBuilder;

impl UpdateListInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_list::UpdateListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_list::UpdateListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_list();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateList`.
///
/// <p> Updates a list. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateListFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_list::builders::UpdateListInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_list::UpdateListOutput,
        crate::operation::update_list::UpdateListError,
    > for UpdateListFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_list::UpdateListOutput,
            crate::operation::update_list::UpdateListError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateListFluentBuilder {
    /// Creates a new `UpdateList`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateList as a reference.
    pub fn as_input(&self) -> &crate::operation::update_list::builders::UpdateListInputBuilder {
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
        crate::operation::update_list::UpdateListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_list::UpdateListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_list::UpdateList::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_list::UpdateList::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_list::UpdateListOutput,
        crate::operation::update_list::UpdateListError,
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
    /// <p> The name of the list to update. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p> The name of the list to update. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p> The name of the list to update. </p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// Appends an item to `elements`.
    ///
    /// To override the contents of this collection use [`set_elements`](Self::set_elements).
    ///
    /// <p> One or more list elements to add or replace. If you are providing the elements, make sure to specify the <code>updateMode</code> to use. </p>
    /// <p>If you are deleting all elements from the list, use <code>REPLACE</code> for the <code>updateMode</code> and provide an empty list (0 elements).</p>
    pub fn elements(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.elements(input.into());
        self
    }
    /// <p> One or more list elements to add or replace. If you are providing the elements, make sure to specify the <code>updateMode</code> to use. </p>
    /// <p>If you are deleting all elements from the list, use <code>REPLACE</code> for the <code>updateMode</code> and provide an empty list (0 elements).</p>
    pub fn set_elements(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_elements(input);
        self
    }
    /// <p> One or more list elements to add or replace. If you are providing the elements, make sure to specify the <code>updateMode</code> to use. </p>
    /// <p>If you are deleting all elements from the list, use <code>REPLACE</code> for the <code>updateMode</code> and provide an empty list (0 elements).</p>
    pub fn get_elements(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_elements()
    }
    /// <p> The new description. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p> The new description. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> The new description. </p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p> The update mode (type). </p>
    /// <ul>
    /// <li> <p>Use <code>APPEND</code> if you are adding elements to the list.</p> </li>
    /// <li> <p>Use <code>REPLACE</code> if you replacing existing elements in the list.</p> </li>
    /// <li> <p>Use <code>REMOVE</code> if you are removing elements from the list.</p> </li>
    /// </ul>
    pub fn update_mode(mut self, input: crate::types::ListUpdateMode) -> Self {
        self.inner = self.inner.update_mode(input);
        self
    }
    /// <p> The update mode (type). </p>
    /// <ul>
    /// <li> <p>Use <code>APPEND</code> if you are adding elements to the list.</p> </li>
    /// <li> <p>Use <code>REPLACE</code> if you replacing existing elements in the list.</p> </li>
    /// <li> <p>Use <code>REMOVE</code> if you are removing elements from the list.</p> </li>
    /// </ul>
    pub fn set_update_mode(mut self, input: ::std::option::Option<crate::types::ListUpdateMode>) -> Self {
        self.inner = self.inner.set_update_mode(input);
        self
    }
    /// <p> The update mode (type). </p>
    /// <ul>
    /// <li> <p>Use <code>APPEND</code> if you are adding elements to the list.</p> </li>
    /// <li> <p>Use <code>REPLACE</code> if you replacing existing elements in the list.</p> </li>
    /// <li> <p>Use <code>REMOVE</code> if you are removing elements from the list.</p> </li>
    /// </ul>
    pub fn get_update_mode(&self) -> &::std::option::Option<crate::types::ListUpdateMode> {
        self.inner.get_update_mode()
    }
    /// <p> The variable type you want to assign to the list. </p> <note>
    /// <p>You cannot update a variable type of a list that already has a variable type assigned to it. You can assign a variable type to a list only if the list does not already have a variable type.</p>
    /// </note>
    pub fn variable_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.variable_type(input.into());
        self
    }
    /// <p> The variable type you want to assign to the list. </p> <note>
    /// <p>You cannot update a variable type of a list that already has a variable type assigned to it. You can assign a variable type to a list only if the list does not already have a variable type.</p>
    /// </note>
    pub fn set_variable_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_variable_type(input);
        self
    }
    /// <p> The variable type you want to assign to the list. </p> <note>
    /// <p>You cannot update a variable type of a list that already has a variable type assigned to it. You can assign a variable type to a list only if the list does not already have a variable type.</p>
    /// </note>
    pub fn get_variable_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_variable_type()
    }
}
