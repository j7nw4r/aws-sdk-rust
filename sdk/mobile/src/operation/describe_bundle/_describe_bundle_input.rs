// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Request structure to request the details of a specific bundle. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeBundleInput {
    /// <p> Unique bundle identifier. </p>
    pub bundle_id: ::std::option::Option<::std::string::String>,
}
impl DescribeBundleInput {
    /// <p> Unique bundle identifier. </p>
    pub fn bundle_id(&self) -> ::std::option::Option<&str> {
        self.bundle_id.as_deref()
    }
}
impl DescribeBundleInput {
    /// Creates a new builder-style object to manufacture [`DescribeBundleInput`](crate::operation::describe_bundle::DescribeBundleInput).
    pub fn builder() -> crate::operation::describe_bundle::builders::DescribeBundleInputBuilder {
        crate::operation::describe_bundle::builders::DescribeBundleInputBuilder::default()
    }
}

/// A builder for [`DescribeBundleInput`](crate::operation::describe_bundle::DescribeBundleInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeBundleInputBuilder {
    pub(crate) bundle_id: ::std::option::Option<::std::string::String>,
}
impl DescribeBundleInputBuilder {
    /// <p> Unique bundle identifier. </p>
    /// This field is required.
    pub fn bundle_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bundle_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Unique bundle identifier. </p>
    pub fn set_bundle_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bundle_id = input;
        self
    }
    /// <p> Unique bundle identifier. </p>
    pub fn get_bundle_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.bundle_id
    }
    /// Consumes the builder and constructs a [`DescribeBundleInput`](crate::operation::describe_bundle::DescribeBundleInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_bundle::DescribeBundleInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_bundle::DescribeBundleInput { bundle_id: self.bundle_id })
    }
}
