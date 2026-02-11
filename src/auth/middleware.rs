//! Authentication middleware

use crate::auth::jwt::JwtService;
use crate::auth::claims::TokenUser;
use crate::core::error::{AppError, AppResult};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Auth middleware state
#[derive(Clone)]
pub struct AuthState {
    pub jwt_service: Arc<JwtService>,
}

impl AuthState {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }
}

/// Authentication middleware
///
/// Validates JWT tokens and adds user info to request extensions
pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            Some(header.trim_start_matches("Bearer ").to_string())
        }
        Some(_) => None,
        None => None,
    };

    // If no token provided, return unauthorized
    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate token
    let claims = match state.jwt_service.validate_token(&token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Extract user from claims
    let user = match TokenUser::from_claims(&claims) {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Add user to request extensions
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

/// Optional authentication middleware
///
/// Validates JWT tokens if present, but doesn't require them
pub async fn optional_auth_middleware(
    State(state): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Response {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            if let Ok(claims) = state.jwt_service.validate_token(token) {
                if let Ok(user) = TokenUser::from_claims(&claims) {
                    req.extensions_mut().insert(user);
                }
            }
        }
    }

    next.run(req).await
}

/// Require admin role middleware
pub async fn require_admin_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get user from request extensions
    let user = req
        .extensions()
        .get::<TokenUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if user is admin
    if !user.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}

/// Require moderator or admin role middleware
pub async fn require_moderator_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get user from request extensions
    let user = req
        .extensions()
        .get::<TokenUser>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if user is moderator or admin
    if !user.is_moderator() {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}

/// Extractor for authenticated user (works with any router state type)
pub struct AuthenticatedUser(pub TokenUser);

impl<S> axum::extract::FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<TokenUser>()
            .cloned()
            .map(AuthenticatedUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

/// Optional extractor for authenticated user
pub struct OptionalAuthUser(pub Option<TokenUser>);

impl<S> axum::extract::FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(OptionalAuthUser(parts.extensions.get::<TokenUser>().cloned()))
    }
}
