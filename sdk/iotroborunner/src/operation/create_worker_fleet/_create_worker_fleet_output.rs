// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateWorkerFleetOutput {
    /// Full ARN of the worker fleet.
    pub arn: ::std::string::String,
    /// Filters access by the worker fleet's identifier
    pub id: ::std::string::String,
    /// Timestamp at which the resource was created.
    pub created_at: ::aws_smithy_types::DateTime,
    /// Timestamp at which the resource was last updated.
    pub updated_at: ::aws_smithy_types::DateTime,
    _request_id: Option<String>,
}
impl CreateWorkerFleetOutput {
    /// Full ARN of the worker fleet.
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// Filters access by the worker fleet's identifier
    pub fn id(&self) -> &str {
        use std::ops::Deref;
        self.id.deref()
    }
    /// Timestamp at which the resource was created.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// Timestamp at which the resource was last updated.
    pub fn updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.updated_at
    }
}
impl ::aws_types::request_id::RequestId for CreateWorkerFleetOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateWorkerFleetOutput {
    /// Creates a new builder-style object to manufacture [`CreateWorkerFleetOutput`](crate::operation::create_worker_fleet::CreateWorkerFleetOutput).
    pub fn builder() -> crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder {
        crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder::default()
    }
}

/// A builder for [`CreateWorkerFleetOutput`](crate::operation::create_worker_fleet::CreateWorkerFleetOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateWorkerFleetOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl CreateWorkerFleetOutputBuilder {
    /// Full ARN of the worker fleet.
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// Full ARN of the worker fleet.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// Full ARN of the worker fleet.
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// Filters access by the worker fleet's identifier
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Filters access by the worker fleet's identifier
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Filters access by the worker fleet's identifier
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Timestamp at which the resource was created.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// Timestamp at which the resource was created.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// Timestamp at which the resource was created.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// Timestamp at which the resource was last updated.
    /// This field is required.
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// Timestamp at which the resource was last updated.
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateWorkerFleetOutput`](crate::operation::create_worker_fleet::CreateWorkerFleetOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder::arn)
    /// - [`id`](crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder::id)
    /// - [`created_at`](crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder::created_at)
    /// - [`updated_at`](crate::operation::create_worker_fleet::builders::CreateWorkerFleetOutputBuilder::updated_at)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_worker_fleet::CreateWorkerFleetOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_worker_fleet::CreateWorkerFleetOutput {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building CreateWorkerFleetOutput",
                )
            })?,
            id: self.id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "id",
                    "id was not specified but it is required when building CreateWorkerFleetOutput",
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building CreateWorkerFleetOutput",
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "updated_at",
                    "updated_at was not specified but it is required when building CreateWorkerFleetOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
