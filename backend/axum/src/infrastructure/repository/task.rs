use crate::entity::prelude::Tasks;
use crate::entity::tasks;
// use axum::response;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde::Serialize;
#[derive(Serialize)]
struct Task {
    id: i32,
}

pub async fn create_one(title: &str) {
    let utc: DateTime<Utc> = Utc::now();
    // entity を使ってinsert処理を実行
    let new_task = tasks::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(title.to_owned()),
        is_closed: ActiveValue::Set(false),
        created_at: ActiveValue::Set(utc.naive_utc()),
        updated_at: ActiveValue::Set(utc.naive_utc()),
    };
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = new_task.insert(&db).await;
    println!("{:?}", result);
}

pub async fn get_task_by_id() {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = Tasks::find_by_id(5).one(&db).await;
    println!("{:?}", result);

    // return response::Json(Task { id: 11 });
}
