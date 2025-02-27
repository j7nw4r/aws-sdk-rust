// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The response from the request to list users.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListUsersOutput {
    /// <p>A list of the user pool users, and their attributes, that match your query.</p> <note>
    /// <p>Amazon Cognito creates a profile in your user pool for each native user in your user pool, and each unique user ID from your third-party identity providers (IdPs). When you link users with the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html">AdminLinkProviderForUser</a> API operation, the output of <code>ListUsers</code> displays both the IdP user and the native user that you linked. You can identify IdP users in the <code>Users</code> object of this API response by the IdP prefix that Amazon Cognito appends to <code>Username</code>.</p>
    /// </note>
    pub users: ::std::option::Option<::std::vec::Vec<crate::types::UserType>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub pagination_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListUsersOutput {
    /// <p>A list of the user pool users, and their attributes, that match your query.</p> <note>
    /// <p>Amazon Cognito creates a profile in your user pool for each native user in your user pool, and each unique user ID from your third-party identity providers (IdPs). When you link users with the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html">AdminLinkProviderForUser</a> API operation, the output of <code>ListUsers</code> displays both the IdP user and the native user that you linked. You can identify IdP users in the <code>Users</code> object of this API response by the IdP prefix that Amazon Cognito appends to <code>Username</code>.</p>
    /// </note>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.users.is_none()`.
    pub fn users(&self) -> &[crate::types::UserType] {
        self.users.as_deref().unwrap_or_default()
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn pagination_token(&self) -> ::std::option::Option<&str> {
        self.pagination_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListUsersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListUsersOutput {
    /// Creates a new builder-style object to manufacture [`ListUsersOutput`](crate::operation::list_users::ListUsersOutput).
    pub fn builder() -> crate::operation::list_users::builders::ListUsersOutputBuilder {
        crate::operation::list_users::builders::ListUsersOutputBuilder::default()
    }
}

/// A builder for [`ListUsersOutput`](crate::operation::list_users::ListUsersOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListUsersOutputBuilder {
    pub(crate) users: ::std::option::Option<::std::vec::Vec<crate::types::UserType>>,
    pub(crate) pagination_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListUsersOutputBuilder {
    /// Appends an item to `users`.
    ///
    /// To override the contents of this collection use [`set_users`](Self::set_users).
    ///
    /// <p>A list of the user pool users, and their attributes, that match your query.</p> <note>
    /// <p>Amazon Cognito creates a profile in your user pool for each native user in your user pool, and each unique user ID from your third-party identity providers (IdPs). When you link users with the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html">AdminLinkProviderForUser</a> API operation, the output of <code>ListUsers</code> displays both the IdP user and the native user that you linked. You can identify IdP users in the <code>Users</code> object of this API response by the IdP prefix that Amazon Cognito appends to <code>Username</code>.</p>
    /// </note>
    pub fn users(mut self, input: crate::types::UserType) -> Self {
        let mut v = self.users.unwrap_or_default();
        v.push(input);
        self.users = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the user pool users, and their attributes, that match your query.</p> <note>
    /// <p>Amazon Cognito creates a profile in your user pool for each native user in your user pool, and each unique user ID from your third-party identity providers (IdPs). When you link users with the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html">AdminLinkProviderForUser</a> API operation, the output of <code>ListUsers</code> displays both the IdP user and the native user that you linked. You can identify IdP users in the <code>Users</code> object of this API response by the IdP prefix that Amazon Cognito appends to <code>Username</code>.</p>
    /// </note>
    pub fn set_users(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UserType>>) -> Self {
        self.users = input;
        self
    }
    /// <p>A list of the user pool users, and their attributes, that match your query.</p> <note>
    /// <p>Amazon Cognito creates a profile in your user pool for each native user in your user pool, and each unique user ID from your third-party identity providers (IdPs). When you link users with the <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html">AdminLinkProviderForUser</a> API operation, the output of <code>ListUsers</code> displays both the IdP user and the native user that you linked. You can identify IdP users in the <code>Users</code> object of this API response by the IdP prefix that Amazon Cognito appends to <code>Username</code>.</p>
    /// </note>
    pub fn get_users(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UserType>> {
        &self.users
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn pagination_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pagination_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn set_pagination_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pagination_token = input;
        self
    }
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    pub fn get_pagination_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.pagination_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListUsersOutput`](crate::operation::list_users::ListUsersOutput).
    pub fn build(self) -> crate::operation::list_users::ListUsersOutput {
        crate::operation::list_users::ListUsersOutput {
            users: self.users,
            pagination_token: self.pagination_token,
            _request_id: self._request_id,
        }
    }
}
