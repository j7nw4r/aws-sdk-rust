// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DescribeProjectOutput {
    /// <p>The display name for the project.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the project.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The description of the project, if any.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// <p>The date and time the project was created, in timestamp format.</p>
    pub created_time_stamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    pub stack_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    pub project_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The project creation or deletion status.</p>
    pub status: ::std::option::Option<crate::types::ProjectStatus>,
    _request_id: Option<String>,
}
impl DescribeProjectOutput {
    /// <p>The display name for the project.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ID of the project.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The description of the project, if any.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// <p>The date and time the project was created, in timestamp format.</p>
    pub fn created_time_stamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time_stamp.as_ref()
    }
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    pub fn stack_id(&self) -> ::std::option::Option<&str> {
        self.stack_id.as_deref()
    }
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    pub fn project_template_id(&self) -> ::std::option::Option<&str> {
        self.project_template_id.as_deref()
    }
    /// <p>The project creation or deletion status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ProjectStatus> {
        self.status.as_ref()
    }
}
impl ::std::fmt::Debug for DescribeProjectOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeProjectOutput");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("id", &self.id);
        formatter.field("arn", &self.arn);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("client_request_token", &self.client_request_token);
        formatter.field("created_time_stamp", &self.created_time_stamp);
        formatter.field("stack_id", &self.stack_id);
        formatter.field("project_template_id", &self.project_template_id);
        formatter.field("status", &self.status);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for DescribeProjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeProjectOutput {
    /// Creates a new builder-style object to manufacture [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
    pub fn builder() -> crate::operation::describe_project::builders::DescribeProjectOutputBuilder {
        crate::operation::describe_project::builders::DescribeProjectOutputBuilder::default()
    }
}

/// A builder for [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DescribeProjectOutputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) created_time_stamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) stack_id: ::std::option::Option<::std::string::String>,
    pub(crate) project_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ProjectStatus>,
    _request_id: Option<String>,
}
impl DescribeProjectOutputBuilder {
    /// <p>The display name for the project.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name for the project.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The display name for the project.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The ID of the project.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the project.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the project.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the project.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The description of the project, if any.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the project, if any.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the project, if any.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>A user- or system-generated token that identifies the entity that requested project creation. </p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_request_token
    }
    /// <p>The date and time the project was created, in timestamp format.</p>
    pub fn created_time_stamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time_stamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the project was created, in timestamp format.</p>
    pub fn set_created_time_stamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_time_stamp = input;
        self
    }
    /// <p>The date and time the project was created, in timestamp format.</p>
    pub fn get_created_time_stamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_time_stamp
    }
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_id = input;
        self
    }
    /// <p>The ID of the primary stack in AWS CloudFormation used to generate resources for the project.</p>
    pub fn get_stack_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.stack_id
    }
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    pub fn project_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    pub fn set_project_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_template_id = input;
        self
    }
    /// <p>The ID for the AWS CodeStar project template used to create the project.</p>
    pub fn get_project_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.project_template_id
    }
    /// <p>The project creation or deletion status.</p>
    pub fn status(mut self, input: crate::types::ProjectStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The project creation or deletion status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ProjectStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The project creation or deletion status.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ProjectStatus> {
        &self.status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeProjectOutput`](crate::operation::describe_project::DescribeProjectOutput).
    pub fn build(self) -> crate::operation::describe_project::DescribeProjectOutput {
        crate::operation::describe_project::DescribeProjectOutput {
            name: self.name,
            id: self.id,
            arn: self.arn,
            description: self.description,
            client_request_token: self.client_request_token,
            created_time_stamp: self.created_time_stamp,
            stack_id: self.stack_id,
            project_template_id: self.project_template_id,
            status: self.status,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for DescribeProjectOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeProjectOutputBuilder");
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("id", &self.id);
        formatter.field("arn", &self.arn);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("client_request_token", &self.client_request_token);
        formatter.field("created_time_stamp", &self.created_time_stamp);
        formatter.field("stack_id", &self.stack_id);
        formatter.field("project_template_id", &self.project_template_id);
        formatter.field("status", &self.status);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
