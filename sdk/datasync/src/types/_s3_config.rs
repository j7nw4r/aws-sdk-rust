// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role used to access an Amazon S3 bucket.</p>
/// <p>For detailed information about using such a role, see Creating a Location for Amazon S3 in the <i>DataSync User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3Config {
    /// <p>The ARN of the IAM role for accessing the S3 bucket. </p>
    pub bucket_access_role_arn: ::std::string::String,
}
impl S3Config {
    /// <p>The ARN of the IAM role for accessing the S3 bucket. </p>
    pub fn bucket_access_role_arn(&self) -> &str {
        use std::ops::Deref;
        self.bucket_access_role_arn.deref()
    }
}
impl S3Config {
    /// Creates a new builder-style object to manufacture [`S3Config`](crate::types::S3Config).
    pub fn builder() -> crate::types::builders::S3ConfigBuilder {
        crate::types::builders::S3ConfigBuilder::default()
    }
}

/// A builder for [`S3Config`](crate::types::S3Config).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct S3ConfigBuilder {
    pub(crate) bucket_access_role_arn: ::std::option::Option<::std::string::String>,
}
impl S3ConfigBuilder {
    /// <p>The ARN of the IAM role for accessing the S3 bucket. </p>
    /// This field is required.
    pub fn bucket_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_access_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the IAM role for accessing the S3 bucket. </p>
    pub fn set_bucket_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_access_role_arn = input;
        self
    }
    /// <p>The ARN of the IAM role for accessing the S3 bucket. </p>
    pub fn get_bucket_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket_access_role_arn
    }
    /// Consumes the builder and constructs a [`S3Config`](crate::types::S3Config).
    /// This method will fail if any of the following fields are not set:
    /// - [`bucket_access_role_arn`](crate::types::builders::S3ConfigBuilder::bucket_access_role_arn)
    pub fn build(self) -> ::std::result::Result<crate::types::S3Config, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::S3Config {
            bucket_access_role_arn: self.bucket_access_role_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "bucket_access_role_arn",
                    "bucket_access_role_arn was not specified but it is required when building S3Config",
                )
            })?,
        })
    }
}
