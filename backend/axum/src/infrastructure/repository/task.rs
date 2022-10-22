use crate::entity::prelude::Tasks;

// Custom する関数を定義
trait CustomTaskRepository {
    fn test(&mut self);
}

// implでtraitをstructに注入
impl CustomTaskRepository for Tasks {
    fn test(&mut self) {}
}
