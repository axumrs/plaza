use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{mid, user, ArcAppState};

pub fn init(state: ArcAppState) -> Router {
    let auth_router = auth_init(state.clone());

    Router::new()
        .nest("/auth", auth_router)
        .layer(middleware::from_extractor::<mid::HttpClient>())
}

fn auth_init(state: ArcAppState) -> Router {
    Router::new()
        .route("/register/user", post(user::handler::register))
        .route("/login/user", post(user::handler::login))
        .route("/active/user/{code}", get(user::handler::activate))
        .with_state(state)
}
