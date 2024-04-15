use std::{rc::Rc, borrow::Borrow};

use chrono::{DateTime, TimeZone, Datelike};
use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once, UseLocalStorageHandle, use_local_storage};
use yewdux::prelude::*;
use yew_router::prelude::*;
use enclose::enclose;
use crate::{utils::{log_str, string_time_to_date, operation_done_icon, error_message, warning_message}, models::{self, OperationResult, Pause, Task}, api, state::{self, State, AppStateActions}, router::Route, CurrentOperation};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct PauseItemProps {
	index: usize,
	pause: Pause,
	task_id: u32,
	operation_id: u32,
}

#[function_component]
pub fn PauseItem(props: &PauseItemProps) -> Html {
	html! {
		<Link<Route> classes="panel-block" to={Route::ChangePause { task_id: props.task_id, operation_id: props.operation_id, index: props.index }}>
			<div>
				<b class="has-text-weight-bold">{props.pause.time()}</b>
				<div>{props.pause.comment.clone().unwrap_or("<Нет комментария>".into())}</div>
			</div>
		</Link<Route>>
	}
}

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct PausesListProps {
	pauses: Vec<Pause>,
	task_id: u32,
	operation_id: u32,
}

#[function_component]
pub fn PausesList(props: &PausesListProps) -> Html {
	html! {
		<div class="panel is-primary">
			{props.pauses.iter().enumerate().map(|(index, pause)| {
				html! {<PauseItem index={index} pause={pause.clone()} task_id={props.task_id} operation_id={props.operation_id}/>}
			}).collect::<Vec<_>>()}
		</div>
	}
}

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct ChangeFinishedOperationProps {
	task_id: u32,
	operation_id: u32,
}

#[function_component]
pub fn ChangeFinishedOperation(props: &ChangeFinishedOperationProps) -> Html {
	let ChangeFinishedOperationProps { task_id, operation_id } = &props;
	let comment = use_state(|| AttrValue::Rc("".into()));
	// let pass_time = use_state(|| AttrValue::Rc("".into()));
	let count = use_state(|| 0.0);
	let need_comment = use_state(|| false);
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let Some(finished_tasks) = state.finished_tasks.clone() else {
		return html! {<Redirect<Route> to={Route::FinishedTasks}/>};
	};
	let task = finished_tasks.into_iter().find(|t| t.id == props.task_id).unwrap();
	let operation = task.operations.iter().find(|t| t.id == *operation_id).unwrap().clone();

	use_effect_once(enclose!((operation, count, comment) move || {
		let window = web_sys::window().expect("global window does not exists");
		let document = window.document().expect("expecting a document on window");

		let pass_count = operation.pass_count.unwrap_or(0.0);
		let element = document.get_element_by_id("count").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();		
		element.set_attribute("value", &pass_count.to_string()).unwrap();
		count.set(operation.pass_count.unwrap());

		let pass_comment = operation.pass_comment.unwrap_or("".into());
		let element = document.get_element_by_id("comment").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();		
		element.set_attribute("value", &pass_comment).unwrap();
		comment.set(pass_comment.into());

		|| {}
	}));

	let finished_tasks_list_async = use_async(enclose!((state) async move {
		api::finished_tasks(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));

	let change_finished_operation_async = use_async(enclose!((state, task, operation, count, comment) async move {
		let str = comment.as_str();
		api::change_finished_operation(&state.user_session_id.as_ref().unwrap(), task.id, operation.id, *count, if str == "" { None } else { Some(str.to_string()) }).await.or_else(|e| Err(e.error))
	}));

	let submit = Callback::from(enclose!((change_finished_operation_async, operation, comment, count, need_comment) move |_e| {
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
		
		change_finished_operation_async.run();
	}));

	use_effect_with_deps(enclose!((navigator, task, finished_tasks_list_async) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::FinishedTaskView { task_id: task.id });
		}
	}), change_finished_operation_async.data);

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
							<input id="comment" class="input" type="text" oninput={Callback::from(move |e: InputEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								let comment = comment.clone();
								comment.set(AttrValue::Rc(input.value().into()));
							})}/>
							<span class="icon is-small is-left"><i class="fa fa-comment"/></span>
						</div>
					</div>
				</div>
				// <div class="field">
				// 	<label class="label">{"Время операции"}</label>
				// 	<div class="control has-icons-left">
				// 		{operation}
				// 	</div>
				// </div>
				<div class="field">
					<label class="label">{"Паузы"}</label>
					<div class="control has-icons-left">					
						<PausesList pauses={operation.pass_pauses} task_id={task.id} operation_id={operation.id}/>
					</div>
				</div>
			</form>
			<div class="is-flex is-flex-direction-column" style="gap: 10px;">
				<button class="button is-success mt-3" onclick={submit}>{"Сохранить"}</button>
			</div>
			// if let Some(err) = &finish_operation_async.error {
			// 	<div class="notification is-danger mt-3">
			// 		{err}
			// 	</div>
			// }
		</div>
	}
}