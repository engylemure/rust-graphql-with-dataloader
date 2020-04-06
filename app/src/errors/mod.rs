use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use graphql_depth_limit::ExceedMaxDepth;
use juniper::graphql_value;
use std::convert::From;
use validator::ValidationErrors;

#[derive(Debug)]
pub struct DuplicateErrorInfo {
    pub origin: String,
    pub info: String,
}

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Duplicate")]
    Duplicate(DuplicateErrorInfo),

    #[display(fmt = "Validation Error")]
    ValidationError(ValidationErrors),

    #[display(fmt = "Unimplemented")]
    Unimplemented,

    MaxDepthLimit(ExceedMaxDepth),
}

impl juniper::IntoFieldError for ServiceError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ServiceError::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),
            ServiceError::Unimplemented => juniper::FieldError::new(
                "This functionality is not implemented yet.",
                graphql_value!({
                    "type": "UNIMPLEMENTED"
                }),
            ),
            ServiceError::Duplicate(error_info) => juniper::FieldError::new(
                error_info.origin,
                graphql_value!({
                    "type": "DUPLICATE_INFO"
                }),
            ),
            // Improve Logic for generating error messages for field validation errors
            ServiceError::ValidationError(_err) => juniper::FieldError::new(
                "Validation Error",
                graphql_value!({
                    "type": "VALIDATION_ERROR"
                }),
            ),
            ServiceError::MaxDepthLimit(err) => {
                let message = format!("{}", err);
                juniper::FieldError::new(
                    "Max Depth Limit",
                    graphql_value!({
                        "type": "MAX_DEPTH_LIMIT",
                         "message": message
                    }),
                )
            }
            _ => juniper::FieldError::new(
                "Unknown Error",
                graphql_value!({
                    "type": "UNKNOWN_ERROR",
                }),
            ),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, _info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    return ServiceError::Duplicate(DuplicateErrorInfo {
                        origin: _info.message().to_string(),
                        info: _info.details().unwrap_or("No Info").to_string(),
                    });
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
