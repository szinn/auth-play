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
        .route("/logout", get(self::get::logout))
        .with_state(session_adapter)
        .layer(TimeoutLayer::new(Duration::from_secs(2)))
}

mod get {
    use axum::{
        response::{IntoResponse, Redirect},
        Json,
    };
    use serde::Serialize;

    use crate::{http::session::adapter::AuthSession, ApiError};

    #[derive(Debug, Serialize)]
    pub(crate) struct SessionResponse {
        pub name: Option<String>,
        pub email: Option<String>,
    }

    #[tracing::instrument(level = "trace", skip(auth_session))]
    pub async fn session(auth_session: AuthSession) -> Result<Json<SessionResponse>, ApiError> {
        let response = match auth_session.user {
            Some(user) => SessionResponse {
                name: Some(user.name),
                email: Some(user.email),
            },
            None => SessionResponse { name: None, email: None },
        };

        Ok(Json(response))
    }

    #[tracing::instrument(level = "trace", skip(auth_session))]
    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        let _result = auth_session.logout().await;
        Redirect::to("/app").into_response()
    }
}

mod post {
    use auth_domain_models::auth::NewUser;
    use axum::{
        extract::State,
        response::{IntoResponse, Redirect},
        Json,
    };
    use axum_login::AuthnBackend;
    use serde::{Deserialize, Serialize};

    use crate::{
        http::session::adapter::{AuthSession, Credentials},
        ApiError,
    };

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

        let _user = session_adapter.auth_api.register(&new_user).await?;

        Ok(Redirect::to("/app").into_response())
    }

    #[tracing::instrument(level = "trace", skip(auth_session, session_adapter, payload))]
    pub async fn login(mut auth_session: AuthSession, State(session_adapter): State<SessionAdapter>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
        let credentials = Credentials {
            email: payload.email,
            password: payload.password,
            _next: None,
        };

        let result = session_adapter.authenticate(credentials).await;
        if result.is_err() {
            return Json(LoginResponse {
                result: "error".to_string(),
                message: Some("Error authenticating".to_string()),
            })
            .into_response();
        }
        match result.unwrap() {
            Some(user) => {
                if auth_session.login(&user).await.is_ok() {
                    Redirect::to("/app").into_response()
                } else {
                    Json(LoginResponse {
                        result: "error".to_string(),
                        message: Some("Not implemented".to_string()),
                    })
                    .into_response()
                }
            }
            None => Json(LoginResponse {
                result: "error".to_string(),
                message: Some("Not implemented".to_string()),
            })
            .into_response(),
        }
    }
}
