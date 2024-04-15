use yew::{function_component, Html, html};
use yewdux::prelude::use_store;
use crate::{models::*, State, utils::short_time, router::Route, CurrentOperation};
use super::{FloatActiveOperationActions, GeneralActiveOperationActions};

#[function_component]
pub fn ActiveOperationActions() -> Html {
	let (state, dispatch) = use_store::<State>();

	if let Some(active_task) = &state.active_task {
		if let Some(current_operation_id) = active_task.current_operation_id {
			let result = &active_task.operation_results[&current_operation_id];
			let operation = result.get_operation(&state);
			if operation.floating {
				return html! {<FloatActiveOperationActions operation={(operation, result.clone())}/>};
			} else {
				return html! {<GeneralActiveOperationActions operation={(operation, result.clone())}/>};
			}
		}
	}

	return html! {<></>};
}