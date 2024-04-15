use std::{rc::Rc, collections::HashMap};

use js_sys::eval;
use pub_this::pub_this;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle};
use yewdux::{prelude::*, storage};
use yew_router::prelude::*;

use crate::{utils::{log_str, short_time, register_alarms}, models::{self, Alarm, OperationResult, TaskType}, api, State, router::Route, components::SimpleOperation, state::{ActiveTask, AppStateActions}};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct TakenTaskViewProps {
	task_id: u32,
}

#[function_component]
pub fn NewTaskView(props: &TakenTaskViewProps) -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();
	let storage: UseLocalStorageHandle<State> = use_local_storage("state".to_string());

	let Some(new_tasks) = state.new_tasks.clone() else {
		return html! {<Redirect<Route> to={Route::NewTasks}/>};
	};
	let task = new_tasks.into_iter().find(|t| t.id == props.task_id).unwrap();

	let clonned_state = state.as_ref().clone();
	let accept_task_async = use_async(async move {
		api::accept_task(&clonned_state.user_session_id.unwrap(), task.id).await.or_else(|e| Err(e.error))
	});

	let clonned_state = state.as_ref().clone();
	let decline_task_async = use_async(async move {
		api::decline_task(&clonned_state.user_session_id.unwrap(), task.id).await.or_else(|e| Err(e.error))
	});

	let clonned_task = task.clone();
	let loading = accept_task_async.to_owned().clone().loading.clone();
	if !loading {
		if let Some(data) = accept_task_async.data.clone() {
			if data {
				// dispatch.reduce_mut(|state| state.launch_task_from_new(&clonned_task));
				dispatch.reduce_mut(|state: &mut State| {
					state.new_tasks_count -= 1;
				});
				return html! {<Redirect<Route> to={Route::TakenTasks}/>};
				// return html! {<Redirect<Route> to={Route::WorkTask}/>};
			}
		}
	}

	let loading = decline_task_async.to_owned().clone().loading.clone();
	if !loading {
		if let Some(data) = decline_task_async.data.clone() {
			if data {
				dispatch.reduce_mut(|state: &mut State| {
					state.new_tasks_count -= 1;
				});
				return html! {<Redirect<Route> to={Route::NewTasks}/>};
			}
		}
	}

	let accept_task = {
		Callback::from(move |_e| {
			accept_task_async.run();
		})
	};

	let decline_task = {
		Callback::from(move |_e| {
			decline_task_async.run();
		})
	};
	
	// use_effect_with_deps(move |data| {
	// 	let dispatch = dispatch.clone(); 
	// 	if let Some(data) = data {
	// 		dispatch.reduce_mut(|state: &mut State| state.user_session_id = data.to_string());
	// 		storage.set(data.to_string());
	// 		navigator.push(&Route::NewTasks);
	// 	}
	// }, decline_task_async.data.clone());

	html! {
		<div class="panel">
			<p class="panel-heading is-flex is-justify-content-space-between is-align-items-center">
				<div>
					{task.name.clone()} 
					if task.r#type == TaskType::General { 
						<span class="ml-2">
							{task.operations.iter().filter(|o| !o.floating).count()}{" "}<i class="fas fa-clock fa-xs"></i>
						</span>
					}
					if task.r#type == TaskType::Floating { 
						<span class="ml-2">
							{task.operations.iter().filter(|o| o.floating).count()}{" "}<i class="fas fa-calculator fa-xs"></i>
						</span>
					}
					if task.r#type == TaskType::Mixed {
						<span class="ml-2">
							{task.operations.iter().filter(|o| !o.floating).count()}{" "}<i class="fas fa-clock fa-xs mr-1"></i>
							{task.operations.iter().filter(|o| o.floating).count()}{" "}<i class="fas fa-calculator fa-xs"></i>
						</span>
					}

					<div class="content is-small">
						{task.time()}
					</div>
				</div>
				{task.plan_salary}{"₽"}
			</p>

			{task.operations.into_iter().map(|o| html! {
				<SimpleOperation operation={o}/>
			}).collect::<Html>()}
			
			<div class="panel-block columns">
				<div class="column">
					<button class="button is-link is-out2lined is-fullwidth is-success" onclick={accept_task}>
						{"Принять"}
					</button>
				</div>
				<div class="column">
					<button class="button is-link is-outl2ined is-fullwidth is-warning" onclick={decline_task}>
						{"Отклонить"}
					</button>
				</div>
			</div>
		</div>
	}
}