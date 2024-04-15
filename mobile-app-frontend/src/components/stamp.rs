use pub_this::pub_this;
use yew::{html, function_component, Html, Properties, Callback};
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{models::*, State, utils::short_time, router::Route, CurrentOperation};

use super::{FloatActiveOperationActions, GeneralActiveOperationActions};

#[derive(Clone, Copy, PartialEq)]
pub enum StampType {
	Info,
	Zero,
	Partial,
	Full,
}

#[derive(Properties, PartialEq)]
#[pub_this]
pub struct StampProps {
	r#type: StampType,
	title: String,
}

#[function_component]
pub fn Stamp(props: &StampProps) -> Html {
	return html! {
		<div class={match props.r#type {
			StampType::Info => "stamp-info",
			StampType::Zero => "stamp-zero",
			StampType::Partial => "stamp-partial",
			StampType::Full => "stamp-full",
		}}>{&props.title}</div>
	};
}