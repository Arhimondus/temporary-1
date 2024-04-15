use std::{rc::Rc, collections::BTreeMap};

use chrono::{DateTime, TimeZone, Datelike};
use pub_this::pub_this;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_effect_once};
use yewdux::{prelude::*, dispatch};
use yew_router::prelude::*;
use enclose::enclose;

use crate::{utils::{log_str, string_time_to_date, cancel_all_alarms, warning_message, get_first_passed_operation_id}, models::{self, OperationResult, OperationResultStatus}, api, State, router::Route, state::{AppStateActions, next_operation, get_next_operation_id}, components::PausesList};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct FinishOperationProps {
	task_id: u32,
	operation_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SubmitType {
	None,
	General,
	Next,
	Rest,
}

// Завершить операцию можно любую, которая в статусе Pass
#[function_component]
pub fn FinishGeneralOperation(props: &FinishOperationProps) -> Html {
	let comment = use_state(|| AttrValue::Rc("".into()));
	let pass_time = use_state(|| AttrValue::Rc("".into()));
	let submit_type = use_state(|| SubmitType::None);
	let count = use_state(|| 0.0);
	let need_comment = use_state(|| false);
	let navigator = use_navigator().unwrap();
	let first_load = use_state(|| false);

	let (state, dispatch) = use_store::<State>();

	let operation_id = props.operation_id;
	let active_task = state.active_task.clone().unwrap();
	let task_id = active_task.task.id;

	use_effect_once(enclose!((navigator, first_load, task_id, active_task) move || {
		let first_passed_operation_id = get_first_passed_operation_id(&active_task.operation_results, &active_task.operation_results_order);
		if let Some(first_passed_operation_id) = first_passed_operation_id {
			if first_passed_operation_id != operation_id {
				if let Some(last_op_id) = active_task.operation_results_order.last() {
					if operation_id == *last_op_id {
						warning_message("Перенаправление", "Сперва закройте эту операцию");
						navigator.push(&Route::FinishAutoOperation { task_id, operation_id: first_passed_operation_id });
					}
				}
			} else {
				first_load.set(true);
			}
		}
		
		|| {}
	}));
	
	let operation = active_task.task.operations.iter().find(|op| op.id == props.operation_id).unwrap().clone();
	use_effect_once(enclose!((operation, count) move || {
		let window = web_sys::window().expect("global window does not exists");    
		let document = window.document().expect("expecting a document on window");
		let element = document.get_element_by_id("count").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
		let initial_count = operation.plan_count;
		element.set_attribute("value", &initial_count.to_string()).unwrap();
		count.set(initial_count);
		|| {}
	}));

	let submit_next = Callback::from(enclose!((operation, comment, count, submit_type, need_comment) move |_e| {
		if *count > operation.plan_count && comment.is_empty() {
			warning_message("Количество выше плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		if *count < operation.plan_count && comment.is_empty() {
			warning_message("Количество ниже плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		submit_type.set(SubmitType::Next);
	}));

	let submit_rest = Callback::from(enclose!((operation, comment, count, submit_type, need_comment) move |_e| {
		if *count > operation.plan_count && comment.is_empty() {
			warning_message("Количество выше плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		if *count < operation.plan_count && comment.is_empty() {
			warning_message("Количество ниже плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		submit_type.set(SubmitType::Rest);
	}));

	let submit_general = Callback::from(enclose!((operation, comment, count, submit_type, need_comment) move |_e| {
		if *count > operation.plan_count && comment.is_empty() {
			warning_message("Количество выше плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		if *count < operation.plan_count && comment.is_empty() {
			warning_message("Количество ниже плана", "Укажите в комментариях причину изменения");
			need_comment.set(true);
			return;
		}
		submit_type.set(SubmitType::General);
	}));

	let operation_result = &active_task.operation_results[&props.operation_id];

	use_effect_with_deps(enclose!((active_task, comment, pass_time, count, submit_type) move |_| {
		match *submit_type {
			SubmitType::None => {},
			SubmitType::General => {
				let submit_comment = comment.to_string();
				let mut submit_pass_time = pass_time.to_string();
				dispatch.reduce_mut(|state| {
					state.finish_operation(
						operation_id, 
						*count,
						if &*comment == "" { None } else { Some(comment.to_string()) },
						chrono::Utc::now(),
					);
				});
		
				navigator.push(&Route::WorkTask);
			},
			SubmitType::Next => {
				let submit_comment = comment.to_string();
				let mut submit_pass_time = pass_time.to_string();
				dispatch.reduce_mut(|state| {
					state.finish_operation(
						operation_id, 
						*count,
						if &*comment == "" { None } else { Some(comment.to_string()) },
						chrono::Utc::now(),
					);
		
					if let Some(current_operation_id) = active_task.current_operation_id {
						if current_operation_id == operation_id {
							if let Some(next_operation_id) = get_next_operation_id(&state.active_task.clone().unwrap(), operation_id) {
								state.start_initial_general_operation(next_operation_id);
							}
						}
					}
				});
		
				navigator.push(&Route::WorkTask);
			},
			SubmitType::Rest => {
				let submit_comment = comment.to_string();
				let mut submit_pass_time = pass_time.to_string();
				dispatch.reduce_mut(|state| {
					state.finish_operation(
						operation_id, 
						*count,
						if &*comment == "" { None } else { Some(comment.to_string()) },
						chrono::Utc::now(),
					);
		
					cancel_all_alarms(state);
					state.no_operation();
				});
		
				navigator.push(&Route::WorkTask);
			},
		};
	}), submit_type.clone());

	html! {
		<div class="section">
			<h1 class="title is-5">
				<div class="has-text-grey is-size-6">{"Завершить операцию №"}{operation.id}</div>
				{operation.name.clone()}
			</h1>
			<div style="display: flex; flex-direction: column;">
				<div class="field">
					<label class="label">{"Количество "}{" (макс. "}{operation.count.clone()}{" "}{operation.unit.clone()}{")"}</label>
						<div class="control has-icons-left">
						<input id="count" class="input" type="number" oninput={Callback::from(enclose!((count) move |e: InputEvent| {
							let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
							count.set(input.value().parse().unwrap());
						}))}/>
						<span class="icon is-small is-left"><i class="fa fa-calculator"/></span>
					</div>
				</div>
				<div class="field">
					<label class="label">{"Комментарий"}</label>
					<div class="control has-icons-left">
						<input class="input" type="text" oninput={Callback::from(enclose!((comment) move |e: InputEvent| {
							let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
							comment.set(AttrValue::Rc(input.value().into()));
						}))}/>
						<span class="icon is-small is-left"><i class="fa fa-comment"/></span>
					</div>
				</div>
				<div class="field">
					<label class="label">{"Время операции"}</label>
					<div class="control has-icons-left">
						// {operation.time()}
					</div>
				</div>
				<div class="field">
					<label class="label">{"Паузы"}</label>
					<div class="control has-icons-left">
						<PausesList operation_result={operation_result.clone()}/>
					</div>
				</div>
			</div>
			<div class="is-flex is-flex-direction-column" style="gap: 10px;">
				if operation.is_current(&state) {
					<button class="button is-success mt-3" onclick={submit_next}>{"Завершить и дальше"}</button>
					<button class="button is-warning mt-3" onclick={submit_rest}>{"Завершить и другая"}</button>
				} else {
					<button class="button is-success mt-3" onclick={submit_general}>{"Записать результат"}</button>
				}
			</div>
			// if let Some(err) = &finish_operation_async.error {
			// 	<div class="notification is-danger mt-3">
			// 		{err}
			// 	</div>
			// }
		</div>
	}
}