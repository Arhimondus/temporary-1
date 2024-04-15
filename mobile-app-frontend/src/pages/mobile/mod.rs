mod finish_general_operation;
pub use finish_general_operation::FinishGeneralOperation;

mod finish_floating_operation;
pub use finish_floating_operation::FinishFloatingOperation;
// mod finished_tasks_list;
// pub use finished_tasks_list::FinishedTasksList;
mod auth;
pub use auth::Auth;

mod new_tasks_list;
pub use new_tasks_list::NewTasksList;
mod new_task_view;
pub use new_task_view::NewTaskView;

// mod finish_all_operations;
// pub use finish_all_operations::FinishAllOperations;

mod taken_tasks_list;
pub use taken_tasks_list::TakenTasksList;
mod finished_tasks_list;
pub use finished_tasks_list::FinishedTasksList;
mod taken_task_view;
pub use taken_task_view::TakenTaskView;

mod work_instruments;
pub use work_instruments::WorkInstruments;
mod work_materials;
pub use work_materials::WorkMaterials;
mod work_task;
pub use work_task::WorkTask;

mod finish_auto_operation;
pub use finish_auto_operation::FinishAutoOperation;

mod finished_task_view;
pub use finished_task_view::FinishedTaskView;

mod cancel_task;
pub use cancel_task::CancelTask;

mod change_finished_operation;
pub use change_finished_operation::ChangeFinishedOperation;

mod change_pause;
pub use change_pause::ChangePause;

mod home_redirector;
pub use home_redirector::HomeRedirector;