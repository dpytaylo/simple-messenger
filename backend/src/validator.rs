use api_error_derive::ApiError;
use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json, RequestExt,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = ValidatedJsonError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req.extract::<Json<T>, _>().await?;

        data.validate()?;
        Ok(Self(data))
    }
}

#[derive(ApiError, Debug, Error)]
pub enum ValidatedJsonError {
    #[error("json rejection ({0})")]
    #[status_code(BAD_REQUEST)]
    JsonRejection(#[from] JsonRejection),

    #[error("validation error")]
    #[status_code(BAD_REQUEST)]
    ValidationError(#[from] validator::ValidationErrors),
}
