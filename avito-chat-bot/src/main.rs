use std::{borrow::{Borrow, BorrowMut}, cell::Cell, fmt::{Debug, Error}, future::IntoFuture, pin::Pin, process::Command, rc::Rc, sync::{Arc, Mutex}, thread, vec};
use dotenv::dotenv;
use log_derive::{logfn, logfn_inputs};
use actix_web::{get, post, web::{self, head, Json, Path}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use maplit::hashmap;
use reqwest::{self, Client, StatusCode, Url};
use enclose::enclose;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use nestify::nest;
use log::{debug, error, log_enabled, info, Level};
mod avito;
use avito::*;
mod models;
use models::*;
use teloxide::{prelude::*, utils::command::BotCommands};

#[macro_use]
extern crate dotenv_codegen;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ChatAnswer {
	text: String,
	value: String,
	printable: bool,
}

// #[derive(Clone)]
// struct ChatStory {
// 	user_id: i64,
// 	chat_id: String,
// 	current_index: Option<usize>,
// 	chat_elements: Vec<AvQuestion>,
// 	last_message: String,
// }

struct DbHelper;

impl DbHelper {
	async fn set_answer(pool: &PgPool, ad_id: i64, user_id: i64, question_id: i32, answer: &str) {
		sqlx::query("INSERT INTO av_answers (ad_id, user_id, question_id, answer) VALUES($1, $2, $3, $4) ON CONFLICT ON CONSTRAINT av_answers_pk DO UPDATE SET answer = $4")
			.bind(ad_id)
			.bind(user_id)
			.bind(question_id)
			.bind(answer.to_string())
			.fetch_optional(pool).await.unwrap();
	}

	async fn change_current_question_id(pool: &PgPool, ad_id: i64, user_id: i64, current_question_id: i32, chat_id: &str) {
		info!("change_current_question_id {ad_id} {user_id} {current_question_id} {chat_id}");
		sqlx::query("INSERT INTO av_questionnaires (user_id, ad_id, current_question_id, chat_id) VALUES($1, $2, $3, $4) ON CONFLICT ON CONSTRAINT av_questionnaires_pk DO UPDATE SET current_question_id = $3")
			.bind(user_id)
			.bind(ad_id)
			.bind(current_question_id)
			.bind(chat_id.to_string())
			.fetch_optional(pool).await.unwrap();
	}

	async fn get_questionnaire(pool: &PgPool, ad_id: i64, user_id: i64) -> AvQuestionnaires {
		let questionnaire: AvQuestionnaires = sqlx::query_as("SELECT * FROM av_questionnaires WHERE ad_id = $1 AND user_id = $2 LIMIT 1")
			.bind(ad_id)
			.bind(user_id)
			.fetch_one(pool).await.unwrap();
		questionnaire
	}

	async fn get_questionnaire_by_chat_id(pool: &PgPool, chat_id: &str) -> AvQuestionnaires {
		let questionnaire: AvQuestionnaires = sqlx::query_as("SELECT * FROM av_questionnaires WHERE chat_id = $1 LIMIT 1")
			.bind(chat_id.to_string())
			.fetch_one(pool).await.unwrap();
		questionnaire
	}

	async fn get_questions(pool: &PgPool, ad_id: i64) -> Vec<AvQuestions> {
		let questions: Vec<AvQuestions> = sqlx::query_as("SELECT * FROM av_questions WHERE ad_id = $1 ORDER BY order_id")
			.bind(ad_id)	
			.fetch_all(pool).await.unwrap();
		questions
	}

	async fn get_answers_by_ad_id_and_user_id(pool: &PgPool, ad_id: i64, user_id: i64) -> Vec<AvAnswers> {
		let answers: Vec<AvAnswers> = sqlx::query_as("SELECT av_answers.*, av_questions.mnemo as question_mnemo, av_questions.name as question_name FROM av_answers LEFT JOIN av_questions ON av_questions.id = av_answers.question_id WHERE av_answers.ad_id = $1 AND av_answers.user_id = $2 ORDER BY av_questions.order_id")
			.bind(ad_id)
			.bind(user_id)
			.fetch_all(pool).await.unwrap();
		answers
	}

	async fn get_first_questions(pool: &PgPool, ad_id: i64) -> AvQuestions {
		let question: AvQuestions = sqlx::query_as("SELECT * FROM av_questions WHERE ad_id = $1 ORDER BY order_id LIMIT 1")
			.bind(ad_id)	
			.fetch_one(pool).await.unwrap();
		question
	}

	async fn get_current_question(pool: &PgPool, ad_id: i64, user_id: i64) -> Option<AvQuestions> {
		let questionnaire = DbHelper::get_questionnaire(pool, ad_id, user_id).await;
		if let Some(question_id) = questionnaire.current_question_id {
			let question: AvQuestions = sqlx::query_as("SELECT * FROM av_questions WHERE id = $1 LIMIT 1")
			.bind(question_id)	
			.fetch_one(pool).await.unwrap();
			Some(question)
		} else {
			None
		}
	}

	async fn clear_answers(pool: &PgPool, ad_id: i64, user_id: i64) {
		sqlx::query("DELETE av_answers WHERE ad_id = $1 and user_id = $2")
			.bind(ad_id)	
			.bind(user_id)
			.fetch_optional(pool).await.unwrap();
	}

	async fn get_ad(pool: &PgPool, ad_id: i64) -> AvAds {
		let ad: AvAds = sqlx::query_as("SELECT * FROM av_ads WHERE id = $1 LIMIT 1")
			.bind(ad_id)
			.fetch_one(pool).await.unwrap();
		ad
	}
}

struct Logic;

impl Logic {
	async fn get_current_user_id_and_ad_id_from_chat(http_client: &reqwest::Client, access_token: &str, chat_id: &str) -> (i64, i64) {
		let avito_main_user_id: i64 = dotenv!("AVITO_MAIN_USER_ID").parse().unwrap();
		let chat: AvitoChatResponse = avito_get::<AvitoChatResponse>(&format!("/messenger/v2/accounts/{}/chats/{}", avito_main_user_id, chat_id), &access_token, &http_client).await.unwrap();
		let current_user_id = chat.users.iter().find(|it| it.id != avito_main_user_id).unwrap().id;
		let ad_id = chat.context.value.id;
		(current_user_id, ad_id)
	}

	async fn clear(_http_client: &reqwest::Client, pool: &PgPool, _access_token: &str,  ad_id: i64, user_id: i64) {
		DbHelper::clear_answers(pool, ad_id, user_id).await;
	}

	async fn restart(http_client: &reqwest::Client, pool: &PgPool, access_token: &str, ad_id: i64, user_id: i64, chat_id: &str) {
		Logic::clear(http_client, pool, access_token, ad_id, user_id).await;
		Logic::start(http_client, pool, access_token, ad_id, user_id, chat_id).await;
	}

	async fn start(http_client: &reqwest::Client, pool: &PgPool, access_token: &str, ad_id: i64, user_id: i64, chat_id: &str) {
		let first_question = DbHelper::get_first_questions(pool, ad_id).await;
		info!("first_question {:?}", first_question);
		DbHelper::change_current_question_id(pool, ad_id, user_id, first_question.id, chat_id).await;
		let questionnaire = DbHelper::get_questionnaire(pool, ad_id, user_id).await;
		info!("questionnaire {:?}", questionnaire);
		send_message(http_client, access_token, &questionnaire.chat_id, &first_question.question).await;
	}

	async fn next_question(http_client: &reqwest::Client, pool: &PgPool, access_token: &str, chat_id: &str, ad_id: i64, user_id: i64, question_id: i32, questions: &Vec<AvQuestions>, bot: &Bot, bot_chat_id: &ChatId) {
		let current_pos = questions.iter().position(|q| q.id == question_id).unwrap();
		let next_question = questions.get(current_pos + 1);
		if let Some(next_question) = next_question {
			DbHelper::change_current_question_id(pool, ad_id, user_id, next_question.id, chat_id).await;
			send_message(http_client, access_token, chat_id, &next_question.question).await;
		} else {
			info!("Вопросы закончились");
			let ad = DbHelper::get_ad(pool, ad_id).await;
			send_message(http_client, access_token, chat_id, &ad.last_message).await;

			let answers = DbHelper::get_answers_by_ad_id_and_user_id(pool, ad_id, user_id).await;
			let fio = answers.iter().find(|it| &it.question_mnemo.clone().unwrap() == "fio").unwrap().answer.clone();
			let phone = answers.iter().find(|it| &it.question_mnemo.clone().unwrap() == "phone").unwrap().answer.clone();
			
			bot.send_message(*bot_chat_id, format!("ФИО: {fio}, телефон: {phone}, подробная информация: https://vrc.d20.ru/workers/chat/{chat_id}")).await.unwrap();
		}
	}

	async fn answer(http_client: &reqwest::Client, pool: &PgPool, access_token: &str, ad_id: i64, user_id: i64, answer: &str, bot: &Bot, bot_chat_id: &ChatId) {
		let questionnaire = DbHelper::get_questionnaire(pool, ad_id, user_id).await;
		if let Some(question_id) = questionnaire.current_question_id {
			// Проверка ответа
			let questions = DbHelper::get_questions(pool, ad_id).await;
			let current_question = questions.iter().find(|it| it.id == question_id).unwrap();
			if let Some(available_answers) = &current_question.available_answers {
				let available_answers = available_answers.as_ref();
				match available_answers.iter().find(|it| it.text.eq(answer)) {
					Some(c) => {
						info!("Пользователь {} ответил на вопрос {} ответом \"{}\", значение = {}", user_id, questionnaire.current_question_id.unwrap(), answer, c.value);
						DbHelper::set_answer(pool, ad_id, user_id, questionnaire.current_question_id.unwrap(), answer).await;

						Logic::next_question(http_client, pool, access_token, &questionnaire.chat_id, ad_id, user_id, question_id, &questions, bot, bot_chat_id).await;
					},
					None => {
						info!("Пользователь {} неверно на вопрос {} ответом \"{}\"", user_id, questionnaire.current_question_id.unwrap(), answer);
						let answers = available_answers.iter().filter(|it| it.printable).map(|it| it.text.clone()).collect::<Vec<String>>().join(" | ");
						send_message(http_client, access_token, &questionnaire.chat_id, &format!("Ответьте однозначно на вопрос - {answers}")).await;
					},
				};
			} else {
				// Значит свободный вопрос
				info!("Получен свободный ответ \"{}\" на вопрос {}", answer, questionnaire.current_question_id.unwrap());
				DbHelper::set_answer(pool, ad_id, user_id, questionnaire.current_question_id.unwrap(), answer).await;

				Logic::next_question(http_client, pool, access_token, &questionnaire.chat_id, ad_id, user_id, question_id, &questions, &bot, bot_chat_id).await;
			}
		} else {
			info!("Тест уже пройден!! Но пользователь {} пытается что-то написать", user_id);
		}
	}
}

nest! {
	#[derive(Serialize, Deserialize, Debug)]*
	struct AvitoCallbackBody {
		#>[derive(Serialize, Deserialize, Debug)]*
		id: String,
		version: String,
		timestamp: i64,

		payload: struct AvitoCallbackPayload {
			r#type: String,
			value: struct AvitoCallbackPayloadValue {
				id: String,
				chat_id: String,
				user_id: i64,
				author_id: i64,
				created: i64,
				r#type: String,
				chat_type: String,
				content: struct AvitoCallbackPayloadValueContent {
					text: String,
				},
				item_id: i64,
			},
		},
	}
}

#[derive(Debug)]
struct AvitoMessage {
	message_id: String,
	chat_id: String,
	user_id: i64,
	ad_id: i64,
	text: String,
	created: i64,
}

impl Into<AvitoMessage> for AvitoCallbackBody {
	fn into(self: AvitoCallbackBody) -> AvitoMessage {
		AvitoMessage {
			message_id: self.id,
			chat_id: self.payload.value.chat_id,
			user_id: self.payload.value.author_id,
			ad_id: self.payload.value.item_id,
			text: self.payload.value.content.text,
			created: self.payload.value.created,
		}
	}
}

nest! {
	#[derive(Serialize, Deserialize, Debug)]*
	struct AvitoSendMessageBody {
		#>[derive(Serialize, Deserialize, Debug)]*
		r#type: String,
		message: struct AvitoSendMessageMessage {
			text: String,
		},
	}
}

nest! {
	#[derive(Serialize, Deserialize, Debug)]
	struct AvitoSendMessageResponse {
		#>[derive(Serialize, Deserialize, Debug)]
		content: struct AvitoSendMessageResponseContent {
			text: String,
		},
		created: i64,
		direction: String,
		id: String,
		r#type: String,
	}
}

nest! {
	#[derive(Serialize, Deserialize, Debug)]
	struct AvitoChatResponse {
		#>[derive(Serialize, Deserialize, Debug)]*
		context: struct AvitoChatResponseContext {
			r#type: String,
			value: struct AvitoChatResponseContextValue {
				id: i64,
				status_id: i32,
				title: String,
				url: String,
				user_id: i64,
			},
		},
		#>[derive(Serialize, Deserialize, Debug)]*
		users: Vec<struct AvitoChatResponseUser  {
			id: i64,
			name: String,
		}>,
	}
}

#[logfn(Info)]
async fn send_message(http_client: &reqwest::Client, access_token: &str, chat_id: &str, message: &str) -> AvitoSendMessageResponse {
	let response: AvitoSendMessageResponse = avito_json(&format!("/messenger/v1/accounts/{}/chats/{}/messages", dotenv!("AVITO_MAIN_USER_ID"), chat_id), AvitoSendMessageBody {
		r#type: "text".to_string(),
		message: AvitoSendMessageMessage {
			text: message.to_string(),
		},
	}, access_token, http_client).await.unwrap();

	response
}

#[logfn(Info)]
async fn register_web_hook(http_client: &reqwest::Client, access_token: &str) -> SimpleOkResponse {
	let response: SimpleOkResponse = avito_json("/messenger/v3/webhook", &hashmap!{
		"url" => dotenv!("CALLBACK_URL"),
	}, access_token, http_client).await.unwrap();

	response
}

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[post("/start-force/{chat_id}")]
async fn echo(request: HttpRequest, path: Path<String>, http_client: web::Data<reqwest::Client>, pool: web::Data<PgPool>, access_token: web::Data<String>) -> impl Responder {
	let chat_id = path.into_inner();
	// let secret_key = request.headers().get("Authorization");
	let (user_id, ad_id) = Logic::get_current_user_id_and_ad_id_from_chat(&http_client, &access_token, &chat_id).await;
	
	Logic::start(&http_client, &pool, &access_token, ad_id, user_id, &chat_id).await;

	HttpResponse::Ok().body("started")
}

async fn start_chat(pool: &PgPool, ad_id: i64, user_id: i64, chat_id: &str) {
	sqlx::query("INSERT INTO av_questionnaires(user_id, ad_id, chat_id) VALUES($1, $2, $3)
	ON CONFLICT ON CONSTRAINT av_questionnaires_pk DO NOTHING;")
		.bind(user_id)
		.bind(ad_id)
		.bind(chat_id)
		.fetch_optional(pool).await.unwrap();
}

async fn change_chat(pool: &PgPool, ad_id: i64, user_id: i64, chat_id: &str, question_id: &str) {
	sqlx::query("INSERT INTO av_questionnaires(user_id, ad_id, chat_id, question_id) VALUES($1, $2, $3, $4)
	ON CONFLICT ON CONSTRAINT av_questionnaires_pk DO UPDATE SET = question_id = $4;")
		.bind(user_id)
		.bind(ad_id)
		.bind(chat_id)
		.bind(question_id)
		.fetch_optional(pool).await.unwrap();
}

async fn save_answer(pool: &PgPool, ad_id: i64, question_id: &str, user_id: i64,answer: &str) {
	sqlx::query("INSERT INTO av_answers(ad_id, question_id, user_id, answer) VALUES($1, $2, $3, $4)
	ON CONFLICT ON CONSTRAINT av_answers_pk DO UPDATE SET answer = $4;")
		.bind(ad_id)
		.bind(question_id)
		.bind(user_id)
		.bind(answer.to_string())
		.fetch_optional(pool).await.unwrap();
}

const START_COMMAND: &str = "/start";
const RESTART_COMMAND: &str = "/restart";

#[post("/callback")]
async fn callback(req_body: Json<AvitoCallbackBody>, pool: web::Data<PgPool>, http_client: web::Data<reqwest::Client>, access_token: web::Data<String>, bot: web::Data<Arc<Bot>>, bot_chat_id: web::Data<Arc<Mutex<ChatId>>>) -> impl Responder {
	let avito_main_user_id: i64 = dotenv!("AVITO_MAIN_USER_ID").parse().unwrap();

	debug!("callback {:?}", req_body.payload.value);

	let message: AvitoMessage = req_body.into_inner().into();
	debug!("{:?}", message);

	if message.ad_id != 2673181747 {
		debug!("Не подсобник, а другое объявление - {}", message.ad_id);
		return HttpResponse::Ok().body("Игнорируем объявления не подсобников");
	}

	if message.user_id == avito_main_user_id || message.user_id == 0 {
		info!("Это сообщение от аккаунта владельца или от бота - \"{}\"", message.text);
		let chat: AvitoChatResponse = avito_get::<AvitoChatResponse>(&format!("/messenger/v2/accounts/{}/chats/{}", avito_main_user_id, &message.chat_id), &access_token, &http_client).await.unwrap();
		let current_user_id = chat.users.iter().find(|it| it.id != avito_main_user_id).unwrap().id;
		let ad = DbHelper::get_ad(&pool, message.ad_id).await;
		
		info!("trigger_message = {}", ad.trigger_message);

		if message.text == START_COMMAND || message.text == ad.trigger_message {
			info!("Команда старт");
			Logic::start(&http_client, &pool, &access_token, message.ad_id, current_user_id, &message.chat_id).await;
			return HttpResponse::Ok().body("Ok");
		} else if message.text == RESTART_COMMAND {
			info!("Команда рестарт");
			Logic::restart(&http_client, &pool, &access_token, message.ad_id, current_user_id, &message.chat_id).await;
			return HttpResponse::Ok().body("Ok");
		} else {
			println!("Ignoring AVITO_MAIN_USER_ID account!");
			return HttpResponse::Ok().body("Ignoring self message");
		}
	}

	// TODO: Убрать ниже!
	// if message.user_id != 117220655 {
	// 	println!("Ignoring message other than an Artemiy account");
	// 	return HttpResponse::Ok().body("Ignoring message other than an Artemiy account!");
	// }

	// Пользовательское сообщение
	let chat_id = bot_chat_id.lock().unwrap().clone();
	Logic::answer(&http_client, &pool, &access_token, message.ad_id, message.user_id, &message.text, &bot, &chat_id).await;
	HttpResponse::Ok().body("")
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

nest! {
	#[derive(Serialize, Deserialize, Debug)]
	struct SubscriptionsResponse {
		#>[derive(Serialize, Deserialize, Debug)]
		subscriptions: Vec<struct Subscription {
			url: String,
			version: String,
		}>,
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct SimpleOkResponse {
	ok: bool,
}

async fn get_access_token_from_db(pool: &PgPool) -> String {
	let row: (String, i32, String) = sqlx::query_as(
		"
SELECT * FROM avito LIMIT 1
		"
	)
	.fetch_one(pool) // -> Vec<{ country: String, count: i64 }>
	.await.unwrap();

	let access_token = row.0;
	access_token
}

#[derive(Serialize, Deserialize, Debug)]
struct UnsubscribeParams {
	url: String,
}

async fn get_access_token(pool: &PgPool, http_client: &reqwest::Client) -> String {
	let access_token = get_access_token_from_db(pool).await;

	if let Ok(SubscriptionsResponse { subscriptions }) = avito("/messenger/v1/subscriptions", &access_token, http_client).await {
		for subscription in subscriptions {
			avito_json::<UnsubscribeParams, SimpleOkResponse>("/messenger/v1/webhook/unsubscribe", UnsubscribeParams { url: subscription.url }, &access_token, http_client).await.unwrap();
		}

		info!("access_token ({access_token}) из базы верный! Возвращаем его.");
		return access_token;
	} else {
		info!("access_token ({access_token}) неверный, делаем запрос на получение нового.");
		let access_token = avito_token(http_client).await;
		info!("Получен новый access_token ({access_token}).");
		sqlx::query("
UPDATE avito SET access_token = $1
		")
			.bind(&access_token)
			.fetch_optional(pool)
			.await
			.unwrap();
		info!("Новый access_token ({access_token}) записан в базу данных.");

		let SubscriptionsResponse { subscriptions } = avito("/messenger/v1/subscriptions", &access_token, http_client).await.unwrap();

		for subscription in subscriptions {
			avito_json::<UnsubscribeParams, SimpleOkResponse>("/messenger/v1/webhook/unsubscribe", UnsubscribeParams { url: subscription.url }, &access_token, http_client).await.unwrap();
		}
		return access_token;
	}
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum BotCommand {
	#[command(description = "start.")]
	Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    match cmd {
		BotCommand::Start => {
			println!("msg chat id {}", msg.chat.id);
			bot.send_message(msg.chat.id, msg.chat.id.to_string()).await?;
			bot.send_dice(msg.chat.id).await?;
			bot.send_message(msg.chat.id, "Test1").await?;
		}
        BotCommand::Help => {
			bot.send_message(msg.chat.id, BotCommand::descriptions().to_string()).await?;
		},
        BotCommand::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?;
        }
        BotCommand::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?;
        }
    };

    Ok(())
}

const ART_CHAT_ID: i64 = 6228792868;
const JAN_CHAT_ID: i64 = 987314482;

#[derive(Deserialize)]
struct CustomSendBody {
	message: String,
	chat_id: i64,
}

#[derive(Deserialize)]
struct ChangeReceiverBody {
	chat_id: i64,
}

#[post("/change-receiver")]
async fn change_receiver(req_body: Json<ChangeReceiverBody>, bot_chat_id: web::Data<Arc<Mutex<ChatId>>>) -> impl Responder {
	let mut bot_chat_id = bot_chat_id.lock().unwrap();

	bot_chat_id.0 = req_body.chat_id;

	HttpResponse::Ok().body("ok")
}

#[post("/custom-send")]
async fn custom_send(req_body: Json<CustomSendBody>, bot: web::Data<Arc<Bot>>) -> impl Responder {
	bot.send_message(ChatId(req_body.chat_id), &req_body.message).await.unwrap();
	HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
	env_logger::init();

	let bot = Arc::new(Bot::new(dotenv!("TELOXIDE_TOKEN")));
	let bot_chat_id = Arc::new(Mutex::new(ChatId(ART_CHAT_ID)));

	// let clonned_bot = bot.clone();
	
	// actix_rt::spawn(async move {
	// 	BotCommand::repl(clonned_bot, answer).await;
	// });
	
	let mut db_connection_string = Url::parse(dotenv!("DB_CONNECTION_STRING")).unwrap();
	let _ = db_connection_string.set_username(dotenv!("DB_USER"));
	let _ = db_connection_string.set_password(Some(dotenv!("DB_PASSWORD")));
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(db_connection_string.as_str()).await.unwrap();

	// let answers = DbHelper::get_answers_by_ad_id_and_user_id(&pool, 2673181747, 317670664).await;
	// println!("{:?}", answers);

	let http_client = web::Data::new(reqwest::Client::new());
	let access_token = web::Data::new(get_access_token(&pool, &http_client).await);

	// TODO: Раскомментить
	register_web_hook(&http_client, &access_token).await;

	let _ = HttpServer::new(enclose!((http_client) move || {
		App::new()
			.app_data(http_client.clone())
			.app_data(access_token.clone())
			// .app_data(app_state.clone())
			.app_data(web::Data::new(pool.clone()))
			.app_data(web::Data::new(bot.clone()))
			.app_data(web::Data::new(bot_chat_id.clone()))
			.service(hello)
			.service(echo)
			.service(callback)
			.service(change_receiver)
			.service(custom_send)
			.route("/hey", web::get().to(manual_hello))
	}))
		.bind(("0.0.0.0", 8080))?
		.run()
		.await;

	// let _ = http_client.get("https://ya.ru").send().await.unwrap();

	println!("Program has finished!");

	Ok(())
}

#[cfg(test)]
mod tests {
	use actix_web::{http::header::ContentType, test, App};
    use super::*;

	#[actix_web::test]
	async fn test_index_get() {
		let app = test::init_service(App::new().service(index)).await;
		let req = test::TestRequest::default()
			.insert_header(ContentType::plaintext())
			.to_request();
		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success());
	}
}