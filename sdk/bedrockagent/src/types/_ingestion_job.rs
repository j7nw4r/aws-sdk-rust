// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Contains the information of an ingestion job.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IngestionJob {
    /// Identifier for a resource.
    pub knowledge_base_id: ::std::string::String,
    /// Identifier for a resource.
    pub data_source_id: ::std::string::String,
    /// Identifier for a resource.
    pub ingestion_job_id: ::std::string::String,
    /// Description of the Resource.
    pub description: ::std::option::Option<::std::string::String>,
    /// The status of an ingestion job.
    pub status: crate::types::IngestionJobStatus,
    /// The document level statistics of an ingestion job
    pub statistics: ::std::option::Option<crate::types::IngestionJobStatistics>,
    /// Failure Reasons for Error.
    pub failure_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// Time Stamp.
    pub started_at: ::aws_smithy_types::DateTime,
    /// Time Stamp.
    pub updated_at: ::aws_smithy_types::DateTime,
}
impl IngestionJob {
    /// Identifier for a resource.
    pub fn knowledge_base_id(&self) -> &str {
        use std::ops::Deref;
        self.knowledge_base_id.deref()
    }
    /// Identifier for a resource.
    pub fn data_source_id(&self) -> &str {
        use std::ops::Deref;
        self.data_source_id.deref()
    }
    /// Identifier for a resource.
    pub fn ingestion_job_id(&self) -> &str {
        use std::ops::Deref;
        self.ingestion_job_id.deref()
    }
    /// Description of the Resource.
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// The status of an ingestion job.
    pub fn status(&self) -> &crate::types::IngestionJobStatus {
        &self.status
    }
    /// The document level statistics of an ingestion job
    pub fn statistics(&self) -> ::std::option::Option<&crate::types::IngestionJobStatistics> {
        self.statistics.as_ref()
    }
    /// Failure Reasons for Error.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.failure_reasons.is_none()`.
    pub fn failure_reasons(&self) -> &[::std::string::String] {
        self.failure_reasons.as_deref().unwrap_or_default()
    }
    /// Time Stamp.
    pub fn started_at(&self) -> &::aws_smithy_types::DateTime {
        &self.started_at
    }
    /// Time Stamp.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
}
impl IngestionJob {
    /// Creates a new builder-style object to manufacture [`IngestionJob`](crate::types::IngestionJob).
    pub fn builder() -> crate::types::builders::IngestionJobBuilder {
        crate::types::builders::IngestionJobBuilder::default()
    }
}

/// A builder for [`IngestionJob`](crate::types::IngestionJob).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IngestionJobBuilder {
    pub(crate) knowledge_base_id: ::std::option::Option<::std::string::String>,
    pub(crate) data_source_id: ::std::option::Option<::std::string::String>,
    pub(crate) ingestion_job_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::IngestionJobStatus>,
    pub(crate) statistics: ::std::option::Option<crate::types::IngestionJobStatistics>,
    pub(crate) failure_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) started_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl IngestionJobBuilder {
    /// Identifier for a resource.
    /// This field is required.
    pub fn knowledge_base_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.knowledge_base_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_knowledge_base_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.knowledge_base_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_knowledge_base_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.knowledge_base_id
    }
    /// Identifier for a resource.
    /// This field is required.
    pub fn data_source_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.data_source_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_data_source_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.data_source_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_data_source_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.data_source_id
    }
    /// Identifier for a resource.
    /// This field is required.
    pub fn ingestion_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ingestion_job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier for a resource.
    pub fn set_ingestion_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ingestion_job_id = input;
        self
    }
    /// Identifier for a resource.
    pub fn get_ingestion_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ingestion_job_id
    }
    /// Description of the Resource.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// Description of the Resource.
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Description of the Resource.
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// The status of an ingestion job.
    /// This field is required.
    pub fn status(mut self, input: crate::types::IngestionJobStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// The status of an ingestion job.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::IngestionJobStatus>) -> Self {
        self.status = input;
        self
    }
    /// The status of an ingestion job.
    pub fn get_status(&self) -> &::std::option::Option<crate::types::IngestionJobStatus> {
        &self.status
    }
    /// The document level statistics of an ingestion job
    pub fn statistics(mut self, input: crate::types::IngestionJobStatistics) -> Self {
        self.statistics = ::std::option::Option::Some(input);
        self
    }
    /// The document level statistics of an ingestion job
    pub fn set_statistics(mut self, input: ::std::option::Option<crate::types::IngestionJobStatistics>) -> Self {
        self.statistics = input;
        self
    }
    /// The document level statistics of an ingestion job
    pub fn get_statistics(&self) -> &::std::option::Option<crate::types::IngestionJobStatistics> {
        &self.statistics
    }
    /// Appends an item to `failure_reasons`.
    ///
    /// To override the contents of this collection use [`set_failure_reasons`](Self::set_failure_reasons).
    ///
    /// Failure Reasons for Error.
    pub fn failure_reasons(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.failure_reasons.unwrap_or_default();
        v.push(input.into());
        self.failure_reasons = ::std::option::Option::Some(v);
        self
    }
    /// Failure Reasons for Error.
    pub fn set_failure_reasons(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.failure_reasons = input;
        self
    }
    /// Failure Reasons for Error.
    pub fn get_failure_reasons(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.failure_reasons
    }
    /// Time Stamp.
    /// This field is required.
    pub fn started_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.started_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_started_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.started_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_started_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.started_at
    }
    /// Time Stamp.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Time Stamp.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Time Stamp.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// Consumes the builder and constructs a [`IngestionJob`](crate::types::IngestionJob).
    /// This method will fail if any of the following fields are not set:
    /// - [`knowledge_base_id`](crate::types::builders::IngestionJobBuilder::knowledge_base_id)
    /// - [`data_source_id`](crate::types::builders::IngestionJobBuilder::data_source_id)
    /// - [`ingestion_job_id`](crate::types::builders::IngestionJobBuilder::ingestion_job_id)
    /// - [`status`](crate::types::builders::IngestionJobBuilder::status)
    /// - [`started_at`](crate::types::builders::IngestionJobBuilder::started_at)
    /// - [`updated_at`](crate::types::builders::IngestionJobBuilder::updated_at)
    pub fn build(self) -> ::std::result::Result<crate::types::IngestionJob, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::IngestionJob {
            knowledge_base_id: self.knowledge_base_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "knowledge_base_id",
                    "knowledge_base_id was not specified but it is required when building IngestionJob",
                )
            })?,
            data_source_id: self.data_source_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data_source_id",
                    "data_source_id was not specified but it is required when building IngestionJob",
                )
            })?,
            ingestion_job_id: self.ingestion_job_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "ingestion_job_id",
                    "ingestion_job_id was not specified but it is required when building IngestionJob",
                )
            })?,
            description: self.description,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building IngestionJob",
                )
            })?,
            statistics: self.statistics,
            failure_reasons: self.failure_reasons,
            started_at: self.started_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "started_at",
                    "started_at was not specified but it is required when building IngestionJob",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building IngestionJob",
                )
            })?,
        })
    }
}
