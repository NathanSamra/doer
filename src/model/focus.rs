use crate::model::task::SharedTask;

#[derive(Clone)]
pub struct Focus {
    #[allow(dead_code)]
    task: SharedTask,
}
