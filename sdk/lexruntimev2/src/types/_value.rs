// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The value of a slot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Value {
    /// <p>The text of the utterance from the user that was entered for the slot.</p>
    pub original_value: ::std::option::Option<::std::string::String>,
    /// <p>The value that Amazon Lex V2 determines for the slot. The actual value depends on the setting of the value selection strategy for the bot. You can choose to use the value entered by the user, or you can have Amazon Lex V2 choose the first value in the <code>resolvedValues</code> list.</p>
    pub interpreted_value: ::std::string::String,
    /// <p>A list of additional values that have been recognized for the slot.</p>
    pub resolved_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl Value {
    /// <p>The text of the utterance from the user that was entered for the slot.</p>
    pub fn original_value(&self) -> ::std::option::Option<&str> {
        self.original_value.as_deref()
    }
    /// <p>The value that Amazon Lex V2 determines for the slot. The actual value depends on the setting of the value selection strategy for the bot. You can choose to use the value entered by the user, or you can have Amazon Lex V2 choose the first value in the <code>resolvedValues</code> list.</p>
    pub fn interpreted_value(&self) -> &str {
        use std::ops::Deref;
        self.interpreted_value.deref()
    }
    /// <p>A list of additional values that have been recognized for the slot.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resolved_values.is_none()`.
    pub fn resolved_values(&self) -> &[::std::string::String] {
        self.resolved_values.as_deref().unwrap_or_default()
    }
}
impl Value {
    /// Creates a new builder-style object to manufacture [`Value`](crate::types::Value).
    pub fn builder() -> crate::types::builders::ValueBuilder {
        crate::types::builders::ValueBuilder::default()
    }
}

/// A builder for [`Value`](crate::types::Value).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ValueBuilder {
    pub(crate) original_value: ::std::option::Option<::std::string::String>,
    pub(crate) interpreted_value: ::std::option::Option<::std::string::String>,
    pub(crate) resolved_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ValueBuilder {
    /// <p>The text of the utterance from the user that was entered for the slot.</p>
    pub fn original_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.original_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The text of the utterance from the user that was entered for the slot.</p>
    pub fn set_original_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.original_value = input;
        self
    }
    /// <p>The text of the utterance from the user that was entered for the slot.</p>
    pub fn get_original_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.original_value
    }
    /// <p>The value that Amazon Lex V2 determines for the slot. The actual value depends on the setting of the value selection strategy for the bot. You can choose to use the value entered by the user, or you can have Amazon Lex V2 choose the first value in the <code>resolvedValues</code> list.</p>
    /// This field is required.
    pub fn interpreted_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.interpreted_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value that Amazon Lex V2 determines for the slot. The actual value depends on the setting of the value selection strategy for the bot. You can choose to use the value entered by the user, or you can have Amazon Lex V2 choose the first value in the <code>resolvedValues</code> list.</p>
    pub fn set_interpreted_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.interpreted_value = input;
        self
    }
    /// <p>The value that Amazon Lex V2 determines for the slot. The actual value depends on the setting of the value selection strategy for the bot. You can choose to use the value entered by the user, or you can have Amazon Lex V2 choose the first value in the <code>resolvedValues</code> list.</p>
    pub fn get_interpreted_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.interpreted_value
    }
    /// Appends an item to `resolved_values`.
    ///
    /// To override the contents of this collection use [`set_resolved_values`](Self::set_resolved_values).
    ///
    /// <p>A list of additional values that have been recognized for the slot.</p>
    pub fn resolved_values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resolved_values.unwrap_or_default();
        v.push(input.into());
        self.resolved_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of additional values that have been recognized for the slot.</p>
    pub fn set_resolved_values(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.resolved_values = input;
        self
    }
    /// <p>A list of additional values that have been recognized for the slot.</p>
    pub fn get_resolved_values(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.resolved_values
    }
    /// Consumes the builder and constructs a [`Value`](crate::types::Value).
    /// This method will fail if any of the following fields are not set:
    /// - [`interpreted_value`](crate::types::builders::ValueBuilder::interpreted_value)
    pub fn build(self) -> ::std::result::Result<crate::types::Value, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Value {
            original_value: self.original_value,
            interpreted_value: self.interpreted_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "interpreted_value",
                    "interpreted_value was not specified but it is required when building Value",
                )
            })?,
            resolved_values: self.resolved_values,
        })
    }
}
