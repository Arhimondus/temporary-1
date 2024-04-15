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

use crate::{state::State, router::Route};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct FinishAutoOperationProps {
	task_id: u32,
	operation_id: u32,
}

#[function_component]
pub fn FinishAutoOperation(props: &FinishAutoOperationProps) -> Html {
	let (state, dispatch) = use_store::<State>();
	let FinishAutoOperationProps { task_id, operation_id } = props;
	 
	let Some(active_task) = state.active_task.as_ref() else { return html! { "Ожидание активной задачи..." } };

	if active_task.task.operations.iter().find(|it| it.id == *operation_id).unwrap().floating {
		html! { <Redirect<Route> to={Route::FinishFloatingOperation { task_id: *task_id, operation_id: *operation_id }}/> }
	} else {
		html! { <Redirect<Route> to={Route::FinishGeneralOperation { task_id: *task_id, operation_id: *operation_id }}/> }
	}
}