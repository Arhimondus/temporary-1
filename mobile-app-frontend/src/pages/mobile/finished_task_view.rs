use std::{rc::Rc, sync::Arc};

use capture_it::capture;
use chrono::Local;
use js_sys::eval;
use log::debug;
use pub_this::pub_this;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle, use_effect_once};
use yewdux::{prelude::*, storage};
use yew_router::prelude::*;
use closure::closure;
use enclose::enclose;
use crate::{utils::{log_str, short_time, scroll_to_operation, register_alarms}, models::{self, Task}, api, State, router::Route, components::{self, SimpleOperation}, state::AppStateActions};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct FinishedTaskViewProps {
	task_id: u32,
}

#[function_component]
pub fn FinishedTaskView(props: &FinishedTaskViewProps) -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();
	let storage: UseLocalStorageHandle<State> = use_local_storage("state".to_string());

	let Some(finished_tasks) = state.finished_tasks.clone() else {
		return html! {<Redirect<Route> to={Route::FinishedTasks}/>};
	};
	let task = Rc::new(finished_tasks.into_iter().find(|t| t.id == props.task_id).unwrap());

	let finished_tasks_list_async = use_async(enclose!((state) async move {
		api::finished_tasks(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));

	use_effect_with_deps(enclose!((finished_tasks_list_async, dispatch) move |data: &Option<Vec<Task>>| {
		if !finished_tasks_list_async.loading && let Some(finished_tasks) = data {
			dispatch.reduce_mut(|state: &mut State| {
				state.finished_tasks = Some(finished_tasks.clone());
				if let Some(finished_tasks) = state.finished_tasks.as_ref() {
					// FIX yR5xgG
					state.finished_tasks_count = finished_tasks.len() as i8;
				}
			});
		}
	}), finished_tasks_list_async.data.clone());

	use_effect_once(enclose!((finished_tasks_list_async) move || {
		finished_tasks_list_async.run();
		|| debug!("Running clean-up of effect on unmount")
	}));

	html! {
		<div class="panel">
			<p class="panel-heading is-flex is-flex-direction-column">
				{task.name.clone()}

				<div class="is-flex is-justify-content-space-between is-align-items-center">
					<div class="content is-small m-0">
						{task.real_time()}
						if let Some(closed_time) = task.closed_time {
							{" (правки до "} {format!("{}",  closed_time.with_timezone(&Local).format("%H:%M"))} {")"} 
						}
					</div>

					<div class="content is-medium m-0">
						{task.plan_salary}{" / "}{task.pass_salary}{"₽"}
					</div>
				</div>
			</p>
			{task.operations.iter().map(|operation| html! {<components::FinishedOperation operation={operation.clone()} task={task.clone()}/> }).collect::<Html>()}
		</div>
	}
}