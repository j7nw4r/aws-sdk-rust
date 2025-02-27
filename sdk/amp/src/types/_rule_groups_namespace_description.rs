// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents a description of the rule groups namespace.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RuleGroupsNamespaceDescription {
    /// The Amazon Resource Name (ARN) of this rule groups namespace.
    pub arn: ::std::string::String,
    /// The rule groups namespace name.
    pub name: ::std::string::String,
    /// The status of rule groups namespace.
    pub status: ::std::option::Option<crate::types::RuleGroupsNamespaceStatus>,
    /// The rule groups namespace data.
    pub data: ::aws_smithy_types::Blob,
    /// The time when the rule groups namespace was created.
    pub created_at: ::aws_smithy_types::DateTime,
    /// The time when the rule groups namespace was modified.
    pub modified_at: ::aws_smithy_types::DateTime,
    /// The tags of this rule groups namespace.
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl RuleGroupsNamespaceDescription {
    /// The Amazon Resource Name (ARN) of this rule groups namespace.
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// The rule groups namespace name.
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// The status of rule groups namespace.
    pub fn status(&self) -> ::std::option::Option<&crate::types::RuleGroupsNamespaceStatus> {
        self.status.as_ref()
    }
    /// The rule groups namespace data.
    pub fn data(&self) -> &::aws_smithy_types::Blob {
        &self.data
    }
    /// The time when the rule groups namespace was created.
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// The time when the rule groups namespace was modified.
    pub fn modified_at(&self) -> &::aws_smithy_types::DateTime {
        &self.modified_at
    }
    /// The tags of this rule groups namespace.
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl RuleGroupsNamespaceDescription {
    /// Creates a new builder-style object to manufacture [`RuleGroupsNamespaceDescription`](crate::types::RuleGroupsNamespaceDescription).
    pub fn builder() -> crate::types::builders::RuleGroupsNamespaceDescriptionBuilder {
        crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::default()
    }
}

/// A builder for [`RuleGroupsNamespaceDescription`](crate::types::RuleGroupsNamespaceDescription).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RuleGroupsNamespaceDescriptionBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::RuleGroupsNamespaceStatus>,
    pub(crate) data: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl RuleGroupsNamespaceDescriptionBuilder {
    /// The Amazon Resource Name (ARN) of this rule groups namespace.
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The Amazon Resource Name (ARN) of this rule groups namespace.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// The Amazon Resource Name (ARN) of this rule groups namespace.
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// The rule groups namespace name.
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// The rule groups namespace name.
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// The rule groups namespace name.
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// The status of rule groups namespace.
    /// This field is required.
    pub fn status(mut self, input: crate::types::RuleGroupsNamespaceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// The status of rule groups namespace.
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::RuleGroupsNamespaceStatus>) -> Self {
        self.status = input;
        self
    }
    /// The status of rule groups namespace.
    pub fn get_status(&self) -> &::std::option::Option<crate::types::RuleGroupsNamespaceStatus> {
        &self.status
    }
    /// The rule groups namespace data.
    /// This field is required.
    pub fn data(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.data = ::std::option::Option::Some(input);
        self
    }
    /// The rule groups namespace data.
    pub fn set_data(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.data = input;
        self
    }
    /// The rule groups namespace data.
    pub fn get_data(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.data
    }
    /// The time when the rule groups namespace was created.
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// The time when the rule groups namespace was created.
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// The time when the rule groups namespace was created.
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// The time when the rule groups namespace was modified.
    /// This field is required.
    pub fn modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.modified_at = ::std::option::Option::Some(input);
        self
    }
    /// The time when the rule groups namespace was modified.
    pub fn set_modified_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.modified_at = input;
        self
    }
    /// The time when the rule groups namespace was modified.
    pub fn get_modified_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.modified_at
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// The tags of this rule groups namespace.
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// The tags of this rule groups namespace.
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// The tags of this rule groups namespace.
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`RuleGroupsNamespaceDescription`](crate::types::RuleGroupsNamespaceDescription).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::arn)
    /// - [`name`](crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::name)
    /// - [`data`](crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::data)
    /// - [`created_at`](crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::created_at)
    /// - [`modified_at`](crate::types::builders::RuleGroupsNamespaceDescriptionBuilder::modified_at)
    pub fn build(self) -> ::std::result::Result<crate::types::RuleGroupsNamespaceDescription, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::RuleGroupsNamespaceDescription {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building RuleGroupsNamespaceDescription",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building RuleGroupsNamespaceDescription",
                )
            })?,
            status: self.status,
            data: self.data.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data",
                    "data was not specified but it is required when building RuleGroupsNamespaceDescription",
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building RuleGroupsNamespaceDescription",
                )
            })?,
            modified_at: self.modified_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "modified_at",
                    "modified_at was not specified but it is required when building RuleGroupsNamespaceDescription",
                )
            })?,
            tags: self.tags,
        })
    }
}
