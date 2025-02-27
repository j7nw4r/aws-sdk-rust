// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details for a transformer object. Transformers describe how to process the incoming EDI (electronic data interchange) documents, and extract the necessary information.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransformerSummary {
    /// <p>Returns the system-assigned unique identifier for the transformer.</p>
    pub transformer_id: ::std::string::String,
    /// <p>Returns the descriptive name for the transformer.</p>
    pub name: ::std::string::String,
    /// <p>Returns that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub file_format: crate::types::FileFormat,
    /// <p>Returns the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub mapping_template: ::std::string::String,
    /// <p>Returns the state of the newly created transformer. The transformer can be either <code>active</code> or <code>inactive</code>. For the transformer to be used in a capability, its status must <code>active</code>.</p>
    pub status: crate::types::TransformerStatus,
    /// <p>Returns the details for the EDI standard that is being used for the transformer. Currently, only X12 is supported. X12 is a set of standards and corresponding messages that define specific business documents.</p>
    pub edi_type: ::std::option::Option<crate::types::EdiType>,
    /// <p>Returns a sample EDI document that is used by a transformer as a guide for processing the EDI data.</p>
    pub sample_document: ::std::option::Option<::std::string::String>,
    /// <p>Returns a timestamp indicating when the transformer was created. For example, <code>2023-07-20T19:58:44.624Z</code>.</p>
    pub created_at: ::aws_smithy_types::DateTime,
    /// <p>Returns a timestamp representing the date and time for the most recent change for the transformer object.</p>
    pub modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl TransformerSummary {
    /// <p>Returns the system-assigned unique identifier for the transformer.</p>
    pub fn transformer_id(&self) -> &str {
        use std::ops::Deref;
        self.transformer_id.deref()
    }
    /// <p>Returns the descriptive name for the transformer.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>Returns that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn file_format(&self) -> &crate::types::FileFormat {
        &self.file_format
    }
    /// <p>Returns the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn mapping_template(&self) -> &str {
        use std::ops::Deref;
        self.mapping_template.deref()
    }
    /// <p>Returns the state of the newly created transformer. The transformer can be either <code>active</code> or <code>inactive</code>. For the transformer to be used in a capability, its status must <code>active</code>.</p>
    pub fn status(&self) -> &crate::types::TransformerStatus {
        &self.status
    }
    /// <p>Returns the details for the EDI standard that is being used for the transformer. Currently, only X12 is supported. X12 is a set of standards and corresponding messages that define specific business documents.</p>
    pub fn edi_type(&self) -> ::std::option::Option<&crate::types::EdiType> {
        self.edi_type.as_ref()
    }
    /// <p>Returns a sample EDI document that is used by a transformer as a guide for processing the EDI data.</p>
    pub fn sample_document(&self) -> ::std::option::Option<&str> {
        self.sample_document.as_deref()
    }
    /// <p>Returns a timestamp indicating when the transformer was created. For example, <code>2023-07-20T19:58:44.624Z</code>.</p>
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// <p>Returns a timestamp representing the date and time for the most recent change for the transformer object.</p>
    pub fn modified_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.modified_at.as_ref()
    }
}
impl TransformerSummary {
    /// Creates a new builder-style object to manufacture [`TransformerSummary`](crate::types::TransformerSummary).
    pub fn builder() -> crate::types::builders::TransformerSummaryBuilder {
        crate::types::builders::TransformerSummaryBuilder::default()
    }
}

/// A builder for [`TransformerSummary`](crate::types::TransformerSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TransformerSummaryBuilder {
    pub(crate) transformer_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) file_format: ::std::option::Option<crate::types::FileFormat>,
    pub(crate) mapping_template: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::TransformerStatus>,
    pub(crate) edi_type: ::std::option::Option<crate::types::EdiType>,
    pub(crate) sample_document: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl TransformerSummaryBuilder {
    /// <p>Returns the system-assigned unique identifier for the transformer.</p>
    /// This field is required.
    pub fn transformer_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transformer_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the system-assigned unique identifier for the transformer.</p>
    pub fn set_transformer_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transformer_id = input;
        self
    }
    /// <p>Returns the system-assigned unique identifier for the transformer.</p>
    pub fn get_transformer_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transformer_id
    }
    /// <p>Returns the descriptive name for the transformer.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the descriptive name for the transformer.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Returns the descriptive name for the transformer.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>Returns that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    /// This field is required.
    pub fn file_format(mut self, input: crate::types::FileFormat) -> Self {
        self.file_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn set_file_format(mut self, input: ::std::option::Option<crate::types::FileFormat>) -> Self {
        self.file_format = input;
        self
    }
    /// <p>Returns that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn get_file_format(&self) -> &::std::option::Option<crate::types::FileFormat> {
        &self.file_format
    }
    /// <p>Returns the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    /// This field is required.
    pub fn mapping_template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mapping_template = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn set_mapping_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mapping_template = input;
        self
    }
    /// <p>Returns the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn get_mapping_template(&self) -> &::std::option::Option<::std::string::String> {
        &self.mapping_template
    }
    /// <p>Returns the state of the newly created transformer. The transformer can be either <code>active</code> or <code>inactive</code>. For the transformer to be used in a capability, its status must <code>active</code>.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::TransformerStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the state of the newly created transformer. The transformer can be either <code>active</code> or <code>inactive</code>. For the transformer to be used in a capability, its status must <code>active</code>.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::TransformerStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Returns the state of the newly created transformer. The transformer can be either <code>active</code> or <code>inactive</code>. For the transformer to be used in a capability, its status must <code>active</code>.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::TransformerStatus> {
        &self.status
    }
    /// <p>Returns the details for the EDI standard that is being used for the transformer. Currently, only X12 is supported. X12 is a set of standards and corresponding messages that define specific business documents.</p>
    /// This field is required.
    pub fn edi_type(mut self, input: crate::types::EdiType) -> Self {
        self.edi_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the details for the EDI standard that is being used for the transformer. Currently, only X12 is supported. X12 is a set of standards and corresponding messages that define specific business documents.</p>
    pub fn set_edi_type(mut self, input: ::std::option::Option<crate::types::EdiType>) -> Self {
        self.edi_type = input;
        self
    }
    /// <p>Returns the details for the EDI standard that is being used for the transformer. Currently, only X12 is supported. X12 is a set of standards and corresponding messages that define specific business documents.</p>
    pub fn get_edi_type(&self) -> &::std::option::Option<crate::types::EdiType> {
        &self.edi_type
    }
    /// <p>Returns a sample EDI document that is used by a transformer as a guide for processing the EDI data.</p>
    pub fn sample_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sample_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns a sample EDI document that is used by a transformer as a guide for processing the EDI data.</p>
    pub fn set_sample_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sample_document = input;
        self
    }
    /// <p>Returns a sample EDI document that is used by a transformer as a guide for processing the EDI data.</p>
    pub fn get_sample_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.sample_document
    }
    /// <p>Returns a timestamp indicating when the transformer was created. For example, <code>2023-07-20T19:58:44.624Z</code>.</p>
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns a timestamp indicating when the transformer was created. For example, <code>2023-07-20T19:58:44.624Z</code>.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>Returns a timestamp indicating when the transformer was created. For example, <code>2023-07-20T19:58:44.624Z</code>.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>Returns a timestamp representing the date and time for the most recent change for the transformer object.</p>
    pub fn modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.modified_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns a timestamp representing the date and time for the most recent change for the transformer object.</p>
    pub fn set_modified_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.modified_at = input;
        self
    }
    /// <p>Returns a timestamp representing the date and time for the most recent change for the transformer object.</p>
    pub fn get_modified_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.modified_at
    }
    /// Consumes the builder and constructs a [`TransformerSummary`](crate::types::TransformerSummary).
    /// This method will fail if any of the following fields are not set:
    /// - [`transformer_id`](crate::types::builders::TransformerSummaryBuilder::transformer_id)
    /// - [`name`](crate::types::builders::TransformerSummaryBuilder::name)
    /// - [`file_format`](crate::types::builders::TransformerSummaryBuilder::file_format)
    /// - [`mapping_template`](crate::types::builders::TransformerSummaryBuilder::mapping_template)
    /// - [`status`](crate::types::builders::TransformerSummaryBuilder::status)
    /// - [`created_at`](crate::types::builders::TransformerSummaryBuilder::created_at)
    pub fn build(self) -> ::std::result::Result<crate::types::TransformerSummary, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TransformerSummary {
            transformer_id: self.transformer_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "transformer_id",
                    "transformer_id was not specified but it is required when building TransformerSummary",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building TransformerSummary",
                )
            })?,
            file_format: self.file_format.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "file_format",
                    "file_format was not specified but it is required when building TransformerSummary",
                )
            })?,
            mapping_template: self.mapping_template.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "mapping_template",
                    "mapping_template was not specified but it is required when building TransformerSummary",
                )
            })?,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building TransformerSummary",
                )
            })?,
            edi_type: self.edi_type,
            sample_document: self.sample_document,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building TransformerSummary",
                )
            })?,
            modified_at: self.modified_at,
        })
    }
}
