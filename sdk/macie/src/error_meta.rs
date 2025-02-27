// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>(Discontinued) You do not have required permissions to access the requested resource.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>(Discontinued) Internal server error.</p>
    InternalException(crate::types::error::InternalException),
    /// <p>(Discontinued) The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInputException(crate::types::error::InvalidInputException),
    /// <p>(Discontinued) The request was rejected because it attempted to create resources beyond the current Amazon Web Services account quotas. The error code describes the quota exceeded.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalException(inner) => inner.fmt(f),
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::Unhandled(_) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl From<::aws_smithy_types::error::operation::BuildError> for Error {
    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {
        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: value.into(),
            meta: ::std::default::Default::default(),
        })
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
        match self {
            Self::AccessDeniedException(inner) => inner.meta(),
            Self::InternalException(inner) => inner.meta(),
            Self::InvalidInputException(inner) => inner.meta(),
            Self::LimitExceededException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::associate_member_account::AssociateMemberAccountError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::associate_member_account::AssociateMemberAccountError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::associate_member_account::AssociateMemberAccountError> for Error {
    fn from(err: crate::operation::associate_member_account::AssociateMemberAccountError) -> Self {
        match err {
            crate::operation::associate_member_account::AssociateMemberAccountError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::associate_member_account::AssociateMemberAccountError::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::operation::associate_member_account::AssociateMemberAccountError::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::operation::associate_member_account::AssociateMemberAccountError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::associate_s3_resources::AssociateS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::associate_s3_resources::AssociateS3ResourcesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::associate_s3_resources::AssociateS3ResourcesError> for Error {
    fn from(err: crate::operation::associate_s3_resources::AssociateS3ResourcesError) -> Self {
        match err {
            crate::operation::associate_s3_resources::AssociateS3ResourcesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::associate_s3_resources::AssociateS3ResourcesError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::associate_s3_resources::AssociateS3ResourcesError::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::operation::associate_s3_resources::AssociateS3ResourcesError::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::operation::associate_s3_resources::AssociateS3ResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::disassociate_member_account::DisassociateMemberAccountError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::disassociate_member_account::DisassociateMemberAccountError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::disassociate_member_account::DisassociateMemberAccountError> for Error {
    fn from(err: crate::operation::disassociate_member_account::DisassociateMemberAccountError) -> Self {
        match err {
            crate::operation::disassociate_member_account::DisassociateMemberAccountError::InternalException(inner) => {
                Error::InternalException(inner)
            }
            crate::operation::disassociate_member_account::DisassociateMemberAccountError::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::operation::disassociate_member_account::DisassociateMemberAccountError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError> for Error {
    fn from(err: crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError) -> Self {
        match err {
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError::InvalidInputException(inner) => {
                Error::InvalidInputException(inner)
            }
            crate::operation::disassociate_s3_resources::DisassociateS3ResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_member_accounts::ListMemberAccountsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_member_accounts::ListMemberAccountsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_member_accounts::ListMemberAccountsError> for Error {
    fn from(err: crate::operation::list_member_accounts::ListMemberAccountsError) -> Self {
        match err {
            crate::operation::list_member_accounts::ListMemberAccountsError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::list_member_accounts::ListMemberAccountsError::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::operation::list_member_accounts::ListMemberAccountsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_s3_resources::ListS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_s3_resources::ListS3ResourcesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_s3_resources::ListS3ResourcesError> for Error {
    fn from(err: crate::operation::list_s3_resources::ListS3ResourcesError) -> Self {
        match err {
            crate::operation::list_s3_resources::ListS3ResourcesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_s3_resources::ListS3ResourcesError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::list_s3_resources::ListS3ResourcesError::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::operation::list_s3_resources::ListS3ResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_s3_resources::UpdateS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_s3_resources::UpdateS3ResourcesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::update_s3_resources::UpdateS3ResourcesError> for Error {
    fn from(err: crate::operation::update_s3_resources::UpdateS3ResourcesError) -> Self {
        match err {
            crate::operation::update_s3_resources::UpdateS3ResourcesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_s3_resources::UpdateS3ResourcesError::InternalException(inner) => Error::InternalException(inner),
            crate::operation::update_s3_resources::UpdateS3ResourcesError::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::operation::update_s3_resources::UpdateS3ResourcesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::InternalException(inner) => inner.source(),
            Error::InvalidInputException(inner) => inner.source(),
            Error::LimitExceededException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::InternalException(e) => e.request_id(),
            Self::InvalidInputException(e) => e.request_id(),
            Self::LimitExceededException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
