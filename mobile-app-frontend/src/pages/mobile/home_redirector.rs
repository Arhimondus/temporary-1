use std::{rc::Rc, collections::HashMap};

use js_sys::eval;
use pub_this::pub_this;
use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle};
use yewdux::{prelude::*, storage};
use yew_router::prelude::*;

use crate::{utils::{log_str, short_time, register_alarms}, models::{self, Alarm, OperationResult, TaskType}, api, State, router::Route, components::SimpleOperation, state::{ActiveTask, AppStateActions}};

#[function_component]
pub fn HomeRedirector() -> Html {
	let (state, dispatch) = use_store::<State>();

	if state.active_task.is_none() {
		html! {<Redirect<Route> to={Route::Auth}/>}
	} else {
		html! {<Redirect<Route> to={Route::WorkTask}/>}
	}
}