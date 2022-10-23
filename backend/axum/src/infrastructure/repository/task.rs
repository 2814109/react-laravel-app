use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection};

use crate::entity::{prelude::Tasks, tasks};
// create時の引数を明示的に型定義する
struct CreateTask {
    title: String,
    is_closed: bool,
}
// Custom する関数を定義
trait CustomTaskRepository {
    fn create_one(create_task: CreateTask);
}

// implでtraitをstructに注入
impl CustomTaskRepository for Tasks {
    fn create_one(mut create_task: CreateTask) {
        // entity を使ってinsert処理を実行
        let new_task = tasks::ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::Set("test".to_owned()),
            is_closed: ActiveValue::Set(false),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };

        // let new_task: tasks::Model = new_task.insert().await?;
    }
}
