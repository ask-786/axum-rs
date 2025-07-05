use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::results::InsertOneResult;

use crate::{
    models::{
        api_res::ApiRes,
        user::{CreateUser, User},
    },
    state::app_state::AppState,
};

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<ApiRes<Option<Vec<User>>>>, impl IntoResponse> {
    let collection = state.db.collection::<User>("users");
    let cursor = collection.find(None, None).await.unwrap();

    let users: Vec<User> = match cursor.try_collect().await {
        Ok(users) => users,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error getting users"));
        }
    };

    Ok(Json(ApiRes {
        success: true,
        data: Some(users),
    }))
}

pub async fn get_user_with_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiRes<Option<User>>>, impl IntoResponse> {
    let collection = state.db.collection::<User>("users");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error adding user"));
        }
    };

    let user = match collection.find_one(doc! {"_id": obj_id}, None).await {
        Ok(user) => user,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error adding user"));
        }
    };

    Ok(Json(ApiRes {
        success: true,
        data: user,
    }))
}

pub async fn add_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<ApiRes<InsertOneResult>>, impl IntoResponse> {
    let collection = state.db.collection::<CreateUser>("users");

    let hashed_pass = match bcrypt::hash(payload.password, 10) {
        Ok(pass) => pass,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password"));
        }
    };

    let user = CreateUser {
        username: payload.username,
        name: payload.name,
        email: payload.email,
        phone: payload.phone,
        password: hashed_pass,
    };

    let result = match collection.insert_one(user, None).await {
        Ok(res) => res,
        Err(_) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error adding user"));
        }
    };

    return Ok(Json(ApiRes {
        success: true,
        data: result,
    }));
}
