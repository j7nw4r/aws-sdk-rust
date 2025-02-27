// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>The Amazon Web Services account doesn’t have access to this resource. </p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>The request processing has failed because of an internal error in the service.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The resource was not found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The service quota has been exceeded for this resource.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The request or operation couldn't be performed because a service is throttling requests. The most common source of throttling errors is when you create resources that exceed your service limit for this resource type. Request a limit increase or delete unused resources, if possible.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The resource passed is invalid.</p>
    ValidationException(crate::types::error::ValidationException),
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
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
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
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ServiceQuotaExceededException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::batch_get_token_balance::BatchGetTokenBalanceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::batch_get_token_balance::BatchGetTokenBalanceError, R>,
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
impl From<crate::operation::batch_get_token_balance::BatchGetTokenBalanceError> for Error {
    fn from(err: crate::operation::batch_get_token_balance::BatchGetTokenBalanceError) -> Self {
        match err {
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::batch_get_token_balance::BatchGetTokenBalanceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_asset_contract::GetAssetContractError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_asset_contract::GetAssetContractError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_asset_contract::GetAssetContractError> for Error {
    fn from(err: crate::operation::get_asset_contract::GetAssetContractError) -> Self {
        match err {
            crate::operation::get_asset_contract::GetAssetContractError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_asset_contract::GetAssetContractError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_asset_contract::GetAssetContractError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_asset_contract::GetAssetContractError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::get_asset_contract::GetAssetContractError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_asset_contract::GetAssetContractError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_asset_contract::GetAssetContractError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_token_balance::GetTokenBalanceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_token_balance::GetTokenBalanceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_token_balance::GetTokenBalanceError> for Error {
    fn from(err: crate::operation::get_token_balance::GetTokenBalanceError) -> Self {
        match err {
            crate::operation::get_token_balance::GetTokenBalanceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_token_balance::GetTokenBalanceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_token_balance::GetTokenBalanceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_token_balance::GetTokenBalanceError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::get_token_balance::GetTokenBalanceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_token_balance::GetTokenBalanceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_token_balance::GetTokenBalanceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_transaction::GetTransactionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_transaction::GetTransactionError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_transaction::GetTransactionError> for Error {
    fn from(err: crate::operation::get_transaction::GetTransactionError) -> Self {
        match err {
            crate::operation::get_transaction::GetTransactionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_transaction::GetTransactionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_transaction::GetTransactionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_transaction::GetTransactionError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::get_transaction::GetTransactionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_transaction::GetTransactionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_transaction::GetTransactionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_asset_contracts::ListAssetContractsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_asset_contracts::ListAssetContractsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_asset_contracts::ListAssetContractsError> for Error {
    fn from(err: crate::operation::list_asset_contracts::ListAssetContractsError) -> Self {
        match err {
            crate::operation::list_asset_contracts::ListAssetContractsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_asset_contracts::ListAssetContractsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_asset_contracts::ListAssetContractsError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::list_asset_contracts::ListAssetContractsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_asset_contracts::ListAssetContractsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_asset_contracts::ListAssetContractsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_token_balances::ListTokenBalancesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_token_balances::ListTokenBalancesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_token_balances::ListTokenBalancesError> for Error {
    fn from(err: crate::operation::list_token_balances::ListTokenBalancesError) -> Self {
        match err {
            crate::operation::list_token_balances::ListTokenBalancesError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_token_balances::ListTokenBalancesError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_token_balances::ListTokenBalancesError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::list_token_balances::ListTokenBalancesError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_token_balances::ListTokenBalancesError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_token_balances::ListTokenBalancesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_transaction_events::ListTransactionEventsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_transaction_events::ListTransactionEventsError, R>,
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
impl From<crate::operation::list_transaction_events::ListTransactionEventsError> for Error {
    fn from(err: crate::operation::list_transaction_events::ListTransactionEventsError) -> Self {
        match err {
            crate::operation::list_transaction_events::ListTransactionEventsError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::list_transaction_events::ListTransactionEventsError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_transaction_events::ListTransactionEventsError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::list_transaction_events::ListTransactionEventsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_transaction_events::ListTransactionEventsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_transaction_events::ListTransactionEventsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_transactions::ListTransactionsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_transactions::ListTransactionsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_transactions::ListTransactionsError> for Error {
    fn from(err: crate::operation::list_transactions::ListTransactionsError) -> Self {
        match err {
            crate::operation::list_transactions::ListTransactionsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_transactions::ListTransactionsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_transactions::ListTransactionsError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::list_transactions::ListTransactionsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_transactions::ListTransactionsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_transactions::ListTransactionsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
