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
use crate::{components::PausesList, utils::{log_str, string_time_to_date, operation_done_icon, error_message, warning_message}, models::{self, OperationResult, Pause, Task}, api, state::{self, State, AppStateActions}, router::Route, CurrentOperation};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct PauseItemProps {
	task_id: u32,
	operation_id: u32,
	index: usize,
}

#[function_component]
pub fn ChangePause(props: &PauseItemProps) -> Html {
	let PauseItemProps { task_id, operation_id, index } = &props;
	let comment = use_state(|| AttrValue::Rc("".into()));
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let Some(finished_tasks) = state.finished_tasks.clone() else {
		return html! {<Redirect<Route> to={Route::FinishedTasks}/>};
	};
	let task = finished_tasks.into_iter().find(|t| t.id == props.task_id).unwrap();
	let operation = task.operations.iter().find(|t| t.id == *operation_id).unwrap().clone();
	let pause: Pause = operation.pass_pauses.iter().enumerate().find(|(i, item)| i == index).unwrap().1.clone();

	use_effect_once(enclose!((operation, comment, pause) move || {
		let window = web_sys::window().expect("global window does not exists");    
		let document = window.document().expect("expecting a document on window");

		let pause_comment = pause.comment.unwrap_or("".into());
		let element = document.get_element_by_id("comment").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap();		
		element.set_attribute("value", &pause_comment).unwrap();
		comment.set(pause_comment.into());

		|| {}
	}));

	let finished_tasks_list_async = use_async(enclose!((state) async move {
		api::finished_tasks(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));

	let change_pause_async = use_async(enclose!((state, comment, index) async move {
		let str = comment.as_str();
		api::change_pause(&state.user_session_id.as_ref().unwrap(), task.id, operation.id, index, if str == "" { None } else { Some(str.to_string()) }).await.or_else(|e| Err(e.error))
	}));

	let submit = Callback::from(enclose!((change_pause_async) move |_| {		
		change_pause_async.run();
	}));

	use_effect_with_deps(enclose!((navigator, task, finished_tasks_list_async) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::FinishedTaskView { task_id: task.id });
		}
	}), change_pause_async.data);

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
					<label class="label">{"Время паузы"}</label>
					<div class="control has-icons-left">
						{"#"} {index + 1} {" "} {pause.time()}
					</div>
				</div>
				<div class="field">
					<label class="label">{"Комментарий"}</label>
					<div>
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
			</form>
			<div class="is-flex is-flex-direction-column" style="gap: 10px;">
				<button class="button is-success mt-3" onclick={submit}>{"Изменить"}</button>
			</div>
		</div>
	}
}