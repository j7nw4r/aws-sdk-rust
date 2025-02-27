// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details on the configuration of a table optimizer. You pass this configuration when creating or updating a table optimizer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TableOptimizerConfiguration {
    /// <p>A role passed by the caller which gives the service permission to update the resources associated with the optimizer on the caller's behalf.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Whether table optimization is enabled. </p>
    pub enabled: ::std::option::Option<bool>,
}
impl TableOptimizerConfiguration {
    /// <p>A role passed by the caller which gives the service permission to update the resources associated with the optimizer on the caller's behalf.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Whether table optimization is enabled. </p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl TableOptimizerConfiguration {
    /// Creates a new builder-style object to manufacture [`TableOptimizerConfiguration`](crate::types::TableOptimizerConfiguration).
    pub fn builder() -> crate::types::builders::TableOptimizerConfigurationBuilder {
        crate::types::builders::TableOptimizerConfigurationBuilder::default()
    }
}

/// A builder for [`TableOptimizerConfiguration`](crate::types::TableOptimizerConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TableOptimizerConfigurationBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl TableOptimizerConfigurationBuilder {
    /// <p>A role passed by the caller which gives the service permission to update the resources associated with the optimizer on the caller's behalf.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A role passed by the caller which gives the service permission to update the resources associated with the optimizer on the caller's behalf.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>A role passed by the caller which gives the service permission to update the resources associated with the optimizer on the caller's behalf.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>Whether table optimization is enabled. </p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether table optimization is enabled. </p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Whether table optimization is enabled. </p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`TableOptimizerConfiguration`](crate::types::TableOptimizerConfiguration).
    pub fn build(self) -> crate::types::TableOptimizerConfiguration {
        crate::types::TableOptimizerConfiguration {
            role_arn: self.role_arn,
            enabled: self.enabled,
        }
    }
}
