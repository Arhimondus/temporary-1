use web_sys::HtmlInputElement;
use yew::{prelude::*};
use yew_hooks::{use_async, use_local_storage, UseLocalStorageHandle};
use yewdux::{prelude::*, storage};
use yew_router::{prelude::*, navigator};
use enclose::enclose;
use crate::{utils::{save_to_app_storage, clear_app_storage, system_navigate, register_fcm_token, log_str}, models::{self, AuthResult}, api::{self, TasksCount}, State, router::Route};

const APP_VERSION: u32 = 90;

#[function_component]
pub fn Auth() -> Html {
	let (state, dispatch) = use_store::<State>();
	let navigator = use_navigator().unwrap();

	let login = use_state(|| AttrValue::Rc("artemiy1".into()));
	let password = use_state(|| AttrValue::Rc("12345z".into()));

	let login_async = use_async(enclose!((login, password) async move {
		log_str(&login);
		log_str(&password);
		api::login(&models::Auth {
			login: login.to_string(),
			password: password.to_string(),
		}).await.or_else(|e| Err(e.error))
	}));

	let tasks_count_async = use_async(enclose!((state) async move {
		api::tasks_count(&state.user_session_id.clone().unwrap()).await.or_else(|e| Err(e.error))
	}));

	let add_test_task_ffff = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "ffff").await
	}));

	let add_test_task_ffggg = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "ffggg").await
	}));

	let add_test_task_fgfgg = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "fgfgg").await
	}));

	let add_test_task_fgggfffg = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "fgggfffg").await
	}));

	let add_test_task_gfffgggf = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "gfffgggf").await
	}));

	let add_test_task_gfgff = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "gfgff").await
	}));

	let add_test_task_ggfff = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "ggfff").await
	}));

	let add_test_task_gggg = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "gggg").await
	}));

	let add_test_task_g15 = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "g15").await
	}));

	let add_test_task_g4medium = use_async(enclose!((state) async move {
		api::add_test_task(&state.user_session_id.as_ref().unwrap(), "g4medium").await
	}));
	
	use_effect_with_deps(enclose!((navigator) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::NewTasks);
		}
	}), add_test_task_gggg.data);
	
	use_effect_with_deps(enclose!((navigator) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::NewTasks);
		}
	}), add_test_task_ffff.data);

	use_effect_with_deps(enclose!((navigator) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::NewTasks);
		}
	}), add_test_task_fgggfffg.data);

	use_effect_with_deps(enclose!((navigator) move |data| {
		if let Some(data) = data {
			navigator.push(&Route::NewTasks);
		}
	}), add_test_task_g4medium.data);

	use_effect_with_deps(enclose!((dispatch, tasks_count_async) move |data: &Option<AuthResult>| {
		log_str("sss101");
		if let Some(data) = data {
			log_str("sss102");
			let session_id: String = data.session_id.clone();
			dispatch.reduce_mut(|state: &mut State| {
				state.user_session_id = Some(session_id.clone());
				state.user = Some(data.user.clone());
				save_to_app_storage("session_id".into(), &session_id.clone());
				register_fcm_token();
			});
			log_str("sss103");
			tasks_count_async.run();
		}
	}), login_async.data.clone());

	let submit = Callback::from(enclose!((login_async) move |_e| {
		login_async.run();
	}));

	let exit = Callback::from(enclose!((dispatch) move |_e| {
		dispatch.reduce_mut(|state: &mut State| { 
			state.user_session_id = None;
			state.user = None;
			clear_app_storage("session_id".into());
		});
	}));

	let to_system_menu = Callback::from(move |e| {
		system_navigate("alarm_tester");
	});

	use_effect_with_deps(enclose!((dispatch) move |data: &Option<TasksCount>| {
		if let Some(data) = data {
			dispatch.reduce_mut(|state: &mut State| { 
				// FIX K4jMBk
				state.taken_tasks_count = data.taken_tasks as i8 - if state.active_task.is_some() { 1 } else { 0 };
				state.new_tasks_count = data.new_tasks as i8;
			});
		}
	}), tasks_count_async.data.clone());

	if let Some(user) = &state.user {
		html! {
			<div class="section py-5">
				<h2 class="subtitle is-4" onclick={to_system_menu}>{"Версия v"}{APP_VERSION}</h2>
				<span class="heading">{"сессия "}{state.user_session_id.clone()}</span>
				<h1 class="title is-5 is-flex is-justify-content-space-between">
					{user.name.clone()}
					<button class="button" onclick={exit}>{"Выход"}<i class="ml-2 fas fa-sign-out-alt"></i></button>
				</h1>
				<h2>{"Специализация: "}{user.specialization_name.clone()}</h2>
				if user.debug_mode {
					<div class="is-flex is-flex-direction-column is-align-items-center pt-5" style="gap: 10px;">
						<button class="button is-light" onclick={Callback::from(move |_e| {
							add_test_task_gggg.run();
						})}>{"Выдать обычную задачу"}</button>
						<button class="button is-light" onclick={Callback::from(move |_e| {
							add_test_task_g4medium.run();
						})}>{"Выдать обычную медиум задачу"}</button>
						<button class="button is-light" onclick={Callback::from(move |_e| {
							add_test_task_g15.run();
						})}>{"Выдать огромную обычную задачу"}</button>
						<button class="button is-dark" style="background-color: #6a6868;" onclick={Callback::from(move |_e| {
							add_test_task_ffff.run();
						})}>{"Выдать нормировочную задачу"}</button>
						<button class="button is-info" style="background-color: #7ca1c7;" onclick={Callback::from(move |_e| {
							add_test_task_fgggfffg.run();
						})}>{"Выдать смешанную задачу"}</button>
					</div>
				}
			</div>
		}
	} else {
		html! {
			<div class="section py-5">
				<h2 class="subtitle is-3" onclick={to_system_menu}>{"Версия v"}{APP_VERSION}</h2>
				<h1 class="title is-5">{"Авторизация"} 
					// <button onclick={to_system_menu}>{"Системное меню"}</button>
				</h1>
				// <span class="heading">{"сессия "}{state.user_session_id.clone()}</span>
				<form style="display: flex; flex-direction: column;">
					<div class="field">
						<label class="label">{"Логин"}</label>
						// {" ("}{login.to_string()}{")"}
						<div class="control has-icons-left">
							<input class="input" type="text" oninput={Callback::from(move |e: InputEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								// let login = login.clone();
								login.set(AttrValue::Rc(input.value().into()));
							})}/>
							<span class="icon is-small is-left"><i class="fa fa-user"/></span>
						</div>
					</div>
					<div class="field">
						<label class="label">{"Пароль"}</label>
						// {" ("}{password.to_string()}{")"}
						<div class="control has-icons-left">
							<input class="input" type="password" oninput={Callback::from(move |e: InputEvent| {
								let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
								let password = password.clone();
								password.set(AttrValue::Rc(input.value().into()));
							})}/>
							<span class="icon is-small is-left"><i class="fa fa-key"/></span>
						</div>
					</div>
				</form>
				<button class="button is-info mt-3" onclick={submit}>{"Войти"}</button>
				if let Some(err) = &login_async.error {
					<div class="notification is-danger mt-3">
						{err}
					</div>
				}
			</div>
		}
	}
}