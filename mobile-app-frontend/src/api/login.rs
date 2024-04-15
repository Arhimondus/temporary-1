use reqwest::{Error, Body};
use crate::{models::*, utils::{post_body, post_unauth, get, ApiError, log_str}, log};

pub async fn login(auth: &Auth) -> Result<AuthResult, ApiError> {
	log_str("login1");
	log_str("login2");
	post_unauth::<AuthResult, Auth>("/auth/login", auth).await
}