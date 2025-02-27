// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>A conflict has occurred.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>An internal error has occurred.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The resource is not available.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The request is not authorized.</p>
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service. </p>
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
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
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
            Self::ConflictException(inner) => inner.meta(),
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::UnauthorizedException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_resource_permission::DeleteResourcePermissionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_resource_permission::DeleteResourcePermissionError, R>,
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
impl From<crate::operation::delete_resource_permission::DeleteResourcePermissionError> for Error {
    fn from(err: crate::operation::delete_resource_permission::DeleteResourcePermissionError) -> Self {
        match err {
            crate::operation::delete_resource_permission::DeleteResourcePermissionError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::delete_resource_permission::DeleteResourcePermissionError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::delete_resource_permission::DeleteResourcePermissionError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::delete_resource_permission::DeleteResourcePermissionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::deregister_application::DeregisterApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::deregister_application::DeregisterApplicationError, R>,
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
impl From<crate::operation::deregister_application::DeregisterApplicationError> for Error {
    fn from(err: crate::operation::deregister_application::DeregisterApplicationError) -> Self {
        match err {
            crate::operation::deregister_application::DeregisterApplicationError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::deregister_application::DeregisterApplicationError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::operation::deregister_application::DeregisterApplicationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::deregister_application::DeregisterApplicationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_application::GetApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_application::GetApplicationError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_application::GetApplicationError> for Error {
    fn from(err: crate::operation::get_application::GetApplicationError) -> Self {
        match err {
            crate::operation::get_application::GetApplicationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_application::GetApplicationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_application::GetApplicationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_component::GetComponentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_component::GetComponentError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_component::GetComponentError> for Error {
    fn from(err: crate::operation::get_component::GetComponentError) -> Self {
        match err {
            crate::operation::get_component::GetComponentError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_component::GetComponentError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::operation::get_component::GetComponentError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_component::GetComponentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_database::GetDatabaseError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_database::GetDatabaseError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_database::GetDatabaseError> for Error {
    fn from(err: crate::operation::get_database::GetDatabaseError) -> Self {
        match err {
            crate::operation::get_database::GetDatabaseError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_database::GetDatabaseError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_database::GetDatabaseError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_operation::GetOperationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_operation::GetOperationError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_operation::GetOperationError> for Error {
    fn from(err: crate::operation::get_operation::GetOperationError) -> Self {
        match err {
            crate::operation::get_operation::GetOperationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_operation::GetOperationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_operation::GetOperationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_resource_permission::GetResourcePermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_resource_permission::GetResourcePermissionError, R>,
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
impl From<crate::operation::get_resource_permission::GetResourcePermissionError> for Error {
    fn from(err: crate::operation::get_resource_permission::GetResourcePermissionError) -> Self {
        match err {
            crate::operation::get_resource_permission::GetResourcePermissionError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::get_resource_permission::GetResourcePermissionError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_resource_permission::GetResourcePermissionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_resource_permission::GetResourcePermissionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_applications::ListApplicationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_applications::ListApplicationsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_applications::ListApplicationsError> for Error {
    fn from(err: crate::operation::list_applications::ListApplicationsError) -> Self {
        match err {
            crate::operation::list_applications::ListApplicationsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_applications::ListApplicationsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_applications::ListApplicationsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_applications::ListApplicationsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_components::ListComponentsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_components::ListComponentsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_components::ListComponentsError> for Error {
    fn from(err: crate::operation::list_components::ListComponentsError) -> Self {
        match err {
            crate::operation::list_components::ListComponentsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_components::ListComponentsError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_components::ListComponentsError::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::operation::list_components::ListComponentsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_components::ListComponentsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_databases::ListDatabasesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_databases::ListDatabasesError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_databases::ListDatabasesError> for Error {
    fn from(err: crate::operation::list_databases::ListDatabasesError) -> Self {
        match err {
            crate::operation::list_databases::ListDatabasesError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_databases::ListDatabasesError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_databases::ListDatabasesError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_databases::ListDatabasesError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_operations::ListOperationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_operations::ListOperationsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_operations::ListOperationsError> for Error {
    fn from(err: crate::operation::list_operations::ListOperationsError) -> Self {
        match err {
            crate::operation::list_operations::ListOperationsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_operations::ListOperationsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_operations::ListOperationsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_resource_permission::PutResourcePermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_resource_permission::PutResourcePermissionError, R>,
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
impl From<crate::operation::put_resource_permission::PutResourcePermissionError> for Error {
    fn from(err: crate::operation::put_resource_permission::PutResourcePermissionError) -> Self {
        match err {
            crate::operation::put_resource_permission::PutResourcePermissionError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::put_resource_permission::PutResourcePermissionError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::put_resource_permission::PutResourcePermissionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::put_resource_permission::PutResourcePermissionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_application::RegisterApplicationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::register_application::RegisterApplicationError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::register_application::RegisterApplicationError> for Error {
    fn from(err: crate::operation::register_application::RegisterApplicationError) -> Self {
        match err {
            crate::operation::register_application::RegisterApplicationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::register_application::RegisterApplicationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::register_application::RegisterApplicationError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::register_application::RegisterApplicationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::register_application::RegisterApplicationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_application_refresh::StartApplicationRefreshError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_application_refresh::StartApplicationRefreshError, R>,
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
impl From<crate::operation::start_application_refresh::StartApplicationRefreshError> for Error {
    fn from(err: crate::operation::start_application_refresh::StartApplicationRefreshError) -> Self {
        match err {
            crate::operation::start_application_refresh::StartApplicationRefreshError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::start_application_refresh::StartApplicationRefreshError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::start_application_refresh::StartApplicationRefreshError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::start_application_refresh::StartApplicationRefreshError::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::operation::start_application_refresh::StartApplicationRefreshError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::start_application_refresh::StartApplicationRefreshError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_application_settings::UpdateApplicationSettingsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_application_settings::UpdateApplicationSettingsError, R>,
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
impl From<crate::operation::update_application_settings::UpdateApplicationSettingsError> for Error {
    fn from(err: crate::operation::update_application_settings::UpdateApplicationSettingsError) -> Self {
        match err {
            crate::operation::update_application_settings::UpdateApplicationSettingsError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::update_application_settings::UpdateApplicationSettingsError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::update_application_settings::UpdateApplicationSettingsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::update_application_settings::UpdateApplicationSettingsError::UnauthorizedException(inner) => {
                Error::UnauthorizedException(inner)
            }
            crate::operation::update_application_settings::UpdateApplicationSettingsError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::update_application_settings::UpdateApplicationSettingsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ConflictException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::UnauthorizedException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::UnauthorizedException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
