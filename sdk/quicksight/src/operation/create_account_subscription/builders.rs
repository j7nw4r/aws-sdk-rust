// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_account_subscription::_create_account_subscription_output::CreateAccountSubscriptionOutputBuilder;

pub use crate::operation::create_account_subscription::_create_account_subscription_input::CreateAccountSubscriptionInputBuilder;

impl CreateAccountSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_account_subscription::CreateAccountSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_account_subscription::CreateAccountSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_account_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAccountSubscription`.
///
/// <p>Creates an Amazon QuickSight account, or subscribes to Amazon QuickSight Q.</p>
/// <p>The Amazon Web Services Region for the account is derived from what is configured in the CLI or SDK. This operation isn't supported in the US East (Ohio) Region, South America (Sao Paulo) Region, or Asia Pacific (Singapore) Region. </p>
/// <p>Before you use this operation, make sure that you can connect to an existing Amazon Web Services account. If you don't have an Amazon Web Services account, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/setting-up-aws-sign-up.html">Sign up for Amazon Web Services</a> in the <i>Amazon QuickSight User Guide</i>. The person who signs up for Amazon QuickSight needs to have the correct Identity and Access Management (IAM) permissions. For more information, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/iam-policy-examples.html">IAM Policy Examples for Amazon QuickSight</a> in the <i>Amazon QuickSight User Guide</i>.</p>
/// <p>If your IAM policy includes both the <code>Subscribe</code> and <code>CreateAccountSubscription</code> actions, make sure that both actions are set to <code>Allow</code>. If either action is set to <code>Deny</code>, the <code>Deny</code> action prevails and your API call fails.</p>
/// <p>You can't pass an existing IAM role to access other Amazon Web Services services using this API operation. To pass your existing IAM role to Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/security_iam_service-with-iam.html#security-create-iam-role">Passing IAM roles to Amazon QuickSight</a> in the <i>Amazon QuickSight User Guide</i>.</p>
/// <p>You can't set default resource access on the new account from the Amazon QuickSight API. Instead, add default resource access from the Amazon QuickSight console. For more information about setting default resource access to Amazon Web Services services, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/scoping-policies-defaults.html">Setting default resource access to Amazon Web Services services</a> in the <i>Amazon QuickSight User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAccountSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_account_subscription::builders::CreateAccountSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_account_subscription::CreateAccountSubscriptionOutput,
        crate::operation::create_account_subscription::CreateAccountSubscriptionError,
    > for CreateAccountSubscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_account_subscription::CreateAccountSubscriptionOutput,
            crate::operation::create_account_subscription::CreateAccountSubscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAccountSubscriptionFluentBuilder {
    /// Creates a new `CreateAccountSubscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAccountSubscription as a reference.
    pub fn as_input(&self) -> &crate::operation::create_account_subscription::builders::CreateAccountSubscriptionInputBuilder {
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
        crate::operation::create_account_subscription::CreateAccountSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_account_subscription::CreateAccountSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_account_subscription::CreateAccountSubscription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_account_subscription::CreateAccountSubscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_account_subscription::CreateAccountSubscriptionOutput,
        crate::operation::create_account_subscription::CreateAccountSubscriptionError,
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
    /// <p>The edition of Amazon QuickSight that you want your account to have. Currently, you can choose from <code>ENTERPRISE</code> or <code>ENTERPRISE_AND_Q</code>.</p>
    /// <p>If you choose <code>ENTERPRISE_AND_Q</code>, the following parameters are required:</p>
    /// <ul>
    /// <li> <p> <code>FirstName</code> </p> </li>
    /// <li> <p> <code>LastName</code> </p> </li>
    /// <li> <p> <code>EmailAddress</code> </p> </li>
    /// <li> <p> <code>ContactNumber</code> </p> </li>
    /// </ul>
    pub fn edition(mut self, input: crate::types::Edition) -> Self {
        self.inner = self.inner.edition(input);
        self
    }
    /// <p>The edition of Amazon QuickSight that you want your account to have. Currently, you can choose from <code>ENTERPRISE</code> or <code>ENTERPRISE_AND_Q</code>.</p>
    /// <p>If you choose <code>ENTERPRISE_AND_Q</code>, the following parameters are required:</p>
    /// <ul>
    /// <li> <p> <code>FirstName</code> </p> </li>
    /// <li> <p> <code>LastName</code> </p> </li>
    /// <li> <p> <code>EmailAddress</code> </p> </li>
    /// <li> <p> <code>ContactNumber</code> </p> </li>
    /// </ul>
    pub fn set_edition(mut self, input: ::std::option::Option<crate::types::Edition>) -> Self {
        self.inner = self.inner.set_edition(input);
        self
    }
    /// <p>The edition of Amazon QuickSight that you want your account to have. Currently, you can choose from <code>ENTERPRISE</code> or <code>ENTERPRISE_AND_Q</code>.</p>
    /// <p>If you choose <code>ENTERPRISE_AND_Q</code>, the following parameters are required:</p>
    /// <ul>
    /// <li> <p> <code>FirstName</code> </p> </li>
    /// <li> <p> <code>LastName</code> </p> </li>
    /// <li> <p> <code>EmailAddress</code> </p> </li>
    /// <li> <p> <code>ContactNumber</code> </p> </li>
    /// </ul>
    pub fn get_edition(&self) -> &::std::option::Option<crate::types::Edition> {
        self.inner.get_edition()
    }
    /// <p>The method that you want to use to authenticate your Amazon QuickSight account.</p>
    /// <p>If you choose <code>ACTIVE_DIRECTORY</code>, provide an <code>ActiveDirectoryName</code> and an <code>AdminGroup</code> associated with your Active Directory.</p>
    /// <p>If you choose <code>IAM_IDENTITY_CENTER</code>, provide an <code>AdminGroup</code> associated with your IAM Identity Center account.</p>
    pub fn authentication_method(mut self, input: crate::types::AuthenticationMethodOption) -> Self {
        self.inner = self.inner.authentication_method(input);
        self
    }
    /// <p>The method that you want to use to authenticate your Amazon QuickSight account.</p>
    /// <p>If you choose <code>ACTIVE_DIRECTORY</code>, provide an <code>ActiveDirectoryName</code> and an <code>AdminGroup</code> associated with your Active Directory.</p>
    /// <p>If you choose <code>IAM_IDENTITY_CENTER</code>, provide an <code>AdminGroup</code> associated with your IAM Identity Center account.</p>
    pub fn set_authentication_method(mut self, input: ::std::option::Option<crate::types::AuthenticationMethodOption>) -> Self {
        self.inner = self.inner.set_authentication_method(input);
        self
    }
    /// <p>The method that you want to use to authenticate your Amazon QuickSight account.</p>
    /// <p>If you choose <code>ACTIVE_DIRECTORY</code>, provide an <code>ActiveDirectoryName</code> and an <code>AdminGroup</code> associated with your Active Directory.</p>
    /// <p>If you choose <code>IAM_IDENTITY_CENTER</code>, provide an <code>AdminGroup</code> associated with your IAM Identity Center account.</p>
    pub fn get_authentication_method(&self) -> &::std::option::Option<crate::types::AuthenticationMethodOption> {
        self.inner.get_authentication_method()
    }
    /// <p>The Amazon Web Services account ID of the account that you're using to create your Amazon QuickSight account.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the account that you're using to create your Amazon QuickSight account.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID of the account that you're using to create your Amazon QuickSight account.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The name of your Amazon QuickSight account. This name is unique over all of Amazon Web Services, and it appears only when users sign in. You can't change <code>AccountName</code> value after the Amazon QuickSight account is created.</p>
    pub fn account_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_name(input.into());
        self
    }
    /// <p>The name of your Amazon QuickSight account. This name is unique over all of Amazon Web Services, and it appears only when users sign in. You can't change <code>AccountName</code> value after the Amazon QuickSight account is created.</p>
    pub fn set_account_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_name(input);
        self
    }
    /// <p>The name of your Amazon QuickSight account. This name is unique over all of Amazon Web Services, and it appears only when users sign in. You can't change <code>AccountName</code> value after the Amazon QuickSight account is created.</p>
    pub fn get_account_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_name()
    }
    /// <p>The email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.</p>
    pub fn notification_email(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.notification_email(input.into());
        self
    }
    /// <p>The email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.</p>
    pub fn set_notification_email(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_notification_email(input);
        self
    }
    /// <p>The email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.</p>
    pub fn get_notification_email(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_notification_email()
    }
    /// <p>The name of your Active Directory. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn active_directory_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.active_directory_name(input.into());
        self
    }
    /// <p>The name of your Active Directory. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn set_active_directory_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_active_directory_name(input);
        self
    }
    /// <p>The name of your Active Directory. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn get_active_directory_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_active_directory_name()
    }
    /// <p>The realm of the Active Directory that is associated with your Amazon QuickSight account. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn realm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.realm(input.into());
        self
    }
    /// <p>The realm of the Active Directory that is associated with your Amazon QuickSight account. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn set_realm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_realm(input);
        self
    }
    /// <p>The realm of the Active Directory that is associated with your Amazon QuickSight account. This field is required if <code>ACTIVE_DIRECTORY</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    pub fn get_realm(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_realm()
    }
    /// <p>The ID of the Active Directory that is associated with your Amazon QuickSight account.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The ID of the Active Directory that is associated with your Amazon QuickSight account.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The ID of the Active Directory that is associated with your Amazon QuickSight account.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// Appends an item to `AdminGroup`.
    ///
    /// To override the contents of this collection use [`set_admin_group`](Self::set_admin_group).
    ///
    /// <p>The admin group associated with your Active Directory or IAM Identity Center account. This field is required if <code>ACTIVE_DIRECTORY</code> or <code>IAM_IDENTITY_CENTER</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn admin_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.admin_group(input.into());
        self
    }
    /// <p>The admin group associated with your Active Directory or IAM Identity Center account. This field is required if <code>ACTIVE_DIRECTORY</code> or <code>IAM_IDENTITY_CENTER</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn set_admin_group(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_admin_group(input);
        self
    }
    /// <p>The admin group associated with your Active Directory or IAM Identity Center account. This field is required if <code>ACTIVE_DIRECTORY</code> or <code>IAM_IDENTITY_CENTER</code> is the selected authentication method of the new Amazon QuickSight account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn get_admin_group(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_admin_group()
    }
    /// Appends an item to `AuthorGroup`.
    ///
    /// To override the contents of this collection use [`set_author_group`](Self::set_author_group).
    ///
    /// <p>The author group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn author_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.author_group(input.into());
        self
    }
    /// <p>The author group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn set_author_group(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_author_group(input);
        self
    }
    /// <p>The author group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn get_author_group(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_author_group()
    }
    /// Appends an item to `ReaderGroup`.
    ///
    /// To override the contents of this collection use [`set_reader_group`](Self::set_reader_group).
    ///
    /// <p>The reader group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn reader_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reader_group(input.into());
        self
    }
    /// <p>The reader group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn set_reader_group(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_reader_group(input);
        self
    }
    /// <p>The reader group associated with your Active Directory or IAM Identity Center account.</p>
    /// <p>For more information about using IAM Identity Center in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/sec-identity-management-identity-center.html">Using IAM Identity Center with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide. For more information about using Active Directory in Amazon QuickSight, see <a href="https://docs.aws.amazon.com/quicksight/latest/user/aws-directory-service.html">Using Active Directory with Amazon QuickSight Enterprise Edition</a> in the Amazon QuickSight User Guide.</p>
    pub fn get_reader_group(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_reader_group()
    }
    /// <p>The first name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn first_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.first_name(input.into());
        self
    }
    /// <p>The first name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn set_first_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_first_name(input);
        self
    }
    /// <p>The first name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn get_first_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_first_name()
    }
    /// <p>The last name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn last_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.last_name(input.into());
        self
    }
    /// <p>The last name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn set_last_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_last_name(input);
        self
    }
    /// <p>The last name of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn get_last_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_last_name()
    }
    /// <p>The email address of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn email_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.email_address(input.into());
        self
    }
    /// <p>The email address of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn set_email_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_email_address(input);
        self
    }
    /// <p>The email address of the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn get_email_address(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_email_address()
    }
    /// <p>A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn contact_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_number(input.into());
        self
    }
    /// <p>A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn set_contact_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_number(input);
        self
    }
    /// <p>A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if <code>ENTERPPRISE_AND_Q</code> is the selected edition of the new Amazon QuickSight account.</p>
    pub fn get_contact_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_number()
    }
}
