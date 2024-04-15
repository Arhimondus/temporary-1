use std::collections::HashMap;
use std::{borrow::{Borrow, BorrowMut}, cell::Cell, fmt::{Debug, Error}, pin::Pin, rc::Rc, sync::{Arc, Mutex}, vec};
use maplit::hashmap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use log::{debug, error, log_enabled, info, Level};
use log_derive::{logfn, logfn_inputs};
use reqwest::StatusCode;

#[logfn(Debug)]
#[logfn_inputs(Debug)]
pub async fn avito_get<T>(url: &str, access_token: &str, http_client: &reqwest::Client) -> Result<T, String> where T: DeserializeOwned, T: Debug {
	info!("avito {}", url);

	let response = http_client.get(format!("{}{}", dotenv!("AVITO_API_URL"), url))
		.header("Authorization", format!("Bearer {}", access_token))
		.send()
		.await
		.unwrap();

	if response.status() == StatusCode::OK {
		let data = response.json::<T>().await.unwrap();
		Ok(data)
	} else {
		Err("Wrong access token!".to_string())
	}
}

#[logfn(Debug)]
#[logfn_inputs(Debug)]
pub async fn avito<T>(url: &str, access_token: &str, http_client: &reqwest::Client) -> Result<T, String> where T: DeserializeOwned, T: Debug {
	info!("avito {}", url);

	let response = http_client.post(format!("{}{}", dotenv!("AVITO_API_URL"), url))
		.header("Authorization", format!("Bearer {}", access_token))
		.send()
		.await
		.unwrap();

	if response.status() == StatusCode::OK {
		let data = response.json::<T>().await.unwrap();
		Ok(data)
	} else {
		Err("Wrong access token!".to_string())
	}
}

#[logfn(Debug)]
#[logfn_inputs(Debug)]
pub async fn avito_json<P, T>(url: &str, params: P, access_token: &str, http_client: &reqwest::Client) -> Result<T, String> where T: DeserializeOwned, T: Debug, P: Serialize, P: Debug {
	info!("avito_with {}", url);

	let response = http_client.post(format!("{}{}", dotenv!("AVITO_API_URL"), url))
		.header("Authorization", format!("Bearer {}", access_token))
		.json(&params)
		.send()
		.await
		.unwrap();

	if response.status() == StatusCode::OK {
		let data = response.json::<T>().await.unwrap();
		Ok(data)
	} else {
		Err("Wrong access token!".to_string())
	}
}

#[logfn(Debug)]
#[logfn_inputs(Debug)]
pub async fn avito_form<P, T, X, Z>(url: &str, params: HashMap<X, Z>, access_token: &str, http_client: &reqwest::Client) -> Result<T, String> where T: DeserializeOwned, T: Debug, P: Serialize, Z: Serialize, Z: Debug, X: Serialize, X: Debug, P: Debug {
	info!("avito_with {}", url);

	let response = http_client.post(format!("{}{}", dotenv!("AVITO_API_URL"), url))
		.header("Authorization", format!("Bearer {}", access_token))
		.form(&params)
		.send()
		.await
		.unwrap();

	if response.status() == StatusCode::OK {
		let data = response.json::<T>().await.unwrap();
		Ok(data)
	} else {
		Err("Wrong access token!".to_string())
	}
}

#[derive(Deserialize)]
struct TokenResponse {
	access_token: String,
	expires_in: i32,
	token_type: String,
}

#[logfn(Info)]
pub async fn avito_token(http_client: &reqwest::Client) -> String {
	let response = http_client.post(format!("{}/token", dotenv!("AVITO_API_URL")))
	.form(&hashmap! {
		"grant_type" => "client_credentials".to_string(),
		"client_id" => dotenv!("AVITO_CLIENT_ID").to_string(),
		"client_secret" => dotenv!("AVITO_CLIENT_SECRET").to_string(),
	})
	.send()
	.await
	.unwrap();
	let access_token = response.json::<TokenResponse>().await.expect("Невозможно выписать новый токен").access_token;
	access_token
}