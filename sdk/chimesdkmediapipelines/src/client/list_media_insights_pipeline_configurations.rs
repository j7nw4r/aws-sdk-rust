// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMediaInsightsPipelineConfigurations`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used to return the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListMediaInsightsPipelineConfigurationsOutput`](crate::operation::list_media_insights_pipeline_configurations::ListMediaInsightsPipelineConfigurationsOutput) with field(s):
    ///   - [`media_insights_pipeline_configurations(Option<Vec::<MediaInsightsPipelineConfigurationSummary>>)`](crate::operation::list_media_insights_pipeline_configurations::ListMediaInsightsPipelineConfigurationsOutput::media_insights_pipeline_configurations): <p>The requested list of media insights pipeline configurations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_media_insights_pipeline_configurations::ListMediaInsightsPipelineConfigurationsOutput::next_token): <p>The token used to return the next page of results. </p>
    /// - On failure, responds with [`SdkError<ListMediaInsightsPipelineConfigurationsError>`](crate::operation::list_media_insights_pipeline_configurations::ListMediaInsightsPipelineConfigurationsError)
    pub fn list_media_insights_pipeline_configurations(
        &self,
    ) -> crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder {
        crate::operation::list_media_insights_pipeline_configurations::builders::ListMediaInsightsPipelineConfigurationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
