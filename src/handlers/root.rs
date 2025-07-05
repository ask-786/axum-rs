use crate::models::api_res::ApiRes;
use axum::Json;

pub async fn root_handler() -> Json<ApiRes> {
    Json(ApiRes {
        success: true,
        data: "Hello World".to_string(),
    })
}
