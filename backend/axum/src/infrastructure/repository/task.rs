use crate::entity::prelude::Tasks;
use crate::entity::tasks;
use axum::response;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    id: i32,
}

#[derive(Serialize)]
pub struct TaskItem {
    id: i32,
    title: String,
    is_closed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
// #[derive(Serialize)]
// pub struct TaskList(Vec<TaskItem>);

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

pub async fn get_task_by_id() -> response::Json<Task> {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = Tasks::find_by_id(5).one(&db).await;

    let get_id: i32 = match result {
        Ok(Some(tasks)) => tasks.id,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };
    return response::Json(Task { id: get_id });
}

pub async fn get_all_task() -> response::Json<Task> {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let results = Tasks::find().all(&db).await;
    let tasks_data = match results {
        Ok(tasks) => tasks,
        Err(_) => todo!(),
        // Ok(None) => todo!(),
    };
    // println!("{:?}", tasks_data);

    // let results_data =    tasks_data.into_iter().map(|argument| argument.id);

    // let results_data = tasks_data.into_iter().map(|argument| argument.id);
    // println!("{:?}", results_data);

    for value in &tasks_data {
        println!("{:?}", value);
    }
    // return response::Json::<Vec<Tasks>>(&tasks_data.unwrap());

    return response::Json(Task { id: 1 });
}
