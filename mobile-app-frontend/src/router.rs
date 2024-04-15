use yew::{html, Html};
use yew_router::prelude::*;
use yewdux::{functional::use_store};

use crate::{pages::*, state::State};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/auth")]
	Auth,
	
	#[at("/new-tasks")]
	NewTasks,
	#[at("/taken-tasks")]
	TakenTasks,
	#[at("/finished-tasks")]
	FinishedTasks,

	#[at("/new-tasks/:task_id")]
	NewTaskView { task_id: u32 },
	#[at("/taken-tasks/:task_id")]
	TakenTaskView { task_id: u32 },
	#[at("/finished-tasks/:task_id")]
	FinishedTaskView { task_id: u32 },
	
	#[not_found]
	#[at("/404")]
	NotFound,

	#[at("/work-task")]
	WorkTask,
	#[at("/work-instruments")]
	WorkInstruments,
	#[at("/work-materials")]
	WorkMaterials,

	#[at("/finish-auto-operation/:task_id/:operation_id")]
	FinishAutoOperation { task_id: u32, operation_id: u32 },
	#[at("/finish-general-operation/:task_id/:operation_id")]
	FinishGeneralOperation { task_id: u32, operation_id: u32 },
	#[at("/finish-floating-operation/:task_id/:operation_id")]
	FinishFloatingOperation { task_id: u32, operation_id: u32 },

	#[at("/change-finished-operation/:task_id/:operation_id")]
	ChangeFinishedOperation { task_id: u32, operation_id: u32 },
	#[at("/change-pauses/:task_id/:operation_id/:index")]
	ChangePause { task_id: u32, operation_id: u32, index: usize },

	#[at("/cancel-task/:task_id")]
	CancelTask { task_id: u32 },
}

pub fn router(route: Route) -> Html {
	match route {
		Route::Auth => {
			html! {<mobile::Auth/>}
		}
		Route::NewTasks => {
			html! {<mobile::NewTasksList/>}
		}
		Route::TakenTasks => {
			html! {<mobile::TakenTasksList/>}
		}
		Route::FinishedTasks => {
			html! {<mobile::FinishedTasksList/>}
		}
		Route::NotFound => {
			html! {<Redirect<Route> to={Route::Auth}/>}
		}
		Route::FinishAutoOperation { task_id, operation_id } => {
			html! {<mobile::FinishAutoOperation task_id={task_id} operation_id={operation_id}/>}
		}
		Route::FinishGeneralOperation { task_id, operation_id } => {
			html! {<mobile::FinishGeneralOperation task_id={task_id} operation_id={operation_id}/>}
		}
		Route::FinishFloatingOperation { task_id, operation_id } => {
			html! {<mobile::FinishFloatingOperation task_id={task_id} operation_id={operation_id}/>}
		}
		Route::CancelTask { task_id } => html! {<mobile::CancelTask task_id={task_id}/>},
		Route::WorkTask => html! {<mobile::WorkTask/>},
		Route::WorkInstruments => html! {<mobile::WorkInstruments/>},
		Route::WorkMaterials => html! {<mobile::WorkMaterials/>},
		Route::Home => html! {<mobile::HomeRedirector/>},
		Route::FinishedTaskView { task_id } => html! {<mobile::FinishedTaskView task_id={task_id}/>},
		Route::TakenTaskView { task_id } => html! {<mobile::TakenTaskView task_id={task_id}/>},
		Route::NewTaskView { task_id } => html! {<mobile::NewTaskView task_id={task_id}/>},
    	Route::ChangeFinishedOperation { task_id, operation_id } => html! {<mobile::ChangeFinishedOperation task_id={task_id} operation_id={operation_id}/>},
		Route::ChangePause { task_id, operation_id, index } => {
			html! {<mobile::ChangePause task_id={task_id} operation_id={operation_id} index={index}/>}
		}
	}
}