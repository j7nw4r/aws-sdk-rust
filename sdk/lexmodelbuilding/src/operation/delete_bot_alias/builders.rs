// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_bot_alias::_delete_bot_alias_output::DeleteBotAliasOutputBuilder;

pub use crate::operation::delete_bot_alias::_delete_bot_alias_input::DeleteBotAliasInputBuilder;

impl DeleteBotAliasInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_bot_alias::DeleteBotAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_bot_alias::DeleteBotAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_bot_alias();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteBotAlias`.
///
/// <p>Deletes an alias for the specified bot. </p>
/// <p>You can't delete an alias that is used in the association between a bot and a messaging channel. If an alias is used in a channel association, the <code>DeleteBot</code> operation returns a <code>ResourceInUseException</code> exception that includes a reference to the channel association that refers to the bot. You can remove the reference to the alias by deleting the channel association. If you get the same exception again, delete the referring association until the <code>DeleteBotAlias</code> operation is successful.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteBotAliasFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_bot_alias::builders::DeleteBotAliasInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_bot_alias::DeleteBotAliasOutput,
        crate::operation::delete_bot_alias::DeleteBotAliasError,
    > for DeleteBotAliasFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_bot_alias::DeleteBotAliasOutput,
            crate::operation::delete_bot_alias::DeleteBotAliasError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteBotAliasFluentBuilder {
    /// Creates a new `DeleteBotAlias`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteBotAlias as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_bot_alias::builders::DeleteBotAliasInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_bot_alias::DeleteBotAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_bot_alias::DeleteBotAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_bot_alias::DeleteBotAlias::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_bot_alias::DeleteBotAlias::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_bot_alias::DeleteBotAliasOutput,
        crate::operation::delete_bot_alias::DeleteBotAliasError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the alias to delete. The name is case sensitive. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the alias to delete. The name is case sensitive. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the alias to delete. The name is case sensitive. </p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The name of the bot that the alias points to.</p>
    pub fn bot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_name(input.into());
        self
    }
    /// <p>The name of the bot that the alias points to.</p>
    pub fn set_bot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_name(input);
        self
    }
    /// <p>The name of the bot that the alias points to.</p>
    pub fn get_bot_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_name()
    }
}
