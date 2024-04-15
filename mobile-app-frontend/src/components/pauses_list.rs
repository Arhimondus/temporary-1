use pub_this::pub_this;
use yew::{html, function_component, Html, Properties};
use yewdux::prelude::use_store;

use crate::{models::*, State};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct PausesListProps {
	operation_result: OperationResult,
}

#[function_component]
pub fn PausesList(props: &PausesListProps) -> Html {
	html! {
		<ul style="display: flex; gap: 5px; flex-direction: column;">
			{props.operation_result.pauses.iter().enumerate().map(|(i, p)| {
				html! {<li style="display: flex; gap: 5px;">
					// <div>{i + 1}{". "}</div>
					<div style="display: flex; flex-direction: column;">
						<div style="font-size: 10px; line-height: 10px;">{p.time()}</div>
						<div style="font-size: 12px;  line-height: 10px;">{p.comment.clone().unwrap_or("".into())}</div>
					</div>
				</li>}
			}).collect::<Vec<_>>()}
		</ul>
	}
}