#![feature(let_chains)]
#![feature(async_fn_in_trait)]
#![feature(try_blocks)]
#![feature(async_closure)]

use std::panic;
use std::rc::Rc;

use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Utc;
use js_sys::eval;
use log::{debug, trace};
use pub_this::pub_this;
use serde::Deserialize;
use serde::Serialize;
use yew::{prelude::*};
use yew_hooks::UseAsyncHandle;
use yew_hooks::UseLocalStorageHandle;
use yew_hooks::use_async;
use yew_hooks::use_effect_once;
use yew_hooks::use_local_storage;
use yew_router::navigator;
use yewdux::dispatch;
use yewdux::prelude::*;
use yew_router::prelude::*;
use state::State;

use crate::api::TasksCount;
use crate::components::ActiveOperationActions;
use crate::pages::*;
use crate::models::*;
use crate::router::{Route, router};
use crate::state::AppStateActions;
use crate::utils::clear_app_storage;
use crate::utils::get_first_passed_operation_id;
use crate::utils::get_second_passed_operation_id;
use crate::utils::load_from_app_storage;
use crate::utils::log_str;
use crate::utils::register_alarms_from_alarms_vec;
use crate::utils::relog;
use crate::utils::restore_alarms;
use crate::utils::scroll_to_operation;
use crate::utils::success_message;
use enclose::enclose;

mod pages;
mod models;
mod components;
mod utils;
mod router;
mod api;
mod state;

#[macro_use]
extern crate dotenv_codegen;


#[derive(Properties, PartialEq)]
pub struct BetaProps {
	pub id: u64,
}


#[function_component]
fn TopNav() -> Html {
	let route: Route = use_route().unwrap();
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let tasks_count_async = use_async(enclose!((state) async move {
		api::tasks_count(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));
	
	use_effect_once(enclose!((tasks_count_async) move || {
		panic::set_hook(Box::new(console_error_panic_hook::hook));
		wasm_log::init(wasm_log::Config::default());
		tasks_count_async.run();
		|| {}
	}));

	use_effect_with_deps(enclose!((state, dispatch, tasks_count_async) move |data: &Option<TasksCount>| {
		if let Some(data) = data {
			dispatch.reduce_mut(|state: &mut State| {
				// FIX bqsP6j
				state.taken_tasks_count = (data.taken_tasks as i8 - if state.active_task.is_some() { 1_i8 } else { 0_i8 }) as i8;
				state.new_tasks_count = data.new_tasks as i8;
				state.finished_tasks_count = data.finished_tasks as i8;
			});

			if state.active_task.is_none() && data.new_tasks > 0 {
				navigator.push(&Route::NewTasks);
			}
		}
	}), tasks_count_async.data.clone());
	
	html! {
		<nav class="tabs">
			<ul>
				<li class={if let Route::WorkTask | Route::WorkInstruments |Route::WorkMaterials | Route::FinishFloatingOperation { task_id: _, operation_id: _ } | Route::FinishGeneralOperation { task_id: _, operation_id: _ } = route {
					"is-active"
				} else {""}}><Link<Route> to={Route::WorkTask}>
					if state.active_task.is_none() {
						<i class="fas fa-tools"></i>
						<span style="color: gray;">{"Активная (0)"}</span>
					} else {
						<i class="fas fa-tools"></i>
						<span>{"Активная (1)"}</span>
					}
				</Link<Route>></li>
				<li class={if let Route::TakenTasks | Route::TakenTaskView { task_id: _ } = route {
					"is-active"
				} else {""}}><Link<Route> to={Route::TakenTasks}>
					<i class="fa fa-briefcase mr-2"></i>{"Принятые"}
					{" ("}{state.taken_tasks_count}{")"}
				</Link<Route>></li>
				<li class={format!("{} {}", if let Route::NewTasks | Route::NewTaskView { task_id: _ } = route {
					"is-active"
				} else {""}, if state.new_tasks_count > 0 { "tab-highlight" } else { "" })}><Link<Route> to={Route::NewTasks}>
					<i class="fa fa-cart-plus mr-2"></i>{"Новые"}
					{" ("}{state.new_tasks_count}{")"}
				</Link<Route>></li>
				<li class={if let Route::FinishedTasks | Route::FinishedTaskView { task_id: _ } = route {
					"is-active"
				} else {""}}><Link<Route> to={Route::FinishedTasks}>
					<i class="fas fa-calendar-check mr-2"></i>{"Завершённые"}
					{" ("}{state.finished_tasks_count}{")"}
				</Link<Route>></li>
				<li class={if let Route::Auth = route {
					"is-active"
				} else {""}}><Link<Route> to={Route::Auth}>
					<div class="is-flex is-flex-direction-column is-justify-content-center" style="height: 24px;"><i class="fas fa-user"></i></div>
				</Link<Route>></li>
			</ul>
		</nav>
	}
}

#[function_component]
fn BottomNav() -> Html {
	let route: Route = use_route().unwrap();
	let html = html! {
		<footer>
			<ActiveOperationActions/>
			<div class="tabs" style="width: 100vw;">
				<ul>
					<li><Link<Route> to={Route::WorkTask}>
						{"Задача"}
					</Link<Route>></li>
					<li><Link<Route> to={Route::WorkInstruments}>
						{"Инструменты"}
					</Link<Route>></li>
					<li><Link<Route> to={Route::WorkMaterials}>
						{"Материалы"}
					</Link<Route>></li>
				</ul>
			</div>
		</footer>
	};

	match route {
		Route::Home => html!{<></>},
		Route::Auth => html!{<></>},
		Route::NewTasks => html!{<></>},
		Route::TakenTasks => html!{<></>},
		Route::FinishedTasks => html!{<></>},

		Route::NewTaskView { task_id } => html!{<></>},
		Route::TakenTaskView { task_id } => html!{<></>},
		Route::FinishedTaskView { task_id } => html!{<></>},

		Route::FinishGeneralOperation { task_id, operation_id } => html!{<></>},
		Route::NotFound => html!{"404"}, // FIX mMrL8e
		Route::WorkTask => html,
		Route::WorkInstruments => html,
		Route::WorkMaterials => html,
		Route::FinishFloatingOperation { task_id, operation_id } => html!{<></>},
		Route::FinishAutoOperation { task_id, operation_id } => html!{<></>},

		Route::CancelTask { task_id } => html!{<></>},
		Route::ChangeFinishedOperation { task_id, operation_id } => html!{<></>},
		Route::ChangePause { task_id, operation_id, index } => html!{<></>},
	}
}

#[derive(Properties, PartialEq)]
pub struct LoadingHTTPTaskOperationProps {
	operation_result: OperationResult,
}

#[function_component]
pub fn LoadingHTTPTaskOperation(props: &LoadingHTTPTaskOperationProps) -> Html {
	let operation_result = &props.operation_result;
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let finish_operation_async = use_async(enclose!((state, operation_result) async move {
		api::finish_operation(&state.user_session_id.as_ref().unwrap(), &operation_result).await.or_else(|e| Err(e.error))
	}));

	use_effect_with_deps(enclose!((finish_operation_async, operation_result) move |_| {
		if let OperationResultStatus::Done(done_status) = operation_result.status {
			if done_status == OperationDoneStatus::Uploading {
				finish_operation_async.run();
			}
		}
	}), operation_result.status.clone());

	// use_effect_with_deps(enclose!((dispatch, navigator, finish_operation_async, operation_result) move |err| {
	// 	log_str("nnnn___-1");
	// 	if let Some(error) = err {
			
	// 	}
	// }), finish_operation_async.error.clone());

	use_effect_with_deps(enclose!((state, dispatch, navigator, operation_result) move |handle: &UseAsyncHandle<bool, String>| {
		log_str(&format!("nnnn___-0.5 d {:#?}", handle.data));
		log_str(&format!("nnnn___-0.5 e {:#?}", handle.error));

		if let Some(error) = &handle.error {
			dispatch.reduce_mut(|state| {
				state.operation_failure_upload(operation_result.operation_id);
			});
			navigator.push(&Route::WorkTask);
		} else if let Some(data) = handle.data {
			log_str("nnnn___-2");
			dispatch.reduce_mut(|state| {
				state.operation_success_upload(operation_result.operation_id);
			});
		}
		

		// FIX it it it!
		// let active_task = state.active_task.as_ref().unwrap();
		// let first_passed_operation_id = get_first_passed_operation_id(&active_task.operation_results, &active_task.operation_results_order);
		// log_str("first_passed_operation_id");
		// log_str(&format!("{:#?}", first_passed_operation_id));

		// if let Some(first_passed_operation_id) = first_passed_operation_id {
		// 	if let Some(current_operation_id) = active_task.current_operation_id {
		// 		if first_passed_operation_id != current_operation_id {
		// 			navigator.push(&Route::FinishAutoOperation { task_id: active_task.task.id, operation_id: first_passed_operation_id });
		// 		}
		// 	} else {
		// 		navigator.push(&Route::FinishAutoOperation { task_id: active_task.task.id, operation_id: first_passed_operation_id });
		// 	}
		// } else {
		// 	navigator.push(&Route::WorkTask);
		// }
	}), finish_operation_async);

	return html! {
		<li>{"OP "}{operation_result.operation_id}</li>
	};
}

#[function_component]
pub fn LoadingHTTPTaskOperations() -> Html {
	let (state, dispatch) = use_store::<State>();

	let Some(active_task) = state.active_task.as_ref() else {
		return html!("");
	};

	let operation_results = active_task.operation_results.iter().map(|it| it.1).filter(|it| {
		if let OperationResultStatus::Done(done_status) = &it.status {
			if let OperationDoneStatus::Uploading = done_status {
				true
			} else {
				false
			}
		} else {
			false
		}
	}).collect::<Vec<_>>();

	return html! {
		<ul>
			{operation_results.iter().map(|or| {
				let or = (*or).clone();
				html!(<LoadingHTTPTaskOperation operation_result={or}/>)
			}).collect::<Vec<_>>()}
		</ul>
	};
}



#[function_component]
fn App() -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator();

	let onpaste = {
		let navigator = navigator.clone();
		Callback::from(enclose!((dispatch, navigator) move |e| {
			// navigator = navigator.clone();
			let operation_id: u32 = eval("getOperationId()").unwrap().as_f64().unwrap() as u32;
			let finish_auto_operation: Option<String> = eval("getFinishAutoOperation()").unwrap().as_string();
			log_str(&format!("finish_auto_operation {:#?}", finish_auto_operation));
			relog(&format!("onpaste operation_id = {}", operation_id));
			if let Some(finish_auto_operation) = finish_auto_operation {
				// let raw_action: RawAction = serde_json::from_str(&finish_auto_operation).unwrap();
			
				// navigator.unwrap().push(&Route::FinishAutoOperation {
				// 	task_id: raw_action.task_id.unwrap(),
				// 	operation_id: raw_action.operation_id.unwrap(),
				// });
			} else {
				if operation_id != 0 {
					scroll_to_operation(operation_id);
					dispatch.reduce_mut(|state| state.start_operation_with_time(operation_id, chrono::Utc::now()));
				} else {
					dispatch.reduce_mut(|state| state.no_operation());
				}
			}
		}))
	};

	// use_effect_once(enclose!((dispatch) move || {
	// 	relog("app_loaded");

		// if let Some((operation_id, start_time)) = restore_alarms() {
		// 	scroll_to_operation(operation_id);
		// 	relog("app_loaded successfull restored");
		// 	dispatch.reduce_mut(|state| state.start_operation_with_time(operation_id, start_time));
		// } else {
		// 	relog("app_loaded no operation");
		// 	dispatch.reduce_mut(|state| state.no_operation())
		// }
		
		// if let Some(raw_action) = load_from_app_storage::<RawAction>("js_action".into()) {
		// 	relog(&format!("app_loaded raw_action = {}", serde_json::to_string(&raw_action).unwrap()));
		// 	clear_app_storage("js_action".into());
		// 	if let ActionType::StartOperation = raw_action.r#type {
		// 		let operation_id = raw_action.operation_id.unwrap();
		// 		relog(&format!("app_loaded start operation = {}", operation_id));
		// 		if operation_id != 0 {
		// 			scroll_to_operation(operation_id);
		// 			dispatch.reduce_mut(|state| state.start_operation_with_time(operation_id, chrono::Utc::now()));
		// 		} else {
		// 			dispatch.reduce_mut(|state| state.no_operation());
		// 		}
		// 	}
		// } else {
		// 	if let Some(alarms) = load_from_app_storage::<Vec<Alarm>>("alarms".into()) {
		// 		register_alarms_from_alarms_vec(&alarms);
		// 	}
		// }
		// || {}
	// }));

	use_effect_with_deps(enclose!((dispatch, state) move |(not_done_operations_count, not_uploaded_operations_count)| {
		let not_done_operations_count = state.not_done_operations_count();
		let not_uploaded_operations_count = state.not_uploaded_operations_count();
		// log_str(&format!("ABRAHAM_GORE22C={not_done_operations_count}, {not_uploaded_operations_count}"));
		if not_done_operations_count == 0 && not_uploaded_operations_count == 0 {
			dispatch.reduce_mut(|state| {
				state.active_task = None;
				state.finished_tasks_count += 1;
				success_message("Завершение", "Вы завершили задачу");
			});
		}
	}), (state.not_done_operations_count(), state.not_uploaded_operations_count()));

	html! {
		<main onpaste={onpaste}>
			<HashRouter>
				if state.user_session_id.is_some() {
					<TopNav/>
				}
				<section>
					<Switch<Route> render={router} />
				</section>
				<BottomNav/>
				<LoadingHTTPTaskOperations/>
				// {"NOTUPL"}{state.not_uploaded_operations_count()}
			</HashRouter>
		</main>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}