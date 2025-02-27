// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Disassociate Agent Knowledge Base Response
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateAgentKnowledgeBaseOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DisassociateAgentKnowledgeBaseOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateAgentKnowledgeBaseOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateAgentKnowledgeBaseOutput`](crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput).
    pub fn builder() -> crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseOutputBuilder {
        crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseOutputBuilder::default()
    }
}

/// A builder for [`DisassociateAgentKnowledgeBaseOutput`](crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateAgentKnowledgeBaseOutputBuilder {
    _request_id: Option<String>,
}
impl DisassociateAgentKnowledgeBaseOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateAgentKnowledgeBaseOutput`](crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput).
    pub fn build(self) -> crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput {
        crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput {
            _request_id: self._request_id,
        }
    }
}
