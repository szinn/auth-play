use std::time::Duration;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::timeout::TimeoutLayer;

use super::session::SessionAdapter;

pub(crate) fn get_routes(session_adapter: SessionAdapter) -> Router<()> {
    axum::Router::new()
        .route("/session", get(self::get::session))
        .route("/register", post(self::post::register))
        .route("/login", post(self::post::login))
        .with_state(session_adapter)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

mod get {
    use axum::{extract::State, Json};
    use serde::Serialize;

    use crate::{http::session::adapter::AuthSession, ApiError};

    use super::SessionAdapter;

    #[derive(Debug, Serialize)]
    pub(crate) struct SessionResponse {
        pub name: Option<String>,
        pub email: Option<String>,
    }

    #[tracing::instrument(level = "trace", skip(_session_adapter))]
    pub async fn session(auth_session: AuthSession, State(_session_adapter): State<SessionAdapter>) -> Result<Json<SessionResponse>, ApiError> {
        let response = SessionResponse {
            name: None,
            email: None,
            //     email: Some("foo@bar.com".to_string()),
            //     name: Some("Foo Bar".to_string()),
        };

        Ok(Json(response))
    }
}

mod post {
    use auth_domain_models::auth::NewUser;
    use axum::{
        extract::State,
        response::{IntoResponse, Redirect},
        Json,
    };
    use serde::{Deserialize, Serialize};
    use tower_sessions::Session;

    use crate::ApiError;

    use super::SessionAdapter;

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct RegisterRequest {
        name: String,
        email: String,
        password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct LoginRequest {
        email: String,
        password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct LoginResponse {
        result: String,
        message: Option<String>,
    }

    #[tracing::instrument(level = "trace", skip(session_adapter))]
    pub async fn register(State(session_adapter): State<SessionAdapter>, Json(payload): Json<RegisterRequest>) -> Result<impl IntoResponse, ApiError> {
        let new_user = NewUser {
            name: payload.name.clone(),
            email: payload.email.clone(),
            password: payload.password.clone(),
        };

        let user = session_adapter.auth_api.register(&new_user).await?;
        tracing::info!("Got user {:?} from registering", user);
        Ok(Redirect::to("/app").into_response())
    }

    #[tracing::instrument(level = "trace", skip(session_adapter))]
    pub async fn login(
        session: Session,
        State(session_adapter): State<SessionAdapter>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, ApiError> {
        Ok(Json(LoginResponse {
            result: "error".to_string(),
            message: Some("Not implemented".to_string()),
        }))
    }
}
