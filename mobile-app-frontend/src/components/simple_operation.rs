
use pub_this::pub_this;
use yew::{html, function_component, Html, Properties};
use yewdux::prelude::use_store;

use crate::{models::*, State, utils::short_time};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct SimpleOperationProps {
	operation: Operation,
}

#[function_component]
pub fn SimpleOperation(props: &SimpleOperationProps) -> Html {
	let (state, dispatch) = use_store::<State>();
	let operation = &props.operation;

	html! {
		<a class="panel-block" id={format!("operation{}", operation.id)}>
			// <span class="panel-icon">
			// 	<i class="fas fa-hashtag"></i>
			// </span>
			<div>
				{operation.name.clone()}{" "}
				<span class="has-text-grey">
				if operation.floating {
					<i class="fas fa-calculator ml-1" style="color: #363636;"></i>
				} else {
					{operation.duration} {" мин"}
				}
				// {" - "}
				// {short_time(&operation.end_time.clone())}
				</span>
				
				<div>
					<b>{operation.plan_count}{" "}{operation.unit.clone()}</b>
					if let Some(pass_count) = operation.pass_count {
						{" / "}<b>{pass_count}{" "}{operation.unit.clone()}</b>
					}
					if let Some(accept_count) = operation.accept_count {
						{" / "}<b>{accept_count}{" "}{operation.unit.clone()}</b>
					}
					<span class="ml-1">{operation.single_price}{"₽"}</span>
				</div>
			</div>
		</a>
	}
}