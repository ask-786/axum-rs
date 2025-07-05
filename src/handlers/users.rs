use axum::{
    Json,
    extract::{Path, State},
};
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;

use crate::{
    models::{api_res::ApiRes, user::User},
    state::app_state::AppState,
};

pub async fn get_all_users(State(state): State<AppState>) -> Json<ApiRes<Option<Vec<User>>>> {
    let collection = state.db.collection::<User>("users");
    let cursor = collection.find(None, None).await.unwrap();

    let users: Vec<User> = cursor.try_collect().await.unwrap();

    Json(ApiRes {
        success: true,
        data: Some(users),
    })
}

pub async fn get_user_with_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiRes<Option<User>>> {
    let collection = state.db.collection::<User>("users");

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Json(ApiRes {
                success: false,
                data: None,
            });
        }
    };

    let user = match collection.find_one(doc! {"_id": obj_id}, None).await {
        Ok(user) => user,
        Err(_) => {
            return Json(ApiRes {
                success: false,
                data: None,
            });
        }
    };

    Json(ApiRes {
        success: true,
        data: user,
    })
}
