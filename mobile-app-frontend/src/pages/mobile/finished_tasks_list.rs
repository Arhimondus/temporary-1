use yew::{html, function_component, Html, Properties, use_state, UseStateHandle, use_effect_with_deps, use_effect};
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::{use_navigator, Link};
use yewdux::{prelude::use_store, log::debug};
use enclose::enclose;

use crate::{models::*, State, api, router::Route, components::{Stamp, StampType}};

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

	let accept_status: StampType = task.accept_status.into();
	html! {
		<li class={format!("card task-card {}", match task.pass_status {
			TaskPassStatus::Open => "",
			TaskPassStatus::Canceled => "task-pass-status-canceled",
			TaskPassStatus::Zero => "task-pass-status-zero",
			TaskPassStatus::Partial => "task-pass-status-partial",
			TaskPassStatus::Full => "task-pass-status-full",
		})}>
			<Link<Route> to={Route::FinishedTaskView { task_id: task.id }} classes="relative">
				<div class="card-content">
					<div class="media">
						<div class="media-content">
							<p class="title is-4">{task.name} if let Some(atask) = &state.active_task && atask.task.id == task.id {<i class="fas fa-fire ml-2"></i>}</p>
							<p class="subtitle is-6"><b>{task.date}{" "}{time}</b>{" #"}{task.id}</p>
							<Stamp r#type={accept_status} title={match task.accept_status {
								TaskAcceptStatus::Open => "Ещё на проверке",
								TaskAcceptStatus::Canceled => "Отменена",
								TaskAcceptStatus::Zero => "Полностью отклонена",
								TaskAcceptStatus::Partial => "Частично принята",
								TaskAcceptStatus::Full => "Полностью принята",
							}}/>
						</div>
					</div>
				</div>
			</Link<Route>>
		</li>	
	}
}

#[function_component]
pub fn FinishedTasksList() -> Html {
	let (state, dispatch) = use_store::<State>();

	let finished_tasks_list_async = use_async(enclose!((state) async move {
		api::finished_tasks(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));

	use_effect_once(enclose!((finished_tasks_list_async) move || {
		finished_tasks_list_async.run();
		|| debug!("Running clean-up of effect on unmount")
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

	if let Some(tasks) = state.finished_tasks.to_owned() {
		html! {
			<div class="section py-5">
				<h1 class="title is-5 is-flex is-justify-content-center">
					{format!("{:.0}", tasks.iter().map(|task| task.plan_salary).sum::<f32>())}{"₽ / "}
					{format!("{:.0}", tasks.iter().map(|task| task.pass_salary).sum::<f32>())}{"₽ / "}
					{format!("{:.0}", tasks.iter().map(|task| task.accept_salary).sum::<f32>())}{"₽"}
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