use crate::entity::prelude::Tasks;

// create時の引数を明示的に型定義する
struct CreateTask {
    title: String,
    is_closed: bool,
}
// Custom する関数を定義
trait CustomTaskRepository {
    fn create_one(&mut self);
}

// implでtraitをstructに注入
impl CustomTaskRepository for Tasks {
    fn create_one(&mut self) {}
}
