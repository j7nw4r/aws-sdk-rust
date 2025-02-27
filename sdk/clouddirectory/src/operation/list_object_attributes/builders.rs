// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_object_attributes::_list_object_attributes_output::ListObjectAttributesOutputBuilder;

pub use crate::operation::list_object_attributes::_list_object_attributes_input::ListObjectAttributesInputBuilder;

impl ListObjectAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_object_attributes::ListObjectAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_object_attributes::ListObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_object_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListObjectAttributes`.
///
/// <p>Lists all attributes that are associated with an object. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListObjectAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_object_attributes::builders::ListObjectAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_object_attributes::ListObjectAttributesOutput,
        crate::operation::list_object_attributes::ListObjectAttributesError,
    > for ListObjectAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_object_attributes::ListObjectAttributesOutput,
            crate::operation::list_object_attributes::ListObjectAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListObjectAttributesFluentBuilder {
    /// Creates a new `ListObjectAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListObjectAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::list_object_attributes::builders::ListObjectAttributesInputBuilder {
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
        crate::operation::list_object_attributes::ListObjectAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_object_attributes::ListObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_object_attributes::ListObjectAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_object_attributes::ListObjectAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_object_attributes::ListObjectAttributesOutput,
        crate::operation::list_object_attributes::ListObjectAttributesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_object_attributes::paginator::ListObjectAttributesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_object_attributes::paginator::ListObjectAttributesPaginator {
        crate::operation::list_object_attributes::paginator::ListObjectAttributesPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides. For more information, see <code>arns</code>.</p>
    pub fn directory_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides. For more information, see <code>arns</code>.</p>
    pub fn set_directory_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides. For more information, see <code>arns</code>.</p>
    pub fn get_directory_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_arn()
    }
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    pub fn object_reference(mut self, input: crate::types::ObjectReference) -> Self {
        self.inner = self.inner.object_reference(input);
        self
    }
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    pub fn set_object_reference(mut self, input: ::std::option::Option<crate::types::ObjectReference>) -> Self {
        self.inner = self.inner.set_object_reference(input);
        self
    }
    /// <p>The reference that identifies the object whose attributes will be listed.</p>
    pub fn get_object_reference(&self) -> &::std::option::Option<crate::types::ObjectReference> {
        self.inner.get_object_reference()
    }
    /// <p>The pagination token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to be retrieved in a single call. This is an approximate number.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    pub fn consistency_level(mut self, input: crate::types::ConsistencyLevel) -> Self {
        self.inner = self.inner.consistency_level(input);
        self
    }
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    pub fn set_consistency_level(mut self, input: ::std::option::Option<crate::types::ConsistencyLevel>) -> Self {
        self.inner = self.inner.set_consistency_level(input);
        self
    }
    /// <p>Represents the manner and timing in which the successful write or update of an object is reflected in a subsequent read operation of that same object.</p>
    pub fn get_consistency_level(&self) -> &::std::option::Option<crate::types::ConsistencyLevel> {
        self.inner.get_consistency_level()
    }
    /// <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
    pub fn facet_filter(mut self, input: crate::types::SchemaFacet) -> Self {
        self.inner = self.inner.facet_filter(input);
        self
    }
    /// <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
    pub fn set_facet_filter(mut self, input: ::std::option::Option<crate::types::SchemaFacet>) -> Self {
        self.inner = self.inner.set_facet_filter(input);
        self
    }
    /// <p>Used to filter the list of object attributes that are associated with a certain facet.</p>
    pub fn get_facet_filter(&self) -> &::std::option::Option<crate::types::SchemaFacet> {
        self.inner.get_facet_filter()
    }
}
