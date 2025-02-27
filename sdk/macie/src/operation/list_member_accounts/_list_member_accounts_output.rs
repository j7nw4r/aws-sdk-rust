// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListMemberAccountsOutput {
    /// <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    pub member_accounts: ::std::option::Option<::std::vec::Vec<crate::types::MemberAccount>>,
    /// <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListMemberAccountsOutput {
    /// <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.member_accounts.is_none()`.
    pub fn member_accounts(&self) -> &[crate::types::MemberAccount] {
        self.member_accounts.as_deref().unwrap_or_default()
    }
    /// <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListMemberAccountsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListMemberAccountsOutput {
    /// Creates a new builder-style object to manufacture [`ListMemberAccountsOutput`](crate::operation::list_member_accounts::ListMemberAccountsOutput).
    pub fn builder() -> crate::operation::list_member_accounts::builders::ListMemberAccountsOutputBuilder {
        crate::operation::list_member_accounts::builders::ListMemberAccountsOutputBuilder::default()
    }
}

/// A builder for [`ListMemberAccountsOutput`](crate::operation::list_member_accounts::ListMemberAccountsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListMemberAccountsOutputBuilder {
    pub(crate) member_accounts: ::std::option::Option<::std::vec::Vec<crate::types::MemberAccount>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListMemberAccountsOutputBuilder {
    /// Appends an item to `member_accounts`.
    ///
    /// To override the contents of this collection use [`set_member_accounts`](Self::set_member_accounts).
    ///
    /// <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    pub fn member_accounts(mut self, input: crate::types::MemberAccount) -> Self {
        let mut v = self.member_accounts.unwrap_or_default();
        v.push(input);
        self.member_accounts = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    pub fn set_member_accounts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MemberAccount>>) -> Self {
        self.member_accounts = input;
        self
    }
    /// <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    pub fn get_member_accounts(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MemberAccount>> {
        &self.member_accounts
    }
    /// <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListMemberAccountsOutput`](crate::operation::list_member_accounts::ListMemberAccountsOutput).
    pub fn build(self) -> crate::operation::list_member_accounts::ListMemberAccountsOutput {
        crate::operation::list_member_accounts::ListMemberAccountsOutput {
            member_accounts: self.member_accounts,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
