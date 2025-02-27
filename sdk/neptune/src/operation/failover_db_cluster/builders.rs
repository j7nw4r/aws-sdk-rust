// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::failover_db_cluster::_failover_db_cluster_output::FailoverDbClusterOutputBuilder;

pub use crate::operation::failover_db_cluster::_failover_db_cluster_input::FailoverDbClusterInputBuilder;

impl FailoverDbClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::failover_db_cluster::FailoverDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::failover_db_cluster::FailoverDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.failover_db_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `FailoverDBCluster`.
///
/// <p>Forces a failover for a DB cluster.</p>
/// <p>A failover for a DB cluster promotes one of the Read Replicas (read-only instances) in the DB cluster to be the primary instance (the cluster writer).</p>
/// <p>Amazon Neptune will automatically fail over to a Read Replica, if one exists, when the primary instance fails. You can force a failover when you want to simulate a failure of a primary instance for testing. Because each instance in a DB cluster has its own endpoint address, you will need to clean up and re-establish any existing connections that use those endpoint addresses when the failover is complete.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct FailoverDBClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::failover_db_cluster::builders::FailoverDbClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::failover_db_cluster::FailoverDbClusterOutput,
        crate::operation::failover_db_cluster::FailoverDBClusterError,
    > for FailoverDBClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::failover_db_cluster::FailoverDbClusterOutput,
            crate::operation::failover_db_cluster::FailoverDBClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl FailoverDBClusterFluentBuilder {
    /// Creates a new `FailoverDBCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the FailoverDBCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::failover_db_cluster::builders::FailoverDbClusterInputBuilder {
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
        crate::operation::failover_db_cluster::FailoverDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::failover_db_cluster::FailoverDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::failover_db_cluster::FailoverDBCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::failover_db_cluster::FailoverDBCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::failover_db_cluster::FailoverDbClusterOutput,
        crate::operation::failover_db_cluster::FailoverDBClusterError,
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
    /// <p>A DB cluster identifier to force a failover for. This parameter is not case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DBCluster.</p> </li>
    /// </ul>
    pub fn db_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_identifier(input.into());
        self
    }
    /// <p>A DB cluster identifier to force a failover for. This parameter is not case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DBCluster.</p> </li>
    /// </ul>
    pub fn set_db_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_cluster_identifier(input);
        self
    }
    /// <p>A DB cluster identifier to force a failover for. This parameter is not case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DBCluster.</p> </li>
    /// </ul>
    pub fn get_db_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_cluster_identifier()
    }
    /// <p>The name of the instance to promote to the primary instance.</p>
    /// <p>You must specify the instance identifier for an Read Replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
    pub fn target_db_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_db_instance_identifier(input.into());
        self
    }
    /// <p>The name of the instance to promote to the primary instance.</p>
    /// <p>You must specify the instance identifier for an Read Replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
    pub fn set_target_db_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_db_instance_identifier(input);
        self
    }
    /// <p>The name of the instance to promote to the primary instance.</p>
    /// <p>You must specify the instance identifier for an Read Replica in the DB cluster. For example, <code>mydbcluster-replica1</code>.</p>
    pub fn get_target_db_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_db_instance_identifier()
    }
}
