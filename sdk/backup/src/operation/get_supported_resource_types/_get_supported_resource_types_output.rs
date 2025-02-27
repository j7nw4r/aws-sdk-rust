// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSupportedResourceTypesOutput {
    /// <p>Contains a string with the supported Amazon Web Services resource types:</p>
    /// <ul>
    /// <li> <p> <code>Aurora</code> for Amazon Aurora</p> </li>
    /// <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li>
    /// <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li>
    /// <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li>
    /// <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li>
    /// <li> <p> <code>FSX</code> for Amazon FSx</p> </li>
    /// <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li>
    /// <li> <p> <code>Storage Gateway</code> for Storage Gateway</p> </li>
    /// <li> <p> <code>DocDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p> </li>
    /// <li> <p> <code>Neptune</code> for Amazon Neptune</p> </li>
    /// </ul>
    pub resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSupportedResourceTypesOutput {
    /// <p>Contains a string with the supported Amazon Web Services resource types:</p>
    /// <ul>
    /// <li> <p> <code>Aurora</code> for Amazon Aurora</p> </li>
    /// <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li>
    /// <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li>
    /// <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li>
    /// <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li>
    /// <li> <p> <code>FSX</code> for Amazon FSx</p> </li>
    /// <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li>
    /// <li> <p> <code>Storage Gateway</code> for Storage Gateway</p> </li>
    /// <li> <p> <code>DocDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p> </li>
    /// <li> <p> <code>Neptune</code> for Amazon Neptune</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resource_types.is_none()`.
    pub fn resource_types(&self) -> &[::std::string::String] {
        self.resource_types.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for GetSupportedResourceTypesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSupportedResourceTypesOutput {
    /// Creates a new builder-style object to manufacture [`GetSupportedResourceTypesOutput`](crate::operation::get_supported_resource_types::GetSupportedResourceTypesOutput).
    pub fn builder() -> crate::operation::get_supported_resource_types::builders::GetSupportedResourceTypesOutputBuilder {
        crate::operation::get_supported_resource_types::builders::GetSupportedResourceTypesOutputBuilder::default()
    }
}

/// A builder for [`GetSupportedResourceTypesOutput`](crate::operation::get_supported_resource_types::GetSupportedResourceTypesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetSupportedResourceTypesOutputBuilder {
    pub(crate) resource_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl GetSupportedResourceTypesOutputBuilder {
    /// Appends an item to `resource_types`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>Contains a string with the supported Amazon Web Services resource types:</p>
    /// <ul>
    /// <li> <p> <code>Aurora</code> for Amazon Aurora</p> </li>
    /// <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li>
    /// <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li>
    /// <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li>
    /// <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li>
    /// <li> <p> <code>FSX</code> for Amazon FSx</p> </li>
    /// <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li>
    /// <li> <p> <code>Storage Gateway</code> for Storage Gateway</p> </li>
    /// <li> <p> <code>DocDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p> </li>
    /// <li> <p> <code>Neptune</code> for Amazon Neptune</p> </li>
    /// </ul>
    pub fn resource_types(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resource_types.unwrap_or_default();
        v.push(input.into());
        self.resource_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>Contains a string with the supported Amazon Web Services resource types:</p>
    /// <ul>
    /// <li> <p> <code>Aurora</code> for Amazon Aurora</p> </li>
    /// <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li>
    /// <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li>
    /// <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li>
    /// <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li>
    /// <li> <p> <code>FSX</code> for Amazon FSx</p> </li>
    /// <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li>
    /// <li> <p> <code>Storage Gateway</code> for Storage Gateway</p> </li>
    /// <li> <p> <code>DocDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p> </li>
    /// <li> <p> <code>Neptune</code> for Amazon Neptune</p> </li>
    /// </ul>
    pub fn set_resource_types(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.resource_types = input;
        self
    }
    /// <p>Contains a string with the supported Amazon Web Services resource types:</p>
    /// <ul>
    /// <li> <p> <code>Aurora</code> for Amazon Aurora</p> </li>
    /// <li> <p> <code>DynamoDB</code> for Amazon DynamoDB</p> </li>
    /// <li> <p> <code>EBS</code> for Amazon Elastic Block Store</p> </li>
    /// <li> <p> <code>EC2</code> for Amazon Elastic Compute Cloud</p> </li>
    /// <li> <p> <code>EFS</code> for Amazon Elastic File System</p> </li>
    /// <li> <p> <code>FSX</code> for Amazon FSx</p> </li>
    /// <li> <p> <code>RDS</code> for Amazon Relational Database Service</p> </li>
    /// <li> <p> <code>Storage Gateway</code> for Storage Gateway</p> </li>
    /// <li> <p> <code>DocDB</code> for Amazon DocumentDB (with MongoDB compatibility)</p> </li>
    /// <li> <p> <code>Neptune</code> for Amazon Neptune</p> </li>
    /// </ul>
    pub fn get_resource_types(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.resource_types
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSupportedResourceTypesOutput`](crate::operation::get_supported_resource_types::GetSupportedResourceTypesOutput).
    pub fn build(self) -> crate::operation::get_supported_resource_types::GetSupportedResourceTypesOutput {
        crate::operation::get_supported_resource_types::GetSupportedResourceTypesOutput {
            resource_types: self.resource_types,
            _request_id: self._request_id,
        }
    }
}
