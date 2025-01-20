use axum::{
    routing::{get, post},
    Router,
};

use crate::{user, ArcAppState};

pub fn init(state: ArcAppState) -> Router {
    let auth_router = auth_init(state.clone());

    Router::new().nest("/auth", auth_router)
}

fn auth_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/register/user", post(user::handler::register))
        .route("/active/user/{code}", get(user::handler::activate))
        .with_state(state)
}
