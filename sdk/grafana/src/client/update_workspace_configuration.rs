// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateWorkspaceConfiguration`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration(impl Into<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::configuration) / [`set_configuration(Option<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::set_configuration):<br>required: **true**<br><p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p><br>
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The ID of the workspace to update.</p><br>
    ///   - [`grafana_version(impl Into<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::grafana_version) / [`set_grafana_version(Option<String>)`](crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::set_grafana_version):<br>required: **false**<br><p>Specifies the version of Grafana to support in the new workspace.</p>  <p>Can only be used to upgrade (for example, from 8.4 to 9.4), not downgrade (for example, from 9.4 to 8.4).</p>  <p>To know what versions are available to upgrade to for a specific workspace, see the <code>ListVersions</code> operation.</p><br>
    /// - On success, responds with [`UpdateWorkspaceConfigurationOutput`](crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationOutput)
    /// - On failure, responds with [`SdkError<UpdateWorkspaceConfigurationError>`](crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationError)
    pub fn update_workspace_configuration(
        &self,
    ) -> crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder {
        crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationFluentBuilder::new(self.handle.clone())
    }
}
