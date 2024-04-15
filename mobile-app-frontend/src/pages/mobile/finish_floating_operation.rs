use std::{rc::Rc, borrow::Borrow};

use chrono::{DateTime, TimeZone, Datelike};
use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_effect_once, UseLocalStorageHandle, use_local_storage};
use yewdux::prelude::*;
use yew_router::prelude::*;
use enclose::enclose;
use crate::{components::PausesList, utils::{log_str, string_time_to_date, operation_done_icon, error_message, warning_message}, models::{self, OperationResult}, api, state::{self, State, AppStateActions}, router::Route, CurrentOperation};

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

#[function_component]
pub fn FinishFloatingOperation(props: &FinishOperationProps) -> Html {
	let comment = use_state(|| AttrValue::Rc("".into()));
	let pass_time = use_state(|| AttrValue::Rc("".into()));
	let submit_type = use_state(|| SubmitType::None);
	let count = use_state(|| 0.0);
	let need_comment = use_state(|| false);
	let navigator = use_navigator().unwrap();

	let (state, dispatch) = use_store::<State>();

	let operation_id = props.operation_id;

	let Some(active_task) = state.active_task.as_ref() else { return html! { <Redirect<Route> to={Route::FinishedTasks}/> } };

	let Some(operation_result) = active_task.current_operation_result() else { return html! { <Redirect<Route> to={Route::FinishedTasks}/> } };
	let operation = operation_result.get_operation_from_task(&active_task);

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

	use_effect_once(enclose!((operation, count) move || {
		let window = web_sys::window().expect("global window does not exists");    
		let document = window.document().expect("expecting a document on window");
		let element = document.get_element_by_id("count").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();
		let initial_count = operation.plan_count;
		element.set_attribute("value", &initial_count.to_string()).unwrap();
		count.set(initial_count);
		|| {}
	}));

	use_effect_with_deps(enclose!((comment, pass_time, count, submit_type) move |_| {
		match *submit_type {
			SubmitType::None => {},
			SubmitType::General => {},
			SubmitType::Next => {
				let submit_comment = comment.to_string();
				let mut submit_pass_time = pass_time.to_string();
				dispatch.reduce_mut(|state| {
					state.finish_operation(
						operation_id, 
						*count,
						if &*submit_comment == "" { None } else { Some(submit_comment.to_string()) },
						chrono::Utc::now(),
					);
					state.operation_pass_upload(operation_id, true);
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
					state.operation_pass_upload(operation_id, false);
				});
				navigator.push(&Route::WorkTask);
			},
		};
	}), submit_type.clone());

	html! {
		<div class="section">
			<h1 class="title is-5">
				<div class="has-text-grey is-size-6">{"Завершить операцию №"}{operation.id}</div>
				{operation.name}
			</h1>

			<form style="display: flex; flex-direction: column;">
				<div class="field">
					<label class="label">{"Количество "}{" (макс. "}{operation.count}{" "}{operation.unit}{")"}</label>
						<div class="control has-icons-left">
						<input id="count" class="input" type="number" oninput={Callback::from(move |e: InputEvent| {
							let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
							let count = count.clone();
							count.set(input.value().parse().unwrap());
						})}/>
						<span class="icon is-small is-left"><i class="fa fa-calculator"/></span>
					</div>
				</div>
				<div class="field">
					<label class="label">{"Комментарий"}</label>
					<div class={if *need_comment { "ramka-5" } else { "" }}>
						<div class="control has-icons-left" style="width: 100%;">
							<input class="input" type="text" oninput={Callback::from(move |e: InputEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								let comment = comment.clone();
								comment.set(AttrValue::Rc(input.value().into()));
							})}/>
							<span class="icon is-small is-left"><i class="fa fa-comment"/></span>
						</div>
					</div>
				</div>
				<div class="field">
					<label class="label">{"Время операции"}</label>
					<div class="control has-icons-left">
						// {operation.potential_time()}
					</div>
				</div>
				<div class="field">
					<label class="label">{"Паузы"}</label>
					<div class="control has-icons-left">					
						<PausesList operation_result={operation_result}/>
					</div>
				</div>
			</form>
			<div class="is-flex is-flex-direction-column" style="gap: 10px;">
				<button class="button is-success mt-3" onclick={submit_next}>{"Завершить и дальше"}</button>
				<button class="button is-warning mt-3" onclick={submit_rest}>{"Завершить и другая"}</button>
			</div>
			// if let Some(err) = &finish_operation_async.error {
			// 	<div class="notification is-danger mt-3">
			// 		{err}
			// 	</div>
			// }
		</div>
	}
}