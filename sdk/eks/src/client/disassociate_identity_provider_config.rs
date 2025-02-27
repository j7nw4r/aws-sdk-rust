// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateIdentityProviderConfig`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The name of the cluster to disassociate an identity provider from.</p><br>
    ///   - [`identity_provider_config(IdentityProviderConfig)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::identity_provider_config) / [`set_identity_provider_config(Option<IdentityProviderConfig>)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::set_identity_provider_config):<br>required: **true**<br><p>An object representing an identity provider configuration.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::set_client_request_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p><br>
    /// - On success, responds with [`DisassociateIdentityProviderConfigOutput`](crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput) with field(s):
    ///   - [`update(Option<Update>)`](crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigOutput::update): <p>An object representing an asynchronous update.</p>
    /// - On failure, responds with [`SdkError<DisassociateIdentityProviderConfigError>`](crate::operation::disassociate_identity_provider_config::DisassociateIdentityProviderConfigError)
    pub fn disassociate_identity_provider_config(
        &self,
    ) -> crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder {
        crate::operation::disassociate_identity_provider_config::builders::DisassociateIdentityProviderConfigFluentBuilder::new(self.handle.clone())
    }
}
