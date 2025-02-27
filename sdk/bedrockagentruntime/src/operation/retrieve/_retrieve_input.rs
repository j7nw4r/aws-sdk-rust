// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct RetrieveInput {
    /// Identifier of the KnowledgeBase
    pub knowledge_base_id: ::std::option::Option<::std::string::String>,
    /// Knowledge base input query.
    pub retrieval_query: ::std::option::Option<crate::types::KnowledgeBaseQuery>,
    /// Search parameters for retrieving from knowledge base.
    pub retrieval_configuration: ::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration>,
    /// Opaque continuation token of previous paginated response.
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl RetrieveInput {
    /// Identifier of the KnowledgeBase
    pub fn knowledge_base_id(&self) -> ::std::option::Option<&str> {
        self.knowledge_base_id.as_deref()
    }
    /// Knowledge base input query.
    pub fn retrieval_query(&self) -> ::std::option::Option<&crate::types::KnowledgeBaseQuery> {
        self.retrieval_query.as_ref()
    }
    /// Search parameters for retrieving from knowledge base.
    pub fn retrieval_configuration(&self) -> ::std::option::Option<&crate::types::KnowledgeBaseRetrievalConfiguration> {
        self.retrieval_configuration.as_ref()
    }
    /// Opaque continuation token of previous paginated response.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::std::fmt::Debug for RetrieveInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("RetrieveInput");
        formatter.field("knowledge_base_id", &self.knowledge_base_id);
        formatter.field("retrieval_query", &"*** Sensitive Data Redacted ***");
        formatter.field("retrieval_configuration", &self.retrieval_configuration);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
impl RetrieveInput {
    /// Creates a new builder-style object to manufacture [`RetrieveInput`](crate::operation::retrieve::RetrieveInput).
    pub fn builder() -> crate::operation::retrieve::builders::RetrieveInputBuilder {
        crate::operation::retrieve::builders::RetrieveInputBuilder::default()
    }
}

/// A builder for [`RetrieveInput`](crate::operation::retrieve::RetrieveInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct RetrieveInputBuilder {
    pub(crate) knowledge_base_id: ::std::option::Option<::std::string::String>,
    pub(crate) retrieval_query: ::std::option::Option<crate::types::KnowledgeBaseQuery>,
    pub(crate) retrieval_configuration: ::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl RetrieveInputBuilder {
    /// Identifier of the KnowledgeBase
    /// This field is required.
    pub fn knowledge_base_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.knowledge_base_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier of the KnowledgeBase
    pub fn set_knowledge_base_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.knowledge_base_id = input;
        self
    }
    /// Identifier of the KnowledgeBase
    pub fn get_knowledge_base_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.knowledge_base_id
    }
    /// Knowledge base input query.
    /// This field is required.
    pub fn retrieval_query(mut self, input: crate::types::KnowledgeBaseQuery) -> Self {
        self.retrieval_query = ::std::option::Option::Some(input);
        self
    }
    /// Knowledge base input query.
    pub fn set_retrieval_query(mut self, input: ::std::option::Option<crate::types::KnowledgeBaseQuery>) -> Self {
        self.retrieval_query = input;
        self
    }
    /// Knowledge base input query.
    pub fn get_retrieval_query(&self) -> &::std::option::Option<crate::types::KnowledgeBaseQuery> {
        &self.retrieval_query
    }
    /// Search parameters for retrieving from knowledge base.
    pub fn retrieval_configuration(mut self, input: crate::types::KnowledgeBaseRetrievalConfiguration) -> Self {
        self.retrieval_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Search parameters for retrieving from knowledge base.
    pub fn set_retrieval_configuration(mut self, input: ::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration>) -> Self {
        self.retrieval_configuration = input;
        self
    }
    /// Search parameters for retrieving from knowledge base.
    pub fn get_retrieval_configuration(&self) -> &::std::option::Option<crate::types::KnowledgeBaseRetrievalConfiguration> {
        &self.retrieval_configuration
    }
    /// Opaque continuation token of previous paginated response.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Opaque continuation token of previous paginated response.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Opaque continuation token of previous paginated response.
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`RetrieveInput`](crate::operation::retrieve::RetrieveInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::retrieve::RetrieveInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::retrieve::RetrieveInput {
            knowledge_base_id: self.knowledge_base_id,
            retrieval_query: self.retrieval_query,
            retrieval_configuration: self.retrieval_configuration,
            next_token: self.next_token,
        })
    }
}
impl ::std::fmt::Debug for RetrieveInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("RetrieveInputBuilder");
        formatter.field("knowledge_base_id", &self.knowledge_base_id);
        formatter.field("retrieval_query", &"*** Sensitive Data Redacted ***");
        formatter.field("retrieval_configuration", &self.retrieval_configuration);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
