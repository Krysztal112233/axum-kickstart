use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;

pub mod error;

#[derive(Debug, Deserialize, Serialize, Default, ToSchema)]
pub struct Empty {}

#[derive(Debug, Deserialize, Serialize, Default, ToSchema)]
#[serde(transparent)]
pub struct StatusCodeOnlyResponse(ApiResponse<Empty>);

#[derive(Debug, Deserialize, Serialize, Default, ToSchema)]
pub struct ApiResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    payload: Option<T>,
}

pub type RestResult<T> = ::std::result::Result<ApiResponse<T>, Error>;

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(payload: T) -> Self {
        Self {
            payload: Some(payload),
            code: None,
            msg: None,
        }
    }

    pub fn error(code: impl Into<u16>, msg: impl Into<String>) -> Self {
        Self {
            code: Some(code.into()),
            msg: Some(msg.into()),
            payload: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Error> for ApiResponse<()> {
    fn from(value: Error) -> Self {
        match value {
            Error::UnknownWithCode(code, msg) => ApiResponse::error(code, msg),
            _ => ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PageInfo {
    pub has_next: bool,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub page_info: PageInfo,
}

impl<T> PagedResponse<T> {
    pub fn with_entire<I>(data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let items = data.into_iter().collect::<Vec<_>>();

        let page_info = PageInfo {
            has_next: false,
            total: items.len(),
        };

        Self { items, page_info }
    }
}
