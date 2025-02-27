// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetKeyspace`](crate::operation::get_keyspace::builders::GetKeyspaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyspace_name(impl Into<String>)`](crate::operation::get_keyspace::builders::GetKeyspaceFluentBuilder::keyspace_name) / [`set_keyspace_name(Option<String>)`](crate::operation::get_keyspace::builders::GetKeyspaceFluentBuilder::set_keyspace_name):<br>required: **true**<br><p>The name of the keyspace.</p><br>
    /// - On success, responds with [`GetKeyspaceOutput`](crate::operation::get_keyspace::GetKeyspaceOutput) with field(s):
    ///   - [`keyspace_name(String)`](crate::operation::get_keyspace::GetKeyspaceOutput::keyspace_name): <p>The name of the keyspace.</p>
    ///   - [`resource_arn(String)`](crate::operation::get_keyspace::GetKeyspaceOutput::resource_arn): <p>Returns the ARN of the keyspace.</p>
    ///   - [`replication_strategy(Rs)`](crate::operation::get_keyspace::GetKeyspaceOutput::replication_strategy): <p> Returns the replication strategy of the keyspace. The options are <code>SINGLE_REGION</code> or <code>MULTI_REGION</code>. </p>
    ///   - [`replication_regions(Option<Vec::<String>>)`](crate::operation::get_keyspace::GetKeyspaceOutput::replication_regions): <p> If the <code>replicationStrategy</code> of the keyspace is <code>MULTI_REGION</code>, a list of replication Regions is returned. </p>
    /// - On failure, responds with [`SdkError<GetKeyspaceError>`](crate::operation::get_keyspace::GetKeyspaceError)
    pub fn get_keyspace(&self) -> crate::operation::get_keyspace::builders::GetKeyspaceFluentBuilder {
        crate::operation::get_keyspace::builders::GetKeyspaceFluentBuilder::new(self.handle.clone())
    }
}
