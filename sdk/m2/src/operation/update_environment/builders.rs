// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_environment::_update_environment_output::UpdateEnvironmentOutputBuilder;

pub use crate::operation::update_environment::_update_environment_input::UpdateEnvironmentInputBuilder;

impl UpdateEnvironmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_environment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateEnvironment`.
///
/// <p>Updates the configuration details for a specific runtime environment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_environment::builders::UpdateEnvironmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_environment::UpdateEnvironmentOutput,
        crate::operation::update_environment::UpdateEnvironmentError,
    > for UpdateEnvironmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_environment::UpdateEnvironmentOutput,
            crate::operation::update_environment::UpdateEnvironmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateEnvironmentFluentBuilder {
    /// Creates a new `UpdateEnvironment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateEnvironment as a reference.
    pub fn as_input(&self) -> &crate::operation::update_environment::builders::UpdateEnvironmentInputBuilder {
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
        crate::operation::update_environment::UpdateEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_environment::UpdateEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_environment::UpdateEnvironment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_environment::UpdateEnvironment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_environment::UpdateEnvironmentOutput,
        crate::operation::update_environment::UpdateEnvironmentError,
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
    /// <p>The unique identifier of the runtime environment that you want to update.</p>
    pub fn environment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The unique identifier of the runtime environment that you want to update.</p>
    pub fn set_environment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The unique identifier of the runtime environment that you want to update.</p>
    pub fn get_environment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_id()
    }
    /// <p>The desired capacity for the runtime environment to update. The minimum possible value is 0 and the maximum is 100.</p>
    pub fn desired_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.desired_capacity(input);
        self
    }
    /// <p>The desired capacity for the runtime environment to update. The minimum possible value is 0 and the maximum is 100.</p>
    pub fn set_desired_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_desired_capacity(input);
        self
    }
    /// <p>The desired capacity for the runtime environment to update. The minimum possible value is 0 and the maximum is 100.</p>
    pub fn get_desired_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_desired_capacity()
    }
    /// <p>The instance type for the runtime environment to update.</p>
    pub fn instance_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_type(input.into());
        self
    }
    /// <p>The instance type for the runtime environment to update.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>The instance type for the runtime environment to update.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_type()
    }
    /// <p>The version of the runtime engine for the runtime environment.</p>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The version of the runtime engine for the runtime environment.</p>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The version of the runtime engine for the runtime environment.</p>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format <code>ddd:hh24:mi-ddd:hh24:mi</code> and must be less than 24 hours. The following two examples are valid maintenance windows: <code>sun:23:45-mon:00:15</code> or <code>sat:01:00-sat:03:00</code>. </p>
    /// <p>If you do not provide a value, a random system-generated value will be assigned.</p>
    pub fn preferred_maintenance_window(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.preferred_maintenance_window(input.into());
        self
    }
    /// <p>Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format <code>ddd:hh24:mi-ddd:hh24:mi</code> and must be less than 24 hours. The following two examples are valid maintenance windows: <code>sun:23:45-mon:00:15</code> or <code>sat:01:00-sat:03:00</code>. </p>
    /// <p>If you do not provide a value, a random system-generated value will be assigned.</p>
    pub fn set_preferred_maintenance_window(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_preferred_maintenance_window(input);
        self
    }
    /// <p>Configures the maintenance window that you want for the runtime environment. The maintenance window must have the format <code>ddd:hh24:mi-ddd:hh24:mi</code> and must be less than 24 hours. The following two examples are valid maintenance windows: <code>sun:23:45-mon:00:15</code> or <code>sat:01:00-sat:03:00</code>. </p>
    /// <p>If you do not provide a value, a random system-generated value will be assigned.</p>
    pub fn get_preferred_maintenance_window(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_preferred_maintenance_window()
    }
    /// <p>Indicates whether to update the runtime environment during the maintenance window. The default is false. Currently, Amazon Web Services Mainframe Modernization accepts the <code>engineVersion</code> parameter only if <code>applyDuringMaintenanceWindow</code> is true. If any parameter other than <code>engineVersion</code> is provided in <code>UpdateEnvironmentRequest</code>, it will fail if <code>applyDuringMaintenanceWindow</code> is set to true.</p>
    pub fn apply_during_maintenance_window(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_during_maintenance_window(input);
        self
    }
    /// <p>Indicates whether to update the runtime environment during the maintenance window. The default is false. Currently, Amazon Web Services Mainframe Modernization accepts the <code>engineVersion</code> parameter only if <code>applyDuringMaintenanceWindow</code> is true. If any parameter other than <code>engineVersion</code> is provided in <code>UpdateEnvironmentRequest</code>, it will fail if <code>applyDuringMaintenanceWindow</code> is set to true.</p>
    pub fn set_apply_during_maintenance_window(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_during_maintenance_window(input);
        self
    }
    /// <p>Indicates whether to update the runtime environment during the maintenance window. The default is false. Currently, Amazon Web Services Mainframe Modernization accepts the <code>engineVersion</code> parameter only if <code>applyDuringMaintenanceWindow</code> is true. If any parameter other than <code>engineVersion</code> is provided in <code>UpdateEnvironmentRequest</code>, it will fail if <code>applyDuringMaintenanceWindow</code> is set to true.</p>
    pub fn get_apply_during_maintenance_window(&self) -> &::std::option::Option<bool> {
        self.inner.get_apply_during_maintenance_window()
    }
    /// <p>Forces the updates on the environment. This option is needed if the applications in the environment are not stopped or if there are ongoing application-related activities in the environment.</p>
    /// <p>If you use this option, be aware that it could lead to data corruption in the applications, and that you might need to perform repair and recovery procedures for the applications.</p>
    /// <p>This option is not needed if the attribute being updated is <code>preferredMaintenanceWindow</code>.</p>
    pub fn force_update(mut self, input: bool) -> Self {
        self.inner = self.inner.force_update(input);
        self
    }
    /// <p>Forces the updates on the environment. This option is needed if the applications in the environment are not stopped or if there are ongoing application-related activities in the environment.</p>
    /// <p>If you use this option, be aware that it could lead to data corruption in the applications, and that you might need to perform repair and recovery procedures for the applications.</p>
    /// <p>This option is not needed if the attribute being updated is <code>preferredMaintenanceWindow</code>.</p>
    pub fn set_force_update(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_update(input);
        self
    }
    /// <p>Forces the updates on the environment. This option is needed if the applications in the environment are not stopped or if there are ongoing application-related activities in the environment.</p>
    /// <p>If you use this option, be aware that it could lead to data corruption in the applications, and that you might need to perform repair and recovery procedures for the applications.</p>
    /// <p>This option is not needed if the attribute being updated is <code>preferredMaintenanceWindow</code>.</p>
    pub fn get_force_update(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_update()
    }
}
