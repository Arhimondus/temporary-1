use chrono::{DateTime, Utc};
use yew::{html, function_component, Html, Properties, use_state, UseStateHandle, use_effect_with_deps, use_effect, Callback};
use yew_hooks::{use_async, use_effect_once, UseAsyncHandle};
use yew_router::{prelude::Link, navigator, hooks::use_navigator};
use yewdux::{prelude::use_store, log::debug, dispatch};
use enclose::enclose;

use crate::{models::*, State, api, router::Route, components, utils::{cancel_all_alarms, relog, ApiError, error_message, success_message}, state::AppStateActions};

#[function_component]
fn BottomNav() -> Html {
	html! {
		<footer class="tabs">
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
		</footer>
	}
}

#[derive(Properties, PartialEq)]
struct TaskRowProps {
	task: Task,
}

#[function_component]
fn TaskRow(props: &TaskRowProps) -> Html {
	let task = props.task.to_owned();
	let start_time = task.start_time();
	let end_time = task.end_time();
	html! {
		<li class="card task-card">
			<a href="/new-tasks/{task.id}">
				<div class="card-content">
					<div class="media">
						<div class="media-content">
							<p class="title is-4">{task.name}</p>
							<p class="subtitle is-6">{start_time}{" - "}{end_time}{" #"}{task.id}</p>
						</div>
					</div>
				</div>
			</a>
		</li>	
	}
}

#[function_component]
pub fn WorkTask() -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();
	let task_menu_show = use_state(|| false);

	let Some(active_task) = state.active_task.clone() else { 
		return html! {
			<>
			<div class="section is-flex is-flex-direction-column is-align-items-center" style="gap: 10px;">
			{"Активной задачи нету!"}
			<Link<Route> to={Route::TakenTasks} classes="button">
				{format!("Перейти в принятые ({})", state.taken_tasks_count)}
			</Link<Route>>
			<Link<Route> to={Route::NewTasks} classes="button">
				{format!("Перейти в новые ({})", state.new_tasks_count)}
			</Link<Route>>
			<Link<Route> to={Route::FinishedTasks} classes="button">
				{format!("Перейти в завершённые ({})", state.finished_tasks_count)}
			</Link<Route>>
			</div>
			</>
		};
	};

	// let clonned_state = state.clone();
	// let new_tasks_list_async = use_async(async move {
	// 	api::new_tasks(&clonned_state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	// });

	// let clonned_new_tasks = new_tasks_list_async.clone();
	// use_effect_once(move || {
	// 	// debug!("Running effect once on mount");
	// 	clonned_new_tasks.run();
	// 	|| debug!("Running clean-up of effect on unmount")
	// });

	// let clonned_new_tasks = new_tasks_list_async.clone();
	// use_effect_with_deps(move |data| {
	// 	if !new_tasks_list_async.loading {
	// 		let dispatch = dispatch.clone();
	// 		dispatch.reduce_mut(|state: &mut State| state.new_tasks = data.data.clone());
	// 	}
	// }, clonned_new_tasks);

	let task_menu_show_action = Callback::from(enclose!((task_menu_show) move |e| {
		task_menu_show.set(!*task_menu_show);
	}));

	let cancel_task_click = Callback::from(enclose!((navigator, active_task) move |_| {
		navigator.push(&Route::CancelTask { task_id: active_task.task.id });
	}));

	let send_all_operations = dispatch.reduce_mut_callback(move |state| {
		let active_task = state.active_task.as_mut().unwrap().clone();
		for or in active_task.operation_results.clone() {
			match or.1.status {
				OperationResultStatus::Open | OperationResultStatus::Pass => {
					let operation = active_task.task.operations.iter().find(|it| it.id == or.0).unwrap();
					state.finish_operation(or.0, operation.plan_count, None, Utc::now());
				},
				OperationResultStatus::Done(done_status) => {
					if let OperationDoneStatus::Successfull = done_status {} else {
						state.operation_upload(or.0);
					}
				},
			};
		}
		// Box::pin(enclose!((mut state) async move {
		// 	api::finish_operations(&&state.user_session_id.unwrap(), &state.active_task.unwrap().operation_results.values().cloned().collect()).await
		// }))
	});

	// let turn_off_task_click = {
	// 	let dispatch = dispatch2.clone();
	// 	Callback::from(move |_| {
	// 		cancel_all_alarms();
	// 		dispatch.reduce_mut(|state| {
	// 			state.active_task = None;
	// 		});
	// 	})
	// };


	// {active_task.current_operation_id}
	// let max_operations = active_task.task.operations.len() as u32;
	// let cur_operations = active_task.operation_results.iter().filter(|or| or.1.status == OperationResultStatus::Pass).count();
	html! {
		<>
		<div style="position: sticky; top: 0; left: 0; background-color: white; z-index: 1500; border-bottom: 1px solid gray;" class="pb-5">
			<h1 class="title is-flex is-justify-content-space-between pt-5 pl-5 pr-5" style="position: relative;">
				<div style="position: absolute; left: 0; top: 0; color: #2c2c2c; font-size: 12px; font-weight: normal;">{active_task.task.id} {" "} {active_task.task.time()}</div>
				<div>{active_task.task.name.clone()} if active_task.task.r#type == TaskType::Floating { <i class="ml-2 fas fa-calculator"></i> }</div>
				<button class="button is-light" onclick={task_menu_show_action}><i class="fa fa-ellipsis-v"></i></button>
				if *task_menu_show {
					<aside class="menu" style="position: absolute; right: 70px; top: 7px; font-weight: normal; background-color: white; border: 1px solid black;">
						<ul class="menu-list">
							// <li><a onclick={turn_off_task_click}><i class="fas fa-power-off mr-2"></i>{"Выключить задачу"}</a></li>
							<li><a onclick={cancel_task_click} style="background-color: #ffb2b2;"><i class="fas fa-minus-circle mr-2" style="color: #dd1111;"></i>{"Завершить задачу"}</a></li>
						</ul>
					</aside>
				}
			</h1>
			// <progress class="progress is-success" value={cur_operations.to_string()} max={max_operations.to_string()}></progress>
		</div>
		
		<ul class="operations-list">
		{active_task.operation_results_order.iter().map(|it| {
			let operation_result = active_task.operation_results.get(&it).unwrap().clone();
			let operation = operation_result.get_operation_from_task(&active_task).clone();
			if operation.floating {
				html! { <components::WorkFloatOperation operation={operation} operation_result={operation_result} can_manual_start={false}/> }
			} else {
				html! { <components::WorkGeneralOperation operation={operation} operation_result={operation_result} can_manual_start={false}/> }
			}
		}).collect::<Html>()}
		</ul>
		// {state.not_done_operations_count()} {" "} {state.not_uploaded_operations_count()}
		// {"RM "}{state.remaining_operations()}
		if state.not_done_operations_count() == 0 && state.not_uploaded_operations_count() > 0 {
			<button class="button" style="align-self: center; margin-top: 10px; margin-left: 10px;" onclick={send_all_operations}>{"Отправить отчёт"}
				// {state.not_done_operations_count()} {" "} {state.not_uploaded_operations_count()}
			</button>
		} else if state.remaining_operations() == 0 {
			<button class="button" style="align-self: center; margin-top: 10px; margin-left: 10px;" onclick={send_all_operations}>{"Отправить отчёт"}
				// {state.not_done_operations_count()} {" "} {state.not_uploaded_operations_count()}
			</button>
		}
		</>
	}
}