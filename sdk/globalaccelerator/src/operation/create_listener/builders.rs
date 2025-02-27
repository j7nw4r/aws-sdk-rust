// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_listener::_create_listener_output::CreateListenerOutputBuilder;

pub use crate::operation::create_listener::_create_listener_input::CreateListenerInputBuilder;

impl CreateListenerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_listener::CreateListenerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_listener::CreateListenerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_listener();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateListener`.
///
/// <p>Create a listener to process inbound connections from clients to an accelerator. Connections arrive to assigned static IP addresses on a port, port range, or list of port ranges that you specify. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateListenerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_listener::builders::CreateListenerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_listener::CreateListenerOutput,
        crate::operation::create_listener::CreateListenerError,
    > for CreateListenerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_listener::CreateListenerOutput,
            crate::operation::create_listener::CreateListenerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateListenerFluentBuilder {
    /// Creates a new `CreateListener`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateListener as a reference.
    pub fn as_input(&self) -> &crate::operation::create_listener::builders::CreateListenerInputBuilder {
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
        crate::operation::create_listener::CreateListenerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_listener::CreateListenerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_listener::CreateListener::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_listener::CreateListener::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_listener::CreateListenerOutput,
        crate::operation::create_listener::CreateListenerError,
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
    /// <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    pub fn accelerator_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.accelerator_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    pub fn set_accelerator_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_accelerator_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your accelerator.</p>
    pub fn get_accelerator_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_accelerator_arn()
    }
    /// Appends an item to `PortRanges`.
    ///
    /// To override the contents of this collection use [`set_port_ranges`](Self::set_port_ranges).
    ///
    /// <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    pub fn port_ranges(mut self, input: crate::types::PortRange) -> Self {
        self.inner = self.inner.port_ranges(input);
        self
    }
    /// <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    pub fn set_port_ranges(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PortRange>>) -> Self {
        self.inner = self.inner.set_port_ranges(input);
        self
    }
    /// <p>The list of port ranges to support for connections from clients to your accelerator.</p>
    pub fn get_port_ranges(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PortRange>> {
        self.inner.get_port_ranges()
    }
    /// <p>The protocol for connections from clients to your accelerator.</p>
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol for connections from clients to your accelerator.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::Protocol>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The protocol for connections from clients to your accelerator.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<crate::types::Protocol> {
        self.inner.get_protocol()
    }
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p>
    /// <p>Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p>
    /// <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p>
    /// <p>The default value is <code>NONE</code>.</p>
    pub fn client_affinity(mut self, input: crate::types::ClientAffinity) -> Self {
        self.inner = self.inner.client_affinity(input);
        self
    }
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p>
    /// <p>Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p>
    /// <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p>
    /// <p>The default value is <code>NONE</code>.</p>
    pub fn set_client_affinity(mut self, input: ::std::option::Option<crate::types::ClientAffinity>) -> Self {
        self.inner = self.inner.set_client_affinity(input);
        self
    }
    /// <p>Client affinity lets you direct all requests from a user to the same endpoint, if you have stateful applications, regardless of the port and protocol of the client request. Client affinity gives you control over whether to always route each client to the same specific endpoint.</p>
    /// <p>Global Accelerator uses a consistent-flow hashing algorithm to choose the optimal endpoint for a connection. If client affinity is <code>NONE</code>, Global Accelerator uses the "five-tuple" (5-tuple) properties—source IP address, source port, destination IP address, destination port, and protocol—to select the hash value, and then chooses the best endpoint. However, with this setting, if someone uses different ports to connect to Global Accelerator, their connections might not be always routed to the same endpoint because the hash value changes. </p>
    /// <p>If you want a given client to always be routed to the same endpoint, set client affinity to <code>SOURCE_IP</code> instead. When you use the <code>SOURCE_IP</code> setting, Global Accelerator uses the "two-tuple" (2-tuple) properties— source (client) IP address and destination IP address—to select the hash value.</p>
    /// <p>The default value is <code>NONE</code>.</p>
    pub fn get_client_affinity(&self) -> &::std::option::Option<crate::types::ClientAffinity> {
        self.inner.get_client_affinity()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn idempotency_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn set_idempotency_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn get_idempotency_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_idempotency_token()
    }
}
