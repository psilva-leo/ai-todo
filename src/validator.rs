use std::collections::HashMap;

use crate::error::AppError;
use axum::{
    Json, async_trait,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extract JSON and handle structural/Enum errors
        let data = match Json::<T>::from_request(req, state).await {
            Ok(Json(d)) => d,
            Err(rejection) => {
                let mut errors = HashMap::new();

                // 1. Try to get the detailed cause (the Serde error)
                // 2. If that's not available, use the top-level rejection message
                let raw_err = std::error::Error::source(&rejection).map_or_else(
                    || rejection.to_string(), // The "Default" (None) case
                    ToString::to_string,
                );

                // 3. Sanitize the "at line X column Y" part
                let clean_msg = raw_err
                    .split(" at line ")
                    .next()
                    .unwrap_or("Invalid JSON format")
                    .to_string();

                errors.insert("payload".to_string(), vec![clean_msg]);
                return Err(AppError::InvalidInput(errors));
            }
        };

        // 2. Validate logic and handle constraint errors
        data.validate().map_err(|err| {
            let field_errors = err
                .field_errors()
                .into_iter()
                .map(|(field, errors)| {
                    let msgs = errors
                        .iter()
                        .map(|e| {
                            e.message
                                .as_ref()
                                .map_or_else(|| e.code.to_string(), ToString::to_string)
                        })
                        .collect();
                    (field.to_string(), msgs)
                })
                .collect();
            AppError::InvalidInput(field_errors)
        })?;

        Ok(ValidatedJson(data))
    }
}
