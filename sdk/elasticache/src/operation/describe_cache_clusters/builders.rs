// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_cache_clusters::_describe_cache_clusters_output::DescribeCacheClustersOutputBuilder;

pub use crate::operation::describe_cache_clusters::_describe_cache_clusters_input::DescribeCacheClustersInputBuilder;

impl DescribeCacheClustersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_cache_clusters::DescribeCacheClustersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_cache_clusters::DescribeCacheClustersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_cache_clusters();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeCacheClusters`.
///
/// <p>Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cache cluster if a cluster identifier is supplied.</p>
/// <p>By default, abbreviated information about the clusters is returned. You can use the optional <i>ShowCacheNodeInfo</i> flag to retrieve detailed information about the cache nodes associated with the clusters. These details include the DNS address and port for the cache node endpoint.</p>
/// <p>If the cluster is in the <i>creating</i> state, only cluster-level information is displayed until all of the nodes are successfully provisioned.</p>
/// <p>If the cluster is in the <i>deleting</i> state, only cluster-level information is displayed.</p>
/// <p>If cache nodes are currently being added to the cluster, node endpoint information and creation time for the additional nodes are not displayed until they are completely provisioned. When the cluster state is <i>available</i>, the cluster is ready for use.</p>
/// <p>If cache nodes are currently being removed from the cluster, no endpoint information for the removed nodes is displayed.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeCacheClustersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_cache_clusters::builders::DescribeCacheClustersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_cache_clusters::DescribeCacheClustersOutput,
        crate::operation::describe_cache_clusters::DescribeCacheClustersError,
    > for DescribeCacheClustersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_cache_clusters::DescribeCacheClustersOutput,
            crate::operation::describe_cache_clusters::DescribeCacheClustersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeCacheClustersFluentBuilder {
    /// Creates a new `DescribeCacheClusters`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeCacheClusters as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_cache_clusters::builders::DescribeCacheClustersInputBuilder {
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
        crate::operation::describe_cache_clusters::DescribeCacheClustersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_cache_clusters::DescribeCacheClustersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_cache_clusters::DescribeCacheClusters::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_cache_clusters::DescribeCacheClusters::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_cache_clusters::DescribeCacheClustersOutput,
        crate::operation::describe_cache_clusters::DescribeCacheClustersError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_cache_clusters::paginator::DescribeCacheClustersPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_cache_clusters::paginator::DescribeCacheClustersPaginator {
        crate::operation::describe_cache_clusters::paginator::DescribeCacheClustersPaginator::new(self.handle, self.inner)
    }
    /// <p>The user-supplied cluster identifier. If this parameter is specified, only information about that specific cluster is returned. This parameter isn't case sensitive.</p>
    pub fn cache_cluster_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cache_cluster_id(input.into());
        self
    }
    /// <p>The user-supplied cluster identifier. If this parameter is specified, only information about that specific cluster is returned. This parameter isn't case sensitive.</p>
    pub fn set_cache_cluster_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cache_cluster_id(input);
        self
    }
    /// <p>The user-supplied cluster identifier. If this parameter is specified, only information about that specific cluster is returned. This parameter isn't case sensitive.</p>
    pub fn get_cache_cluster_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cache_cluster_id()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: minimum 20; maximum 100.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to retrieve information about the individual cache nodes.</p>
    pub fn show_cache_node_info(mut self, input: bool) -> Self {
        self.inner = self.inner.show_cache_node_info(input);
        self
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to retrieve information about the individual cache nodes.</p>
    pub fn set_show_cache_node_info(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_show_cache_node_info(input);
        self
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to retrieve information about the individual cache nodes.</p>
    pub fn get_show_cache_node_info(&self) -> &::std::option::Option<bool> {
        self.inner.get_show_cache_node_info()
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to show only nodes (API/CLI: clusters) that are not members of a replication group. In practice, this mean Memcached and single node Redis clusters.</p>
    pub fn show_cache_clusters_not_in_replication_groups(mut self, input: bool) -> Self {
        self.inner = self.inner.show_cache_clusters_not_in_replication_groups(input);
        self
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to show only nodes (API/CLI: clusters) that are not members of a replication group. In practice, this mean Memcached and single node Redis clusters.</p>
    pub fn set_show_cache_clusters_not_in_replication_groups(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_show_cache_clusters_not_in_replication_groups(input);
        self
    }
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to show only nodes (API/CLI: clusters) that are not members of a replication group. In practice, this mean Memcached and single node Redis clusters.</p>
    pub fn get_show_cache_clusters_not_in_replication_groups(&self) -> &::std::option::Option<bool> {
        self.inner.get_show_cache_clusters_not_in_replication_groups()
    }
}
