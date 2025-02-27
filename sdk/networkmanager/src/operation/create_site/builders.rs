// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_site::_create_site_output::CreateSiteOutputBuilder;

pub use crate::operation::create_site::_create_site_input::CreateSiteInputBuilder;

impl CreateSiteInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_site::CreateSiteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_site();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSite`.
///
/// <p>Creates a new site in a global network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSiteFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_site::builders::CreateSiteInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_site::CreateSiteOutput,
        crate::operation::create_site::CreateSiteError,
    > for CreateSiteFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_site::CreateSiteOutput,
            crate::operation::create_site::CreateSiteError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSiteFluentBuilder {
    /// Creates a new `CreateSite`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSite as a reference.
    pub fn as_input(&self) -> &crate::operation::create_site::builders::CreateSiteInputBuilder {
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
        crate::operation::create_site::CreateSiteOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_site::CreateSiteError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_site::CreateSite::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_site::CreateSite::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_site::CreateSiteOutput,
        crate::operation::create_site::CreateSiteError,
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
    /// <p>The ID of the global network.</p>
    pub fn global_network_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_network_id(input.into());
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn set_global_network_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_network_id(input);
        self
    }
    /// <p>The ID of the global network.</p>
    pub fn get_global_network_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_network_id()
    }
    /// <p>A description of your site.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of your site.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of your site.</p>
    /// <p>Constraints: Maximum length of 256 characters.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
    /// <ul>
    /// <li> <p> <code>Address</code>: The physical address of the site.</p> </li>
    /// <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>
    /// <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>
    /// </ul>
    pub fn location(mut self, input: crate::types::Location) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
    /// <ul>
    /// <li> <p> <code>Address</code>: The physical address of the site.</p> </li>
    /// <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>
    /// <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>
    /// </ul>
    pub fn set_location(mut self, input: ::std::option::Option<crate::types::Location>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The site location. This information is used for visualization in the Network Manager console. If you specify the address, the latitude and longitude are automatically calculated.</p>
    /// <ul>
    /// <li> <p> <code>Address</code>: The physical address of the site.</p> </li>
    /// <li> <p> <code>Latitude</code>: The latitude of the site. </p> </li>
    /// <li> <p> <code>Longitude</code>: The longitude of the site.</p> </li>
    /// </ul>
    pub fn get_location(&self) -> &::std::option::Option<crate::types::Location> {
        self.inner.get_location()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to apply to the resource during creation.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
