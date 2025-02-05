use axum::{
	extract::State,
	routing::{get, post},
	Json, Router,
};
use stump_core::prelude::{ClaimResponse, StumpVersion};

use crate::{config::state::AppState, errors::ApiResult};

pub(crate) mod auth;
pub(crate) mod epub;
pub(crate) mod filesystem;
pub(crate) mod job;
pub(crate) mod library;
pub(crate) mod log;
pub(crate) mod media;
pub(crate) mod reading_list;
pub(crate) mod series;
pub(crate) mod tag;
pub(crate) mod user;

pub(crate) fn mount(app_state: AppState) -> Router<AppState> {
	Router::new()
		.merge(auth::mount())
		.merge(epub::mount(app_state.clone()))
		.merge(library::mount(app_state.clone()))
		.merge(media::mount(app_state.clone()))
		.merge(filesystem::mount(app_state.clone()))
		.merge(job::mount(app_state.clone()))
		.merge(log::mount(app_state.clone()))
		.merge(series::mount(app_state.clone()))
		.merge(tag::mount(app_state.clone()))
		.merge(user::mount(app_state))
		.merge(reading_list::mount())
		.route("/claim", get(claim))
		.route("/ping", get(ping))
		.route("/version", post(version))
}

#[utoipa::path(
	get,
	path = "/api/v1/claim",
	tag = "util",
	responses(
		(status = 200, description = "Claim status successfully determined", body = ClaimResponse)
	)
)]
async fn claim(State(ctx): State<AppState>) -> ApiResult<Json<ClaimResponse>> {
	let db = ctx.get_db();

	Ok(Json(ClaimResponse {
		is_claimed: db.user().find_first(vec![]).exec().await?.is_some(),
	}))
}

#[utoipa::path(
	get,
	path = "/api/v1/ping",
	tag = "util",
	responses(
		(status = 200, description = "Always responds with 'pong'", body = String)
	)
)]
async fn ping() -> ApiResult<String> {
	Ok("pong".to_string())
}

#[utoipa::path(
	post,
	path = "/api/v1/version",
	tag = "util",
	responses(
		(status = 200, description = "Version information for the Stump server instance", body = StumpVersion)
	)
)]
async fn version() -> ApiResult<Json<StumpVersion>> {
	Ok(Json(StumpVersion {
		semver: env!("CARGO_PKG_VERSION").to_string(),
		rev: std::env::var("GIT_REV").ok(),
		compile_time: env!("STATIC_BUILD_DATE").to_string(),
	}))
}
