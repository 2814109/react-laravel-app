use crate::entity::tasks;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection};
// create時の引数を明示的に型定義する
// pub struct CreateTask {
//     title: String,
// }
// Custom する関数を定義
// trait CustomTaskRepository {
//     fn create_one(create_task: CreateTask);
// }

// implでtraitをstructに注入
// impl CustomTaskRepository for Tasks {
//     async fn create_one(mut create_task: CreateTask) {
//         // entity を使ってinsert処理を実行
//         let new_task = tasks::ActiveModel {
//             id: ActiveValue::NotSet,
//             title: ActiveValue::Set("test".to_owned()),
//             is_closed: ActiveValue::Set(false),
//             created_at: ActiveValue::NotSet,
//             updated_at: ActiveValue::NotSet,
//         };
//         let db: DatabaseConnection =
//             Database::connect("postgresql://postgres:postgres@localhost:5432/postgres".to_string())
//                 .await?;

//         new_task.insert(&db);
//     }
// }

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
