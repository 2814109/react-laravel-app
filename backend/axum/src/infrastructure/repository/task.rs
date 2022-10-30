use crate::entity::prelude::Tasks;
use crate::entity::tasks;
use axum::response;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, JsonValue};
use serde::{Serialize, Deserialize};

use axum::{
    extract::Path,
};
use axum::{
    Json,
};
// use std::collections::HashMap;


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

#[derive(Serialize)]
pub struct TaskList {
    pub task_list: Vec<JsonValue>,
}


#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct LogicalDeleteTask {
    pub id: i32,
}



#[derive(Deserialize)]
pub struct UpdateTask {
    pub id: i32,
    pub title: String,
}

pub async fn create_one(title: &str) {
    println!("{:?}", title);
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
    // let urlPathParameterId = id.to_string();
    
    // let intId =  as i32;
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


pub async fn get_all_task() -> response::Json<TaskList> {
    let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let results = Tasks::find().into_json().all(&db).await;
    return response::Json(TaskList { task_list: results.unwrap() });
}


pub async fn get_id(Path(id) :Path<String>) -> response::Json<Task> {

 println!("{:?}", id);

 let int_id:i32 = id.parse().unwrap();
 
        let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");

    let result = Tasks::find_by_id(int_id).one(&db).await;

    let get_id: i32 = match result {
        Ok(Some(tasks)) => tasks.id,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };

     println!("get id {:?}",get_id);

    return response::Json(Task { id: get_id });
    
}


pub async fn create_one_task (Json(payload): Json<CreateTask>) {
    let body = payload;

     println!("request body {:?}",body.title);

    let utc: DateTime<Utc> = Utc::now();
    // entity を使ってinsert処理を実行
    let new_task = tasks::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set("title".to_owned()),
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

pub async fn logical_delete_for_task (Json(payload) : Json<LogicalDeleteTask>){
    let body = payload;
    let int_id:i32 = body.id;

        let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");


    let target_task = Tasks::find_by_id(int_id).one(&db).await;
      let utc: DateTime<Utc> = Utc::now();

    let logical_delete_task = match target_task {
        Ok(Some(tasks)) => tasks,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };

    let target_task = tasks::ActiveModel {
           id: ActiveValue::Set(logical_delete_task.id),
        title: ActiveValue::Set(logical_delete_task.title),
        is_closed: ActiveValue::Set(true),
        created_at: ActiveValue::Set(logical_delete_task.created_at),
        updated_at: ActiveValue::Set(utc.naive_utc()),
    };
  

    let result = target_task.update(&db).await;
    println!("logical delete {:?}", result);

}



pub async fn update_task (Json(payload) : Json<UpdateTask>){
    let body = payload;
    let int_id:i32 = body.id;
    let title: String = body.title;

        let db: DatabaseConnection =
        Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
            .await
            .expect("Database connection failed");


    let target_task = Tasks::find_by_id(int_id).one(&db).await;
      let utc: DateTime<Utc> = Utc::now();

    let update_task = match target_task {
        Ok(Some(tasks)) => tasks,
        Err(_) => todo!(),
        Ok(None) => todo!(),
    };

    let target_task = tasks::ActiveModel {
           id: ActiveValue::Set(update_task.id),
        title: ActiveValue::Set(title),
        is_closed: ActiveValue::Set(update_task.is_closed),
        created_at: ActiveValue::Set(update_task.created_at),
        updated_at: ActiveValue::Set(utc.naive_utc()),
    };
  

    let result = target_task.update(&db).await;
    println!("update {:?}", result);

}