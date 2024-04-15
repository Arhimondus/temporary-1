use chrono::Utc;
use pub_this::pub_this;
use yew::{html, function_component, Html, Properties, Callback};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{models::*, State, utils::{short_time, operation_background, operation_done_icon, stamp_from_operation, short_time2}, router::Route, CurrentOperation, state::AppStateActions, OperationResultStatus, OperationDoneStatus};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct TakenOperationProps {
	operation: Operation,
	operation_result: Option<OperationResult>,
	can_manual_start: bool,
}

#[function_component]
pub fn WorkFloatOperation(props: &TakenOperationProps) -> Html {
	let TakenOperationProps { operation, operation_result, .. } = &props;
	let (state, dispatch) = use_store::<State>();

	// let navigator = use_navigator().unwrap();
	// let navigator2 = use_navigator().unwrap();
	let current_operation_id = if let Some(active_task) = state.active_task.as_ref() {
		active_task.current_operation_id
	} else {
		None
	};

	// let click_operation = {
	// 	let state = state.clone();
	// 	let clonned_operation = operation.clone();
	// 	Callback::from(move |e| {
	// 		let clonned_operation = clonned_operation.clone();
			
	// 		if !state.active_task_operation_results.iter().any(|ato| ato.operation_id == clonned_operation.id) {
	// 			navigator.push(&Route::FinishOperation { task_id: clonned_operation.task_id, operation_id: clonned_operation.id });
	// 		}
	// 	})
	// };

	let start_operation = {
		let state = state.clone();
		let operation = operation.clone();
		Callback::from(move |e| {
			let operation = operation.clone();
			dispatch.reduce_mut(|state| state.start_operation_with_time(operation.id, Utc::now()));
			// navigator2.push(&Route::WorkTask);
		})
	};
	
	html! {
		<li
			class={operation_background(&operation, &operation_result, current_operation_id == Some(operation.id))}
			id={format!("operation{}", operation.id)}
			style="position: relative;"
		>
			<div style="position: absolute; left: 0; top: 0; color: #2c2c2c; font-size: 12px; line-height: 10px; display: flex; flex-direction: row; align-items: center;">{operation.id}<i class="fas fa-calculator" style="color: #363636; font-size: 8px; margin-left: 2px;"></i></div>
			// <span class="panel-icon">
			// 	<i class="fas fa-hashtag"></i>
			// </span>
			<div>
				// {stamp_from_operation(&operation)}
				{operation.name.clone()}{" "}
				if let Some(pass_start_time) = operation.pass_start_time && let Some(pass_end_time) = operation.pass_end_time {
					<span class="has-text-grey">					
						{short_time2(&pass_start_time)}
						{" - "}
						{short_time2(&pass_end_time)}
					</span>	
				} else {
					<span class="has-text-grey">
						<i class="fas fa-calculator ml-1" style="color: #363636;"></i>
						if let Some(operation_result) = operation_result {
							if operation_result.end_time.is_some() { 
								{format!(" {} мин.", operation_result.duration_minutes())}
							} else { 
								{""}
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
						if let Some(count) = operation_result.count {
							{" / "}<b class={if count != operation.plan_count { "has-text-danger" } else { "" }}>{count}{" "}{operation.unit.clone()}</b>
						} else if let Some(pass_count) = operation.pass_count {
							{" / "}<b>{pass_count}{" "}{operation.unit.clone()}</b>
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
							<i>{"Ваш комментарий: "}</i> {comment}
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