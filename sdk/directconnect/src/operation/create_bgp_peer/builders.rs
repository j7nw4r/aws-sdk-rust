// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_bgp_peer::_create_bgp_peer_output::CreateBgpPeerOutputBuilder;

pub use crate::operation::create_bgp_peer::_create_bgp_peer_input::CreateBgpPeerInputBuilder;

impl CreateBgpPeerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_bgp_peer::CreateBgpPeerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bgp_peer::CreateBGPPeerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_bgp_peer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateBGPPeer`.
///
/// <p>Creates a BGP peer on the specified virtual interface.</p>
/// <p>You must create a BGP peer for the corresponding address family (IPv4/IPv6) in order to access Amazon Web Services resources that also use that address family.</p>
/// <p>If logical redundancy is not supported by the connection, interconnect, or LAG, the BGP peer cannot be in the same address family as an existing BGP peer on the virtual interface.</p>
/// <p>When creating a IPv6 BGP peer, omit the Amazon address and customer address. IPv6 addresses are automatically assigned from the Amazon pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <important>
/// <p>If you let Amazon Web Services auto-assign IPv4 addresses, a /30 CIDR will be allocated from 169.254.0.0/16. Amazon Web Services does not recommend this option if you intend to use the customer router peer IP address as the source and destination for traffic. Instead you should use RFC 1918 or other addressing, and specify the address yourself. For more information about RFC 1918 see <a href="https://datatracker.ietf.org/doc/html/rfc1918"> Address Allocation for Private Internets</a>.</p>
/// </important>
/// <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already on the allow list for the virtual interface.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateBGPPeerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_bgp_peer::builders::CreateBgpPeerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_bgp_peer::CreateBgpPeerOutput,
        crate::operation::create_bgp_peer::CreateBGPPeerError,
    > for CreateBGPPeerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_bgp_peer::CreateBgpPeerOutput,
            crate::operation::create_bgp_peer::CreateBGPPeerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateBGPPeerFluentBuilder {
    /// Creates a new `CreateBGPPeer`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateBGPPeer as a reference.
    pub fn as_input(&self) -> &crate::operation::create_bgp_peer::builders::CreateBgpPeerInputBuilder {
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
        crate::operation::create_bgp_peer::CreateBgpPeerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bgp_peer::CreateBGPPeerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_bgp_peer::CreateBGPPeer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_bgp_peer::CreateBGPPeer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_bgp_peer::CreateBgpPeerOutput,
        crate::operation::create_bgp_peer::CreateBGPPeerError,
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
    /// <p>The ID of the virtual interface.</p>
    pub fn virtual_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.virtual_interface_id(input.into());
        self
    }
    /// <p>The ID of the virtual interface.</p>
    pub fn set_virtual_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_virtual_interface_id(input);
        self
    }
    /// <p>The ID of the virtual interface.</p>
    pub fn get_virtual_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_virtual_interface_id()
    }
    /// <p>Information about the BGP peer.</p>
    pub fn new_bgp_peer(mut self, input: crate::types::NewBgpPeer) -> Self {
        self.inner = self.inner.new_bgp_peer(input);
        self
    }
    /// <p>Information about the BGP peer.</p>
    pub fn set_new_bgp_peer(mut self, input: ::std::option::Option<crate::types::NewBgpPeer>) -> Self {
        self.inner = self.inner.set_new_bgp_peer(input);
        self
    }
    /// <p>Information about the BGP peer.</p>
    pub fn get_new_bgp_peer(&self) -> &::std::option::Option<crate::types::NewBgpPeer> {
        self.inner.get_new_bgp_peer()
    }
}
