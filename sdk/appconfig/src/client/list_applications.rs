// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListApplications`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_applications::builders::ListApplicationsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to start the list. Next token is a pagination token generated by AppConfig to describe what page the previous List call ended on. For the first List request, the nextToken should not be set. On subsequent calls, the nextToken parameter should be set to the previous responses nextToken value. Use this token to get the next set of results. </p><br>
    /// - On success, responds with [`ListApplicationsOutput`](crate::operation::list_applications::ListApplicationsOutput) with field(s):
    ///   - [`items(Option<Vec::<Application>>)`](crate::operation::list_applications::ListApplicationsOutput::items): <p>The elements from this collection.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_applications::ListApplicationsOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListApplicationsError>`](crate::operation::list_applications::ListApplicationsError)
    pub fn list_applications(&self) -> crate::operation::list_applications::builders::ListApplicationsFluentBuilder {
        crate::operation::list_applications::builders::ListApplicationsFluentBuilder::new(self.handle.clone())
    }
}
