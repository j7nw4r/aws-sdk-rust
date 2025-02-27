// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListApps`](crate::operation::list_apps::builders::ListAppsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_ids(impl Into<String>)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::app_ids) / [`set_app_ids(Option<Vec::<String>>)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::set_app_ids):<br>required: **false**<br><p>The unique application IDs.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_apps::builders::ListAppsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. </p><br>
    /// - On success, responds with [`ListAppsOutput`](crate::operation::list_apps::ListAppsOutput) with field(s):
    ///   - [`apps(Option<Vec::<AppSummary>>)`](crate::operation::list_apps::ListAppsOutput::apps): <p>The application summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_apps::ListAppsOutput::next_token): <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListAppsError>`](crate::operation::list_apps::ListAppsError)
    pub fn list_apps(&self) -> crate::operation::list_apps::builders::ListAppsFluentBuilder {
        crate::operation::list_apps::builders::ListAppsFluentBuilder::new(self.handle.clone())
    }
}
