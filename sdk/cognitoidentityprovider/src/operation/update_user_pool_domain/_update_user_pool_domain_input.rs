// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The UpdateUserPoolDomain request input.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateUserPoolDomainInput {
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub custom_domain_config: ::std::option::Option<crate::types::CustomDomainConfigType>,
}
impl UpdateUserPoolDomainInput {
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub fn custom_domain_config(&self) -> ::std::option::Option<&crate::types::CustomDomainConfigType> {
        self.custom_domain_config.as_ref()
    }
}
impl UpdateUserPoolDomainInput {
    /// Creates a new builder-style object to manufacture [`UpdateUserPoolDomainInput`](crate::operation::update_user_pool_domain::UpdateUserPoolDomainInput).
    pub fn builder() -> crate::operation::update_user_pool_domain::builders::UpdateUserPoolDomainInputBuilder {
        crate::operation::update_user_pool_domain::builders::UpdateUserPoolDomainInputBuilder::default()
    }
}

/// A builder for [`UpdateUserPoolDomainInput`](crate::operation::update_user_pool_domain::UpdateUserPoolDomainInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateUserPoolDomainInputBuilder {
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) custom_domain_config: ::std::option::Option<crate::types::CustomDomainConfigType>,
}
impl UpdateUserPoolDomainInputBuilder {
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    /// This field is required.
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>The domain name for the custom domain that hosts the sign-up and sign-in pages for your application. One example might be <code>auth.example.com</code>. </p>
    /// <p>This string can include only lowercase letters, numbers, and hyphens. Don't use a hyphen for the first or last character. Use periods to separate subdomain names.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    /// This field is required.
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The ID of the user pool that is associated with the custom domain whose certificate you're updating.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_pool_id
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    /// This field is required.
    pub fn custom_domain_config(mut self, input: crate::types::CustomDomainConfigType) -> Self {
        self.custom_domain_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub fn set_custom_domain_config(mut self, input: ::std::option::Option<crate::types::CustomDomainConfigType>) -> Self {
        self.custom_domain_config = input;
        self
    }
    /// <p>The configuration for a custom domain that hosts the sign-up and sign-in pages for your application. Use this object to specify an SSL certificate that is managed by ACM.</p>
    pub fn get_custom_domain_config(&self) -> &::std::option::Option<crate::types::CustomDomainConfigType> {
        &self.custom_domain_config
    }
    /// Consumes the builder and constructs a [`UpdateUserPoolDomainInput`](crate::operation::update_user_pool_domain::UpdateUserPoolDomainInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_user_pool_domain::UpdateUserPoolDomainInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::update_user_pool_domain::UpdateUserPoolDomainInput {
            domain: self.domain,
            user_pool_id: self.user_pool_id,
            custom_domain_config: self.custom_domain_config,
        })
    }
}
