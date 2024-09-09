use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRef, FromRequest, Request},
    Json,
};
use garde::{Unvalidated, Valid, Validate};

use crate::error::ErrorRejection;

pub struct JsonV<T>(pub Valid<T>);

#[async_trait]
impl<State, T, Context> FromRequest<State> for JsonV<T>
where
    State: Send + Sync,
    Context: FromRef<State> + Send + Sync,
    T: Validate<Context = Context>,
    axum::Json<T>: FromRequest<Context, Rejection = JsonRejection>,
{
    type Rejection = ErrorRejection;

    async fn from_request(req: Request, state: &State) -> Result<Self, Self::Rejection> {
        let ctx = FromRef::from_ref(state);

        let Json(value) = Json::<T>::from_request(req, &ctx).await?;
        let value = Unvalidated::new(value).validate_with(&ctx)?;

        Ok(Self(value))
    }
}
