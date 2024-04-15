use std::ops::Deref;

use yew::{html, function_component, Html, Properties, use_state, UseStateHandle, use_effect_with_deps, use_effect};
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::Link;
use yewdux::{prelude::use_store, log::debug};
use enclose::enclose;

use crate::{models::*, State, api, router::Route};

#[function_component]
fn BottomNav() -> Html {
	html! {
		<footer class="tabs">
			<ul>
				<li><Link<Route> to={Route::WorkTask}>
					{"Задача"}
				</Link<Route>></li>
				<li><Link<Route> to={Route::WorkInstruments}>
					{"Инструменты"}
				</Link<Route>></li>
				<li><Link<Route> to={Route::WorkMaterials}>
					{"Материалы"}
				</Link<Route>></li>
			</ul>
		</footer>
	}
}

#[derive(Properties, PartialEq)]
struct TaskRowProps {
	task: Task,
}

#[function_component]
fn TaskRow(props: &TaskRowProps) -> Html {
	let task = props.task.to_owned();
	let start_time = task.start_time();
	let end_time = task.end_time();
	html! {
		<li class="card task-card">
			<a href="/new-tasks/{task.id}">
				<div class="card-content">
					<div class="media">
						<div class="media-content">
							<p class="title is-4">{task.name}</p>
							<p class="subtitle is-6">{start_time}{" - "}{end_time}{" #"}{task.id}</p>
						</div>
					</div>
				</div>
			</a>
		</li>	
	}
}

#[function_component]
pub fn WorkMaterials() -> Html {
	let (state, dispatch) = use_store::<State>();
	let materials: UseStateHandle<Option<Vec<Material>>> = use_state(|| None);

	let clonned_state = state.clone();
	let materials_async = use_async(enclose!((state) async move {
		api::materials(&clonned_state.user_session_id.clone().unwrap(), state.active_task.as_ref().unwrap().task.id).await.or_else(|e| Err(e.error))
	}));

	let clonned_materials_async = materials_async.clone();
	use_effect_once(move || {
		clonned_materials_async.run();
		|| debug!("Running clean-up of effect on unmount work materials")
	});

	let clonned_materials = materials.clone();
	let clonned_materials_async = materials_async.clone();
	use_effect_with_deps(move |data| {
		let data = &data.data;
		clonned_materials.set(data.clone());
	}, clonned_materials_async);

	html! {
		<div class="section py-5">
		<h1 class="title">{"Материалы"}</h1>
		if let Some(materials) = materials.deref().clone() {
			if materials.len() > 0 {
				<table class="table">
					<tr>
						<th>{"Оп."}</th>
						<th>{"Название"}</th>
						<th>{"Колич."}</th>
						<th>{"Ед. изм."}</th>
					</tr>
					{materials.into_iter().map(|i| html!{<tr>
						<td>{i.operation_id}</td>
						<td>{i.name}</td>
						<td>{i.count}</td>
						<td>{i.unit}</td>
					</tr>}).collect::<Vec<_>>()}
				</table>
			} else {
				{"Нет выданных материалов"}
			}
		}
		</div>
	}
}