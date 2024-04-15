use std::rc::Rc;

use chrono::{Utc, DateTime};
use pub_this::pub_this;
use yew::{html, function_component, Html, Properties, Callback};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;
use enclose::enclose;

use crate::{models::*, State, utils::{short_time, operation_background, operation_done_icon, stamp_from_operation, short_time2}, router::Route, CurrentOperation, state::AppStateActions, OperationResultStatus, OperationDoneStatus};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct FinishedOperationProps {
	operation: Operation,
	task: Rc<Task>,
}

fn get_good_time(task: &Task) -> bool {
	if let Some(closed_time) = task.closed_time {
		Utc::now().naive_local() < closed_time.naive_local()
	} else {
		false
	}
}

#[function_component]
pub fn FinishedOperation(props: &FinishedOperationProps) -> Html {
	let FinishedOperationProps { operation, task } = &props;
	let navigator = use_navigator().unwrap();
	let good_time = get_good_time(&task);

	let change_operation = Callback::from(enclose!((operation, task) move |_| {
		navigator.push(&Route::ChangeFinishedOperation { task_id: task.id, operation_id: operation.id });
	}));

	html! {
		<li
			class={operation_background(&operation, &None, false)}
			id={format!("operation{}", operation.id)}
			style="position: relative;"
		>
			<div style="position: absolute; left: 0; top: 0; color: #2c2c2c; font-size: 12px; line-height: 10px; display: flex; flex-direction: row; align-items: center;">{operation.id}<i class="fas fa-calculator" style="color: #363636; font-size: 8px; margin-left: 2px;"></i></div>
			<div>
				{stamp_from_operation(&operation)}
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
						if operation.pass_start_time.is_some() && operation.pass_end_time.is_some() {
							{operation.duration_minutes()}
						}
					</span>
				}
				<div>
					<b>{operation.plan_count}{" "}{operation.unit.clone()}</b>
					if let Some(pass_count) = operation.pass_count {
						{" / "}<b class={if pass_count != operation.plan_count { "has-text-danger" } else { "" }}>{pass_count}{" "}{operation.unit.clone()}</b>
					}
					if let Some(accept_count) = operation.accept_count {
						{" / "}<b class={if operation.pass_count.unwrap() != accept_count || accept_count == 0. { "has-text-danger" } else { "" }}>{accept_count}{" "}{operation.unit.clone()}</b>
					}
				</div>
				<div class="comment">if let Some(pass_comment) = operation.pass_comment.clone() {
					<i>{"Ваш комментарий: "}</i> {pass_comment}
				}</div>
				<div class="comment">if let Some(accept_comment) = operation.accept_comment.clone() {
					<i>{"Комментарий проверки: "}</i> {accept_comment}
				}</div>
			</div>

			<div>
			if good_time {
				<button class="button mr-2" onclick={change_operation}>{"Исправить"}</button>
			}
			</div>
		</li>
	}
}