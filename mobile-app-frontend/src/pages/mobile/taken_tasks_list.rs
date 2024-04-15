use yew::{html, function_component, Html, Properties, use_state, UseStateHandle, use_effect_with_deps, use_effect};
use yew_hooks::{use_async, use_effect_once, UseLocalStorageHandle, use_local_storage};
use yew_router::prelude::{use_navigator, Link};
use yewdux::{prelude::use_store, log::debug};

use crate::{models::*, State, api, router::Route};

#[derive(Properties, PartialEq)]
struct TaskRowProps {
	task: Task,
}

#[function_component]
fn TaskRow(props: &TaskRowProps) -> Html {
	let navigator = use_navigator().unwrap();
	let (state, dispatch) = use_store::<State>();

	let task = props.task.to_owned();
	let time = task.time();

	html! {
		<li class="card task-card">
			<Link<Route> to={Route::TakenTaskView { task_id: task.id }}>
				if let Some(atask) = &state.active_task && atask.task.id == task.id {<div style="position: absolute; right: 0; bottom: 0; color: white; padding: 0px 5px; background-color: #76cb62;">
					{"Активная задача"}<i class="fas fa-fire ml-2" style="color: white;"></i>
				</div>}
				if task.r#type == TaskType::Floating {<div style="position: absolute; left: 0; bottom: 0; color: white; padding: 0px 5px; background-color: #6a6868;">
					{"Нормировка"}<i class="fas fa-calculator ml-2" style="color: white;"></i>
				</div>}
				if task.r#type == TaskType::Mixed  {<div style="position: absolute; left: 0; bottom: 0; color: white; padding: 0px 5px; background-color: #7ca1c7;">
					{"Смешанная"}<i class="fas fa-clock ml-2" style="color: white;"></i><i class="fas fa-calculator ml-2" style="color: white;"></i>
				</div>}
				<div class="card-content">
					<div class="media">
						<div class="media-content">
							<p class="title is-4">{task.name}
							// if task.floating { <i class="ml-2 fas fa-calculator"></i> }
							// if let Some(atask) = &state.active_task && atask.id == task.id {<i class="fas fa-fire ml-2" style="color: #76cb62;"></i>}
							</p>
							<p class="subtitle is-6" style="color: #838383;"><b>{task.date}{" "}{time}</b>{" #"}{task.id}</p>
						</div>
					</div>
				</div>
			</Link<Route>>
		</li>	
	}
}

#[function_component]
pub fn TakenTasksList() -> Html {
	let (state, dispatch) = use_store::<State>();
	let storage: UseLocalStorageHandle<State> = use_local_storage("state".to_string());
	let clonned_state = state.clone();
	let taken_tasks_list_async = use_async(async move {
		api::taken_tasks(&clonned_state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	});

	let clonned_taken_tasks = taken_tasks_list_async.clone();
	use_effect_once(move || {
		// debug!("Running effect once on mount");
		clonned_taken_tasks.run();
		|| debug!("Running clean-up of effect on unmount")
	});

	let clonned_new_tasks = taken_tasks_list_async.clone();
	use_effect_with_deps(move |data| {
		if !taken_tasks_list_async.loading {
			let dispatch = dispatch.clone();
			dispatch.reduce_mut(|state: &mut State| {
				state.taken_tasks = data.data.clone();
				if let Some(taken_tasks) = state.taken_tasks.as_ref() {
					// FIX aw9nVM
					state.taken_tasks_count = taken_tasks.len() as i8 - if state.active_task.is_some() { 1 } else { 0 };
				}
				// storage.set(state.clone());
			});
		}
	}, clonned_new_tasks);

	if let Some(tasks) = state.taken_tasks.to_owned() {
		html! {
			<div class="section py-5">
				<h1 class="title is-5 is-flex is-justify-content-center">
				{format!("{:.0}", tasks.iter().map(|task| task.plan_salary).sum::<f32>())}{"₽ / "}
				{format!("{:.0}", tasks.iter().map(|task| task.pass_salary).sum::<f32>())}{"₽"}
				</h1>
			
				<ul class="tasks-list">
					{tasks.iter()
						.filter(|task| !(state.active_task.is_some() && state.active_task.as_ref().unwrap().task.id == task.id))
						.map(|task| html!{<TaskRow task={task.clone()}></TaskRow>})
						.collect::<Html>()}
				</ul>
			</div>
		}
	} else {
		html! {
			<>
			{"..."}
			</>
		}
	}
}