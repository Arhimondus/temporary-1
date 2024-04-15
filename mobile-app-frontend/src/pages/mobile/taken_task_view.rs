use std::{rc::Rc, sync::Arc};

use capture_it::capture;
use js_sys::eval;
use pub_this::pub_this;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle};
use yewdux::{prelude::*, storage};
use yew_router::prelude::*;
use closure::closure;
use enclose::enclose;
use crate::{utils::{log_str, short_time, scroll_to_operation, register_alarms}, models::{self, TaskType}, api, State, router::Route, components::{self, SimpleOperation}, state::AppStateActions};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct TakenTaskViewProps {
	task_id: u32,
}

#[function_component]
pub fn TakenTaskView(props: &TakenTaskViewProps) -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();
	let storage: UseLocalStorageHandle<State> = use_local_storage("state".to_string());

	let Some(taken_tasks) = state.taken_tasks.clone() else {
		return html! {<Redirect<Route> to={Route::TakenTasks}/>};
	};
	let task = Rc::new(taken_tasks.into_iter().find(|t| t.id == props.task_id).unwrap());

	let run_task = dispatch.reduce_mut_callback(enclose!((task) move |state| {
		state.launch_task_from_taken(&task)
	}));

	html! {
		<div class="panel">
			<p class="panel-heading is-flex is-flex-direction-column">
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
				</div>

				<div class="is-flex is-justify-content-space-between is-align-items-center">
					<div class="content is-small m-0">
						{task.time()}
					</div>

					<div class="content is-medium m-0">
						{task.plan_salary}{" / "}{task.pass_salary}{"₽"}
					</div>
				</div>

				<div class="is-flex is-justify-content-center mt-3">
					if let Some(atask) = &state.active_task && atask.task.id == task.id {
						<Redirect<Route> to={Route::WorkTask}/>
					} else {
						if state.active_task.is_none() {
							<button class="button is-small is-link is-outlined" onclick={run_task}>
								<i class="fas fa-play mr-2"></i>{"Запустить"}
							</button>
						} else {
							<span style="font-size: 14px; color: #dc3a3a;">{"Чтобы начать эту задачу, завершите активную"}</span>
						}
					}
				</div>
			</p>
			{task.operations.iter().map(|o| html! {
				<SimpleOperation operation={o.clone()}/>
			}).collect::<Html>()}
		</div>
	}
}