use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
    Json,
};
use garde::Report;
use http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    error: ErrorResponseInner,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponseInner {
    kind: &'static str,
    uuid: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ErrorRejection {
    code: StatusCode,
    body: ErrorResponse,
}

impl ErrorRejection {
    pub fn new<T>(code: StatusCode, kind: &'static str, details: Option<T>) -> Self
    where
        T: Serialize,
    {
        Self {
            code,
            body: ErrorResponse {
                error: ErrorResponseInner {
                    kind,
                    uuid: Uuid::new_v4(),
                    details: details.map(|d| serde_json::to_value(&d).unwrap_or(Value::Null)),
                },
            },
        }
    }
}

impl IntoResponse for ErrorRejection {
    fn into_response(self) -> Response {
        (self.code, Json(self.body)).into_response()
    }
}

impl From<JsonRejection> for ErrorRejection {
    fn from(value: JsonRejection) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            match value {
                JsonRejection::JsonDataError(_) => "JsonDataError",
                JsonRejection::JsonSyntaxError(_) => "JsonSyntaxError",
                JsonRejection::MissingJsonContentType(_) => "MissingJsonContentType",
                JsonRejection::BytesRejection(_) => "BytesRejection",
                _ => "JsonUnknownError",
            },
            Option::<()>::None,
        )
    }
}

impl From<Report> for ErrorRejection {
    fn from(value: Report) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "ValidationError",
            Some(
                value
                    .into_inner()
                    .into_iter()
                    .map(|(path, error)| {
                        if path.is_empty() {
                            error.to_string()
                        } else {
                            format!("{path}: {error}")
                        }
                    })
                    .collect::<Vec<_>>(),
            ),
        )
    }
}

#[derive(Clone)]
pub struct ApiError {
    rejection: ErrorRejection,
    description: String,
}

impl ApiError {
    pub fn new(
        code: StatusCode,
        kind: &'static str,
        details: Option<Value>,
        description: String,
    ) -> Self {
        Self {
            rejection: ErrorRejection::new(code, kind, details),
            description,
        }
    }
}

pub fn wrap_error<T>(status_code: StatusCode, error: T) -> Response
where
    T: Into<&'static str> + ToString,
{
    let mut response = status_code.into_response();

    let description = error.to_string();
    response
        .extensions_mut()
        .insert(ApiError::new(status_code, error.into(), None, description));

    response
}

pub fn wrap_error_with_details<Error, Details>(
    status_code: StatusCode,
    error: Error,
    details: Option<Details>,
) -> Response
where
    Error: Into<&'static str> + ToString,
    Details: Serialize,
{
    let mut response = status_code.into_response();

    let description = error.to_string();
    response.extensions_mut().insert(ApiError::new(
        status_code,
        error.into(),
        details.map(|val| serde_json::to_value(&val).unwrap_or(Value::Null)),
        description,
    ));

    response
}

pub fn api_error_to_response(error: ApiError) -> Response {
    error!(
        status_code = error.rejection.code.as_u16(),
        body = ?error.rejection.body,
        server_description = error.description,
    );
    error.rejection.into_response()
}
