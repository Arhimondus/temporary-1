use pub_this::pub_this;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::*;
use yew_router::prelude::*;
use enclose::enclose;

use crate::{utils::{error_message, warning_message, cancel_all_alarms}, state::State, router::Route, api};

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct CancelTaskProps {
	task_id: u32,
}

#[function_component]
pub fn CancelTask(props: &CancelTaskProps) -> Html {
	let comment = use_state(|| AttrValue::Rc("".into()));
	let navigator = use_navigator().unwrap();
	let (state, dispatch) = use_store::<State>();
	let Some(active_task) = state.active_task.as_ref() else { return html! { <Redirect<Route> to={Route::WorkTask}/> } };
	let task = &active_task.task;

	let cancel_action =  use_async(enclose!((state, task, comment) async move {
		api::cancel_task(&state.user_session_id.as_ref().unwrap(), task.id, comment.as_str()).await
	}));

	let submit = Callback::from(enclose!((cancel_action, comment, navigator) move |_e| {
		if comment.is_empty()  {
			warning_message("Ввод комментария", "Необходимо обязательно ввести комментарий!");
			return;
		}
		cancel_action.run();
	}));

	use_effect_with_deps(enclose!((cancel_action, dispatch) move |(loading, data, error)| {
		if let Some(error) = error {
			error_message("Ошибка", "Не удаётся завершить задачу, проверьте наличие Интернета");
			return;
		}
		if !loading && let Some(data) = data && *data {
			dispatch.reduce_mut(|state: &mut State| {
				cancel_all_alarms(state);
				state.active_task = None;
				state.finished_tasks_count += 1;
				error_message("Завершение", "Вы завершили задачу");
			});

			navigator.push(&Route::WorkTask);
		}
	}), (cancel_action.loading, cancel_action.data, cancel_action.error.clone()));

	html! {
		<div class="section">
			<h1 class="title is-5">
				<div class="has-text-grey is-size-6">{"Досрочно завершить задачу №"}{task.id}</div>
				{task.name.clone()}
			</h1>

			<form style="display: flex; flex-direction: column;">
				<div class="field">
					<label class="label">{"Комментарий"}</label>
					<div>
						<div class="control has-icons-left" style="width: 100%;">
							<input class="input" type="text" oninput={Callback::from(move |e: InputEvent| {
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
				<button class="button is-success mt-3" onclick={submit}>{"Завершить задачу"}</button>
			</div>
		</div>
	}
}