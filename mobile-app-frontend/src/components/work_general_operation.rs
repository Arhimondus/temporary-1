use pub_this::pub_this;
use yew::{html, function_component, Html, Properties, Callback};
use yew_router::prelude::use_navigator;
use enclose::enclose;
use yewdux::prelude::use_store;

use crate::{models::*, State, utils::{short_time, operation_background, short_time2, operation_done_icon, stamp_from_operation}, router::Route, CurrentOperation, state::AppStateActions};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct TakenOperationProps {
	operation: Operation,
	operation_result: Option<OperationResult>,
	can_manual_start: bool,
}

// fn operation_color(operation: &Operation, op_result: Option<&OperationResult>, current_operation: Option<&CurrentOperation>) -> String {
// 	if let Some(op_result) = op_result {
// 		"panel-block has-background-primary has-text-white is-flex is-justify-content-space-between".into()
// 	} else if let Some(current_operation) = current_operation && current_operation.operation_id == operation.id {
// 		"panel-block has-background-info has-text-white is-flex is-justify-content-space-between".into()
// 	} else if let Some(accept_count) = operation.accept_count {
// 		if accept_count == operation.pass_count.unwrap() {
// 			"panel-block has-background-success has-text-white is-flex is-justify-content-space-between".into()
// 		} else if accept_count > 0.0 {
// 			"panel-block has-background-warning has-text-white is-flex is-justify-content-space-between".into()
// 		} else {
// 			"panel-block has-background-danger has-text-white is-flex is-justify-content-space-between".into()
// 		}
// 	} else if let Some(pass_count) = operation.pass_count {
// 		"panel-block has-background-primary has-text-white is-flex is-justify-content-space-between".into()
// 	} else {
// 		"panel-block is-flex is-justify-content-space-between".into()
// 	}
// }

#[function_component]
pub fn WorkGeneralOperation(props: &TakenOperationProps) -> Html {
	let TakenOperationProps { operation, operation_result, .. } = &props;
	let (state, dispatch) = use_store::<State>();
	
	let navigator = use_navigator().unwrap();

	let current_operation_id = if let Some(active_task) = state.active_task.as_ref() {
		active_task.current_operation_id
	} else {
		None
	};

	let click_operation = Callback::from(enclose!((state, operation, operation_result, navigator) move |e| {
		if operation_result.clone().unwrap().status == OperationResultStatus::Pass {
			navigator.push(&Route::FinishGeneralOperation { task_id: operation.task_id, operation_id: operation.id });
		}
	}));

	let start_operation = Callback::from(enclose!((state, operation, navigator) move |e| {
		// navigator.push(&Route::WorkTask);
		dispatch.reduce_mut(|state| state.start_initial_general_operation(operation.id));
	}));

	html! {
		<li 
			class={operation_background(&operation, &operation_result.clone(), current_operation_id == Some(operation.id))}
			id={format!("operation{}", operation.id)}
			style="position: relative;"
		>
			<div style="position: absolute; left: 0; top: 0; color: #2c2c2c; font-size: 12px; line-height: 10px;">{operation.id}</div>
			// <span class="panel-icon">
			// 	<i class="fas fa-hashtag"></i>
			// </span>
			<div onclick={click_operation} >
				// {stamp_from_operation(&operation)}
				{operation.name.clone()}
				// {format!(" {:#?}", operation_result.clone().unwrap().status)}
				// {format!("{:#?}", operation_result.clone().unwrap().start_time)}
				if let Some(pass_start_time) = operation.pass_start_time && let Some(pass_end_time) = operation.pass_end_time {
					<span class="has-text-grey">					
						{short_time2(&pass_start_time)}
						{" - "}
						{short_time2(&pass_end_time)}
					</span>	
				} else {
					<span class="has-text-grey">
						// {operation.duration} {" мин."} <= эталонная длительность?
						if let Some(operation_result) = operation_result {
							if operation_result.end_time.is_some() { 
								{format!(" {} мин.", operation_result.duration_minutes())}
							} else { 
								{" "} {operation.duration} {" мин."}
							}
						} else {
							if operation.pass_start_time.is_some() && operation.pass_end_time.is_some() {
								{operation.duration_minutes()}
							}
						}
					</span>	
				}
				<div>
					if let Some(operation_result) = operation_result {
						<b>{operation.plan_count}{" "}{operation.unit.clone()}</b>
						if let Some(pass_count) = operation_result.count {
							{" / "}<b class={if pass_count < operation.plan_count { "has-text-danger" } else { "" }}>{pass_count}{" "}{operation.unit.clone()}</b>
						} else if let Some(pass_count) = operation.pass_count {
							{" / "}<b class={if pass_count < operation.plan_count { "has-text-danger" } else { "" }}>{pass_count}{" "}{operation.unit.clone()}</b>
						}
						if let Some(accept_count) = operation.accept_count {
							{" / "}<b>{accept_count}{" "}{operation.unit.clone()}</b>
						}
					} else {
						<b>{operation.plan_count}{" "}{operation.unit.clone()}</b>
						if let Some(pass_count) = operation.pass_count {
							{" / "}<b class={if pass_count != operation.plan_count { "has-text-danger" } else { "" }}>{pass_count}{" "}{operation.unit.clone()}</b>
						}
						if let Some(accept_count) = operation.accept_count {
							{" / "}<b class={if operation.pass_count.unwrap() != accept_count || accept_count == 0. { "has-text-danger" } else { "" }}>{accept_count}{" "}{operation.unit.clone()}</b>
						}
					}
				</div>
				if let Some(operation_result) = operation_result {
					if let Some(comment) = operation_result.comment.clone() {
						<div class="comment">
							<i>{"Комментарий: "}</i> {comment}
						</div>
					}
				}
				<div class="comment">if let Some(pass_comment) = operation.pass_comment.clone() {
					<i>{"Ваш комментарий: "}</i> {pass_comment}
				}</div>
				<div class="comment">if let Some(accept_comment) = operation.accept_comment.clone() {
					<i>{"Комментарий проверки: "}</i> {accept_comment}
				}</div>
			</div>

			<div>
				if let Some(operation_result) = operation_result {
					{operation_done_icon(&operation_result)}
					if current_operation_id.is_none() && operation_result.status == OperationResultStatus::Open {
						<button class="button mr-2" onclick={start_operation}>{"Начать"}</button>
					}
				}
			</div>
		</li>
	}
}