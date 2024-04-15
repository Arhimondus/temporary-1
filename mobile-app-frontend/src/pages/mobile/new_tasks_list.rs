use yew::{html, function_component, Html, Properties, use_state, UseStateHandle, use_effect_with_deps, use_effect};
use yew_hooks::{use_async, use_effect_once, UseLocalStorageHandle, use_local_storage};
use yew_router::prelude::{Link, Redirect};
use yewdux::{prelude::use_store, log::debug};

use crate::{models::*, state::State, api, router::Route};

#[derive(Properties, PartialEq)]
struct TaskRowProps {
	task: Task,
}

#[function_component]
fn TaskRow(props: &TaskRowProps) -> Html {
	let task = props.task.to_owned();
	let time = task.time();

	html! {
		<li class="card task-card">
			<Link<Route> to={Route::NewTaskView { task_id: task.id }}>
				if task.r#type == TaskType::Floating {<div style="position: absolute; left: 0; bottom: 0; color: white; padding: 0px 5px; background-color: #6a6868;">
					{"Нормировка"}<i class="fas fa-calculator ml-2" style="color: white;"></i>
				</div>}
				if task.r#type == TaskType::Mixed  {<div style="position: absolute; left: 0; bottom: 0; color: white; padding: 0px 5px; background-color: #7ca1c7;">
					{"Смешанная"}<i class="fas fa-clock ml-2" style="color: white;"></i><i class="fas fa-calculator ml-2" style="color: white;"></i>
				</div>}
				<div class="card-content">
					<div class="media">
						<div class="media-content">
							<p class="title is-4">{task.name}</p>
							<p class="subtitle is-6" style="color: #838383;"><b>{task.date}{" "}{time}</b>{" #"}{task.id}</p>
						</div>
					</div>
				</div>
			</Link<Route>>
		</li>	
	}
}

#[function_component]
pub fn NewTasksList() -> Html {
	let (state, dispatch) = use_store::<State>();
	let storage: UseLocalStorageHandle<State> = use_local_storage("state".to_string());
	let clonned_state = state.clone();

	let session = clonned_state.user_session_id.clone();
	// if session.is_none() {
	// 	return html! {<Redirect<Route> to={Route::Auth}/>};
	// }
	
	let new_tasks_list_async = use_async(async move {	
		api::new_tasks(&session.unwrap()).await.or_else(|e| Err(e.error))
	});

	let clonned_new_tasks = new_tasks_list_async.clone();
	use_effect_once(move || {
		// debug!("Running effect once on mount");
		clonned_new_tasks.run();
		|| debug!("Running clean-up of effect on unmount")
	});

	let clonned_new_tasks = new_tasks_list_async.clone();
	use_effect_with_deps(move |data| {
		if !new_tasks_list_async.loading {
			let dispatch = dispatch.clone();
			dispatch.reduce_mut(|state: &mut State| {
				state.new_tasks = data.data.clone();
				if let Some(new_tasks) = state.new_tasks.as_ref() {
					// FIX rN9DfZ
					state.new_tasks_count = new_tasks.len() as i8;
				}
				// storage.set(state.clone());
			});
		}
	}, clonned_new_tasks);

	if let Some(tasks) = state.new_tasks.to_owned() {
		html! {
			<div class="section py-5">
				<h1 class="title is-5 is-flex is-justify-content-center">
					{format!("{:.0}", tasks.iter().map(|task| task.plan_salary).sum::<f32>())}{"₽"}
				</h1>
				
				<ul class="tasks-list">
					{tasks.iter()
						.map(|task| html!{<TaskRow task={task.clone()}></TaskRow>})
						.collect::<Html>()}
					if tasks.len() == 0 {
						{"Нет доступных задач для отображения"}
					}
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