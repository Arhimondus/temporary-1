use std::rc::Rc;

use chrono::{DateTime, Local};
use gloo_timers::callback::Interval;
use pub_this::pub_this;
use web_sys::{HtmlInputElement, InputEvent, FocusEvent};
use yew::{html, function_component, Html, Properties, Callback, UseStateHandle, use_state, use_effect_with_deps, TargetCast};
use yewdux::prelude::*;
use yew_router::prelude::*;
use enclose::enclose;

use crate::{models::*, State, utils::{short_time, log_str, Oor}, router::Route, CurrentOperation, state::{AppStateActions, get_current_operation, get_non_passed_operations}, components::PausesList};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct GeneralActiveOperationActionsProps {
	operation: (Operation, OperationResult),
}

#[function_component]
pub fn GeneralActiveOperationActions(props: &GeneralActiveOperationActionsProps) -> Html {
	let (operation, operation_result) = &props.operation;
	let operation_id = operation.id;

	let pause_window_state = use_state(|| false);
	let interval_state = use_state(|| None);
	let duration_state: UseStateHandle<String> = use_state(|| operation_result.duration());
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let comment_state = use_state(|| "".to_string());

	// let pause_menu = Callback::from(enclose!((pause_window_state) move |_| {
	// 	pause_window_state.set(true);
	// }));

	// let cancel_pause = Callback::from(enclose!((pause_window_state) move |_| {
	// 	pause_window_state.set(false);
	// }));

	let pause = dispatch.reduce_mut_callback(enclose!((pause_window_state, comment_state) move |state| {
		pause_window_state.set(true);
		state.pause_operation(operation_id);
		// state.pause_operation_with_pause(operation_id, &comment_state);
	}));

	let is_on_pause = if let Some(last_pause) = operation_result.pauses.last() {
		last_pause.resume_time.is_none()
	} else {
		false
	};

	let resume = dispatch.reduce_mut_callback(enclose!((pause_window_state, comment_state) move |state| {
		pause_window_state.set(false);
		state.resume_operation_with_comment(operation_id, &comment_state);
	}));

	let done = Callback::from(enclose!((navigator, operation, operation_result) move |_| {
		navigator.push(&Route::FinishGeneralOperation { task_id: operation.task_id, operation_id: operation_result.operation_id });
	}));

	use_effect_with_deps(enclose!((operation_result, duration_state, interval_state) move |data| {
		log_str(&serde_json::to_string(&operation_result.pauses).unwrap());
		if let Some(last_pause) = operation_result.pauses.last() {
			if last_pause.resume_time.is_none() {
				interval_state.set(None);
			} else {
				interval_state.set(Some(Interval::new(100, move || {
					duration_state.set(operation_result.duration());
				})));
			}
		} else {
			interval_state.set(Some(Interval::new(100, move || {				
				duration_state.set(operation_result.duration());
			})));
		}
	}), state.clone());

	use_effect_with_deps(move |(state, duration): &(Rc<State>, UseStateHandle<String>)| {
		log_str("effect 2353250");
		if let Some(alarms) = state.alarms.clone() {
			let non_passed_operations = get_non_passed_operations(&alarms);
			let active_operation = get_current_operation(&alarms);

			if active_operation.is_none() {
				dispatch.reduce_mut(|state| state.no_operation())
			}

			let active_task = state.active_task.clone().unwrap();
			for current_operation in non_passed_operations.iter() {
				let operation = active_task.operation_results.iter().find(|it| *it.0 == current_operation.0).unwrap();
				if operation.1.status == OperationResultStatus::Open {
					let (operation_id, start_time) = current_operation;
					dispatch.reduce_mut(|state| state.start_operation_with_time(*operation_id, start_time.clone()));
				} else if let OperationResultStatus::Done(_) = operation.1.status {
					// TODO перезапуск операций
					// let (operation_id, start_time) = state.next_operation(operation_id);
					// dispatch.reduce_mut(|state| state.start_operation_with_time(*operation_id, start_time.clone()));
				}
			}
		}
	}, (state.clone(), duration_state.clone()));

	match operation_result.status.clone() {
		OperationResultStatus::Done(_) => {
			return html! {
				<div class="p-3" style="border-top: 1px solid lightgray; border-bottom: 1px solid black;">
					<div>
						<PausesList operation_result={operation_result.clone()}/>
						// {"{"}{operation_result.pauses_duration().to_string()}{"}"}
					</div>
					<h3 class="title is-4 has-text-centered mb-0">{operation.name.clone()}</h3>
					<h4 class="is-flex is-justify-content-space-between my-2">
						<div class="is-family-monospace">{"Начало: "}<br/>{format!("{}", Into::<DateTime<Local>>::into(operation_result.start_time()).format("%d.%m %H:%M"))}</div>
						<div class="is-family-monospace">if is_on_pause { <i class="mr-2 fas fa-pause mr-2"></i> }{"Вы завершили текущую операцию"}</div>
					</h4>
				</div>
			};
		}
		_ => {
			return html! {
				<div class="p-3" style={format!("border-top: 1px solid lightgray; border-bottom: 1px solid black; {}", if operation_result.pauses.iter().any(|p| p.resume_time.is_none()) { "background-color: wheat;" } else { "" })}>
					<div>
						// <pre>
						// 	<code style="font-size: 8px;">
						// 	{operation_result.duration_test()}
						// 	</code>
						// </pre>
						<PausesList operation_result={operation_result.clone()}/>
						// {"{"}{operation_result.pauses_duration().to_string()}{"}"}
					</div>
					<h3 class="title is-4 has-text-centered mb-0">{operation.name.clone()}</h3>
					<h4 class="is-flex is-justify-content-space-between my-2">
						<div class="is-family-monospace">{"Начало: "}<br/>{format!("{}", Into::<DateTime<Local>>::into(operation_result.start_time()).format("%d.%m %H:%M"))}</div>
						<div class="is-family-monospace">if is_on_pause { <i class="mr-2 fas fa-pause mr-2"></i> }{"Время: "}<br/>{duration_state.clone().to_string()}</div>
					</h4>
					
					if !operation_result.pauses.iter().any(|p| p.resume_time.is_none()) {
						<div class="is-flex is-justify-content-space-around" style="gap: 10px;">
							<button class="button is-success" disabled={is_on_pause} onclick={done}>{"Завершить"}</button>
							<button class="button is-primary" onclick={pause} style="width: 130px;">{"Пауза"}</button>
						</div>
					} else {
						<div class="is-flex is-justify-content-space-around" style="gap: 10px;">
							<input id="pause_comment" onfocus={Callback::from(move |e: FocusEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								input.set_placeholder("");
							})} onblur={Callback::from(move |e: FocusEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								input.set_placeholder("Комментарий паузы");
							})} class="input dark-placeholder" placeholder="Комментарий паузы" oninput={Callback::from(move |e: InputEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								let comment_state = comment_state.clone();
								comment_state.set(input.value().to_string());
							})}/>
							<button class="button is-success" onclick={resume} style="width: 130px;"><i class="fas fa-check"></i></button>
						</div>
					}
				</div>
			};
		}
	}
}