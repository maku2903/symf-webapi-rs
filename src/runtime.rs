use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, warn};

#[derive(Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub sanitize_nul_chars: bool,
    pub session: Option<SessionConfig>,
    pub timeouts: ClientTimeouts,
}

#[derive(Clone)]
pub struct SessionConfig {
    pub application_key: String,
    pub device_name: String,
    pub max_session_age: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct ClientTimeouts {
    pub request_timeout: Option<Duration>,
    pub connect_timeout: Option<Duration>,
}

impl Default for ClientTimeouts {
    fn default() -> Self {
        Self {
            request_timeout: Some(Duration::from_secs(30)),
            connect_timeout: Some(Duration::from_secs(10)),
        }
    }
}

#[derive(Clone)]
pub struct ClientConfigBuilder {
    base_url: String,
    sanitize_nul_chars: bool,
    session: Option<SessionConfig>,
    timeouts: ClientTimeouts,
}

impl ClientConfig {
    pub fn builder(base_url: impl Into<String>) -> ClientConfigBuilder {
        ClientConfigBuilder {
            base_url: base_url.into(),
            sanitize_nul_chars: false,
            session: None,
            timeouts: ClientTimeouts::default(),
        }
    }

    pub fn new(base_url: impl Into<String>) -> Result<Self, ApiError> {
        let mut base_url = base_url.into().trim().to_owned();
        if base_url.is_empty() {
            return Err(ApiError::InvalidConfig {
                message: "base_url cannot be empty".to_owned(),
            });
        }
        while base_url.ends_with('/') {
            base_url.pop();
        }
        Ok(Self {
            base_url,
            sanitize_nul_chars: false,
            session: None,
            timeouts: ClientTimeouts::default(),
        })
    }

    pub fn with_nul_sanitization(mut self, enabled: bool) -> Self {
        self.sanitize_nul_chars = enabled;
        self
    }

    pub fn with_session(
        mut self,
        application_key: impl Into<String>,
        device_name: impl Into<String>,
    ) -> Result<Self, ApiError> {
        let application_key = application_key.into().trim().to_owned();
        let device_name = device_name.into().trim().to_owned();
        if application_key.is_empty() {
            return Err(ApiError::InvalidConfig {
                message: "application_key cannot be empty".to_owned(),
            });
        }
        if device_name.is_empty() {
            return Err(ApiError::InvalidConfig {
                message: "device_name cannot be empty".to_owned(),
            });
        }
        self.session = Some(SessionConfig {
            application_key,
            device_name,
            max_session_age: Some(Duration::from_secs(25 * 60)),
        });
        Ok(self)
    }

    pub fn with_max_session_age(mut self, max_session_age: Option<Duration>) -> Self {
        if let Some(session) = self.session.as_mut() {
            session.max_session_age = max_session_age;
        }
        self
    }

    pub fn with_request_timeout(mut self, request_timeout: Option<Duration>) -> Self {
        self.timeouts.request_timeout = request_timeout;
        self
    }

    pub fn with_connect_timeout(mut self, connect_timeout: Option<Duration>) -> Self {
        self.timeouts.connect_timeout = connect_timeout;
        self
    }
}

impl ClientConfigBuilder {
    pub fn nul_sanitization(mut self, enabled: bool) -> Self {
        self.sanitize_nul_chars = enabled;
        self
    }

    pub fn session(
        mut self,
        application_key: impl Into<String>,
        device_name: impl Into<String>,
    ) -> Self {
        self.session = Some(SessionConfig {
            application_key: application_key.into(),
            device_name: device_name.into(),
            max_session_age: Some(Duration::from_secs(25 * 60)),
        });
        self
    }

    pub fn timeout(mut self, request_timeout: Duration) -> Self {
        self.timeouts.request_timeout = Some(request_timeout);
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.timeouts.connect_timeout = Some(connect_timeout);
        self
    }

    pub fn max_session_age(mut self, max_session_age: Option<Duration>) -> Self {
        if let Some(session) = self.session.as_mut() {
            session.max_session_age = max_session_age;
        }
        self
    }

    pub fn build(self) -> Result<ClientConfig, ApiError> {
        let mut config = ClientConfig::new(self.base_url)?;
        config.sanitize_nul_chars = self.sanitize_nul_chars;
        config.timeouts = self.timeouts;
        if let Some(session) = self.session {
            let application_key = session.application_key.trim().to_owned();
            let device_name = session.device_name.trim().to_owned();
            if application_key.is_empty() {
                return Err(ApiError::InvalidConfig {
                    message: "application_key cannot be empty".to_owned(),
                });
            }
            if device_name.is_empty() {
                return Err(ApiError::InvalidConfig {
                    message: "device_name cannot be empty".to_owned(),
                });
            }
            config.session = Some(SessionConfig {
                application_key,
                device_name,
                max_session_age: session.max_session_age,
            });
        }
        Ok(config)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestContext {
    pub method: HttpMethod,
    pub path: &'static str,
}

impl RequestContext {
    fn new(method: HttpMethod, path: &'static str) -> Self {
        Self { method, path }
    }
}

impl fmt::Display for RequestContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.method.as_str(), self.path)
    }
}

#[derive(Clone)]
pub struct QueryParam {
    pub name: &'static str,
    pub value: String,
}

impl QueryParam {
    pub fn new(name: &'static str, value: impl Into<String>) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }
}

pub fn push_query_param<T>(
    query: &mut Vec<QueryParam>,
    name: &'static str,
    value: &T,
) -> Result<(), ApiError>
where
    T: Serialize + ?Sized,
{
    if let Some(serialized) = serialize_query_value(value)? {
        query.push(QueryParam::new(name, serialized));
    }
    Ok(())
}

pub fn serialize_query_value<T>(value: &T) -> Result<Option<String>, ApiError>
where
    T: Serialize + ?Sized,
{
    let encoded = serde_json::to_value(value).map_err(|err| {
        ApiError::encode(
            "query parameter",
            format!("failed to serialize query parameter: {err}"),
        )
    })?;
    match encoded {
        Value::Null => Ok(None),
        Value::Bool(value) => Ok(Some(value.to_string())),
        Value::Number(value) => Ok(Some(value.to_string())),
        Value::String(value) => Ok(Some(value)),
        other => serde_json::to_string(&other).map(Some).map_err(|err| {
            ApiError::encode(
                "query parameter",
                format!("failed to encode structured query parameter: {err}"),
            )
        }),
    }
}

#[derive(Clone)]
pub struct RequestSpec {
    pub method: HttpMethod,
    pub path: &'static str,
    pub query: Vec<QueryParam>,
    pub body_json: Option<String>,
    pub authorization: Option<Authorization>,
}

impl RequestSpec {
    pub fn url(&self, base_url: &str) -> String {
        let mut url = format!("{base_url}{}", self.path);
        if self.query.is_empty() {
            return url;
        }
        let encoded = self
            .query
            .iter()
            .map(|param| {
                format!(
                    "{}={}",
                    encode_query_component(param.name),
                    encode_query_component(&param.value)
                )
            })
            .collect::<Vec<_>>()
            .join("&");
        url.push('?');
        url.push_str(&encoded);
        url
    }
}

/// HTTP response whose body is available explicitly, but omitted from `Debug`.
#[derive(Clone)]
pub struct RawResponse {
    pub status: u16,
    pub body_text: String,
    pub content_type: Option<String>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Authorization {
    Application(String),
    Session(String),
}

/// A typed result with explicit access to the original payload.
///
/// `Debug` omits payloads, including `model`. Formatting a model or raw body
/// explicitly is the caller's responsibility.
#[derive(Clone)]
pub struct ResponseEnvelope<T> {
    pub model: T,
    pub status: u16,
    pub raw_text: String,
    pub raw_json: Option<Value>,
}

impl<T> ResponseEnvelope<T> {
    pub fn map<U>(self, mapper: impl FnOnce(T) -> U) -> ResponseEnvelope<U> {
        ResponseEnvelope {
            model: mapper(self.model),
            status: self.status,
            raw_text: self.raw_text,
            raw_json: self.raw_json,
        }
    }

    pub fn into_model(self) -> T {
        self.model
    }
}

/// Request failure. `Display` and `Debug` omit payloads and detailed messages.
/// Inspect `body` and `message` explicitly when needed for trusted diagnostics.
#[derive(Error)]
pub enum ApiError {
    #[error("invalid config (details available in message)")]
    InvalidConfig { message: String },
    #[error("transport error for {context}")]
    Transport {
        context: RequestContext,
        message: String,
    },
    #[error("transport error (details available in message)")]
    TransportOther { message: String },
    #[error("http status {status} for {context}")]
    HttpStatus {
        context: RequestContext,
        status: u16,
        body: String,
    },
    #[error("encode error for {target}")]
    Encode {
        target: &'static str,
        message: String,
    },
    #[error("decode error for {context}")]
    Decode {
        context: RequestContext,
        message: String,
        body: String,
    },
    #[error("session error (details available in message)")]
    Session { message: String },
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfig")
            .field("base_url", &"[REDACTED]")
            .field("sanitize_nul_chars", &self.sanitize_nul_chars)
            .field("session", &self.session)
            .field("timeouts", &self.timeouts)
            .finish()
    }
}

impl fmt::Debug for ClientConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientConfigBuilder")
            .field("base_url", &"[REDACTED]")
            .field("sanitize_nul_chars", &self.sanitize_nul_chars)
            .field("session", &self.session)
            .field("timeouts", &self.timeouts)
            .finish()
    }
}

impl fmt::Debug for SessionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SessionConfig")
            .field("application_key", &"[REDACTED]")
            .field("device_name", &"[REDACTED]")
            .field("max_session_age", &self.max_session_age)
            .finish()
    }
}

impl fmt::Debug for Authorization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Application(_) => f.write_str("Application([REDACTED])"),
            Self::Session(_) => f.write_str("Session([REDACTED])"),
        }
    }
}

impl fmt::Debug for QueryParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QueryParam")
            .field("name", &self.name)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

impl fmt::Debug for RequestSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestSpec")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("query", &self.query)
            .field("has_body", &self.body_json.is_some())
            .field("authorization", &self.authorization)
            .finish()
    }
}

impl fmt::Debug for RawResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawResponse")
            .field("status", &self.status)
            .field("body_bytes", &self.body_text.len())
            .field("has_content_type", &self.content_type.is_some())
            .finish()
    }
}

impl<T> fmt::Debug for ResponseEnvelope<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseEnvelope")
            .field("status", &self.status)
            .field("body_bytes", &self.raw_text.len())
            .field("has_raw_json", &self.raw_json.is_some())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl ApiError {
    fn transport(context: RequestContext, message: impl Into<String>) -> Self {
        Self::Transport {
            context,
            message: message.into(),
        }
    }

    fn transport_other(message: impl Into<String>) -> Self {
        Self::TransportOther {
            message: message.into(),
        }
    }

    fn http_status(context: RequestContext, status: u16, body: impl Into<String>) -> Self {
        Self::HttpStatus {
            context,
            status,
            body: body.into(),
        }
    }

    fn encode(target: &'static str, message: impl Into<String>) -> Self {
        Self::Encode {
            target,
            message: message.into(),
        }
    }

    fn decode(
        context: RequestContext,
        message: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self::Decode {
            context,
            message: message.into(),
            body: body.into(),
        }
    }

    pub fn request_context(&self) -> Option<RequestContext> {
        match self {
            Self::Transport { context, .. }
            | Self::HttpStatus { context, .. }
            | Self::Decode { context, .. } => Some(*context),
            _ => None,
        }
    }

    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::HttpStatus { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn is_authentication_error(&self) -> bool {
        matches!(self, Self::HttpStatus { status: 401, .. })
    }
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn execute(&self, request: RequestSpec) -> Result<RawResponse, ApiError>;
}

#[derive(Debug, Clone)]
pub struct StaticResponseTransport {
    response: RawResponse,
}

impl StaticResponseTransport {
    pub fn json(status: u16, payload: Value) -> Self {
        Self {
            response: RawResponse {
                status,
                body_text: payload.to_string(),
                content_type: Some("application/json".to_owned()),
            },
        }
    }

    pub fn text(status: u16, body_text: impl Into<String>) -> Self {
        Self {
            response: RawResponse {
                status,
                body_text: body_text.into(),
                content_type: None,
            },
        }
    }
}

#[async_trait]
impl Transport for StaticResponseTransport {
    async fn execute(&self, _request: RequestSpec) -> Result<RawResponse, ApiError> {
        Ok(self.response.clone())
    }
}

#[derive(Clone)]
struct SessionState {
    token: Option<String>,
    opened_at: Option<Instant>,
    generation: u64,
}

struct SessionLease {
    token: String,
    generation: u64,
}

impl SessionState {
    fn is_fresh(&self, max_session_age: Option<Duration>) -> bool {
        match (&self.token, &self.opened_at) {
            (Some(_), Some(opened_at)) => {
                if let Some(max_session_age) = max_session_age {
                    opened_at.elapsed() < max_session_age
                } else {
                    true
                }
            }
            (Some(_), None) => true,
            _ => false,
        }
    }
}

struct SessionManager {
    config: SessionConfig,
    transport: Arc<dyn Transport>,
    state: Mutex<SessionState>,
}

impl SessionManager {
    fn new(config: SessionConfig, transport: Arc<dyn Transport>) -> Self {
        Self {
            config,
            transport,
            state: Mutex::new(SessionState {
                token: None,
                opened_at: None,
                generation: 0,
            }),
        }
    }

    async fn ensure_session(&self, base_url: &str) -> Result<String, ApiError> {
        Ok(self.acquire_session(base_url).await?.token)
    }

    async fn acquire_session(&self, base_url: &str) -> Result<SessionLease, ApiError> {
        let mut state = self.state.lock().await;
        self.ensure_session_locked(base_url, &mut state).await
    }

    async fn refresh_after_401(
        &self,
        base_url: &str,
        observed_generation: u64,
    ) -> Result<SessionLease, ApiError> {
        let mut state = self.state.lock().await;
        // A delayed rejection must not invalidate a session opened by another request.
        if state.generation == observed_generation {
            state.token = None;
            state.opened_at = None;
        }
        self.ensure_session_locked(base_url, &mut state).await
    }

    async fn ensure_session_locked(
        &self,
        base_url: &str,
        state: &mut SessionState,
    ) -> Result<SessionLease, ApiError> {
        if state.is_fresh(self.config.max_session_age) {
            if let Some(token) = state.token.clone() {
                return Ok(SessionLease {
                    token,
                    generation: state.generation,
                });
            }
        }

        if let Some(previous_token) = state.token.take() {
            let _ = self.close_session_request(base_url, &previous_token).await;
        }

        let generation = state
            .generation
            .checked_add(1)
            .ok_or_else(|| ApiError::Session {
                message: "session generation exhausted".to_owned(),
            })?;
        let token = self.open_session_request(base_url).await?;
        state.token = Some(token.clone());
        state.opened_at = Some(Instant::now());
        state.generation = generation;
        Ok(SessionLease { token, generation })
    }

    async fn invalidate(&self) {
        let mut state = self.state.lock().await;
        state.token = None;
        state.opened_at = None;
    }

    async fn close_session(&self, base_url: &str) -> Result<(), ApiError> {
        let mut state = self.state.lock().await;
        let Some(token) = state.token.take() else {
            state.opened_at = None;
            return Ok(());
        };
        state.opened_at = None;
        drop(state);
        self.close_session_request(base_url, &token).await
    }

    async fn open_session_request(&self, base_url: &str) -> Result<String, ApiError> {
        let context = RequestContext::new(HttpMethod::Get, "/api/Sessions/OpenNewSession");
        debug!(
            method = %context.method.as_str(),
            path = context.path,
            "opening WebAPI session"
        );
        let request = RequestSpec {
            method: HttpMethod::Get,
            path: "/api/Sessions/OpenNewSession",
            query: vec![QueryParam::new(
                "deviceName",
                self.config.device_name.clone(),
            )],
            body_json: None,
            authorization: Some(Authorization::Application(
                self.config.application_key.clone(),
            )),
        };
        let response = self.transport.execute(request).await?;
        if !(200..300).contains(&response.status) {
            warn!(
                method = %context.method.as_str(),
                path = context.path,
                status = response.status,
                "WebAPI session open failed with non-success status"
            );
            return Err(ApiError::http_status(
                context,
                response.status,
                response.body_text,
            ));
        }
        extract_session_token(&response.body_text, base_url)
    }

    async fn close_session_request(&self, _base_url: &str, token: &str) -> Result<(), ApiError> {
        let context = RequestContext::new(HttpMethod::Get, "/api/Sessions/CloseSession");
        debug!(
            method = %context.method.as_str(),
            path = context.path,
            "closing WebAPI session"
        );
        let request = RequestSpec {
            method: HttpMethod::Get,
            path: "/api/Sessions/CloseSession",
            query: vec![],
            body_json: None,
            authorization: Some(Authorization::Session(token.to_owned())),
        };
        let response = self.transport.execute(request).await?;
        if !(200..300).contains(&response.status) {
            warn!(
                method = %context.method.as_str(),
                path = context.path,
                status = response.status,
                "WebAPI session close failed with non-success status"
            );
            return Err(ApiError::http_status(
                context,
                response.status,
                response.body_text,
            ));
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
    base_url: String,
}

impl ReqwestTransport {
    pub fn new(config: &ClientConfig) -> Result<Self, ApiError> {
        let mut builder = reqwest::Client::builder();
        if let Some(request_timeout) = config.timeouts.request_timeout {
            builder = builder.timeout(request_timeout);
        }
        if let Some(connect_timeout) = config.timeouts.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }
        let client = builder.build().map_err(|err| {
            ApiError::transport_other(format!("failed to build reqwest client: {err}"))
        })?;
        Ok(Self {
            client,
            base_url: config.base_url.clone(),
        })
    }
}

#[async_trait]
impl Transport for ReqwestTransport {
    async fn execute(&self, request: RequestSpec) -> Result<RawResponse, ApiError> {
        let context = RequestContext::new(request.method, request.path);
        let url = request.url(&self.base_url);
        let method = match request.method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        };
        debug!(
            method = %context.method.as_str(),
            path = context.path,
            has_body = request.body_json.is_some(),
            query_len = request.query.len(),
            authenticated = request.authorization.is_some(),
            "sending WebAPI request"
        );
        let mut builder = self.client.request(method, url);
        if let Some(authorization) = request.authorization {
            builder = builder.header(
                "Authorization",
                match authorization {
                    Authorization::Application(value) => format!("Application {value}"),
                    Authorization::Session(value) => format!("Session {value}"),
                },
            );
        }
        if let Some(body_json) = request.body_json {
            builder = builder.header(CONTENT_TYPE, "application/json");
            builder = builder.body(body_json);
        }
        let response = builder
            .send()
            .await
            .map_err(|err| ApiError::transport(context, format!("http request failed: {err}")))?;
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let body_text = response.text().await.map_err(|err| {
            ApiError::transport(context, format!("failed to read response body: {err}"))
        })?;
        debug!(
            method = %context.method.as_str(),
            path = context.path,
            status,
            "received WebAPI response"
        );
        Ok(RawResponse {
            status,
            body_text,
            content_type,
        })
    }
}

pub struct ApiClient {
    config: ClientConfig,
    transport: Arc<dyn Transport>,
    session_manager: Option<Arc<SessionManager>>,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        let transport = ReqwestTransport::new(&config)?;
        Ok(Self::with_transport(config, transport))
    }

    pub fn with_transport(config: ClientConfig, transport: impl Transport + 'static) -> Self {
        Self::from_shared_transport(config, Arc::new(transport))
    }

    pub fn from_shared_transport(config: ClientConfig, transport: Arc<dyn Transport>) -> Self {
        let session_manager = config
            .session
            .clone()
            .map(|session| Arc::new(SessionManager::new(session, Arc::clone(&transport))));
        Self {
            config,
            transport,
            session_manager,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    pub async fn ensure_session(&self) -> Result<Option<String>, ApiError> {
        match &self.session_manager {
            Some(session_manager) => session_manager
                .ensure_session(self.base_url())
                .await
                .map(Some),
            None => Ok(None),
        }
    }

    pub async fn invalidate_session(&self) {
        if let Some(session_manager) = &self.session_manager {
            session_manager.invalidate().await;
        }
    }

    pub async fn close_session(&self) -> Result<(), ApiError> {
        match &self.session_manager {
            Some(session_manager) => session_manager.close_session(self.base_url()).await,
            None => Ok(()),
        }
    }

    pub async fn request_no_body<TResponse>(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
    ) -> Result<ResponseEnvelope<TResponse>, ApiError>
    where
        TResponse: DeserializeOwned,
    {
        self.request_typed(method, path, query, None::<&Value>)
            .await
    }

    pub async fn request_with_body<TResponse, TBody>(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
        body: &TBody,
    ) -> Result<ResponseEnvelope<TResponse>, ApiError>
    where
        TResponse: DeserializeOwned,
        TBody: Serialize + ?Sized,
    {
        self.request_typed(method, path, query, Some(body)).await
    }

    pub async fn request_no_body_raw(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        self.request_raw(method, path, query, None::<&Value>).await
    }

    pub async fn request_with_body_raw<TBody>(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
        body: &TBody,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        TBody: Serialize + ?Sized,
    {
        self.request_raw(method, path, query, Some(body)).await
    }

    async fn request_typed<TResponse, TBody>(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
        body: Option<&TBody>,
    ) -> Result<ResponseEnvelope<TResponse>, ApiError>
    where
        TResponse: DeserializeOwned,
        TBody: Serialize + ?Sized,
    {
        let context = RequestContext::new(method, path);
        let raw = self.request_raw(method, path, query, body).await?;
        let raw_json = raw.raw_json.clone().unwrap_or(Value::Null);
        let typed = serde_json::from_value::<TResponse>(raw_json.clone()).map_err(|err| {
            ApiError::decode(
                context,
                format!("failed to deserialize typed response: {err}"),
                raw.raw_text.clone(),
            )
        })?;
        Ok(ResponseEnvelope {
            model: typed,
            status: raw.status,
            raw_text: raw.raw_text,
            raw_json: Some(raw_json),
        })
    }

    async fn request_raw<TBody>(
        &self,
        method: HttpMethod,
        path: &'static str,
        query: Vec<QueryParam>,
        body: Option<&TBody>,
    ) -> Result<ResponseEnvelope<Value>, ApiError>
    where
        TBody: Serialize + ?Sized,
    {
        let context = RequestContext::new(method, path);
        let body_json = match body {
            Some(payload) => Some(serde_json::to_string(payload).map_err(|err| {
                ApiError::encode(
                    "request body",
                    format!("failed to serialize request body: {err}"),
                )
            })?),
            None => None,
        };

        let response = if let Some(session_manager) = &self.session_manager {
            let lease = session_manager.acquire_session(self.base_url()).await?;
            let first_request = RequestSpec {
                method,
                path,
                query: query.clone(),
                body_json: body_json.clone(),
                authorization: Some(Authorization::Session(lease.token)),
            };
            let first_response = self.transport.execute(first_request).await?;
            if first_response.status == 401 {
                warn!(
                    method = %context.method.as_str(),
                    path = context.path,
                    "received 401 from WebAPI, acquiring a fresh session and retrying once"
                );
                let retry_lease = session_manager
                    .refresh_after_401(self.base_url(), lease.generation)
                    .await?;
                let retry_request = RequestSpec {
                    method,
                    path,
                    query,
                    body_json,
                    authorization: Some(Authorization::Session(retry_lease.token)),
                };
                self.transport.execute(retry_request).await?
            } else {
                first_response
            }
        } else {
            let request = RequestSpec {
                method,
                path,
                query,
                body_json,
                authorization: None,
            };
            self.transport.execute(request).await?
        };
        if !(200..300).contains(&response.status) {
            warn!(
                method = %context.method.as_str(),
                path = context.path,
                status = response.status,
                "WebAPI request completed with non-success status"
            );
            return Err(ApiError::http_status(
                context,
                response.status,
                response.body_text,
            ));
        }

        let sanitized_body = sanitize_body(&response.body_text, self.config.sanitize_nul_chars);
        let json_value = parse_json_body(context, &sanitized_body)?;
        Ok(ResponseEnvelope {
            model: json_value.clone(),
            status: response.status,
            raw_text: sanitized_body,
            raw_json: Some(json_value),
        })
    }
}

fn sanitize_body(body: &str, sanitize_nul_chars: bool) -> String {
    if sanitize_nul_chars {
        body.replace('\0', "")
    } else {
        body.to_owned()
    }
}

fn parse_json_body(context: RequestContext, body: &str) -> Result<Value, ApiError> {
    if body.trim().is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str::<Value>(body).map_err(|err| {
        ApiError::decode(
            context,
            format!("failed to decode response json: {err}"),
            body.to_owned(),
        )
    })
}

fn extract_session_token(body: &str, _base_url: &str) -> Result<String, ApiError> {
    let trimmed = body.trim();
    let stripped = trimmed
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(trimmed)
        .trim();
    if stripped.is_empty() {
        return Err(ApiError::Session {
            message: "session open response did not contain a token".to_owned(),
        });
    }
    Ok(stripped.to_owned())
}

fn encode_query_component(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                char::from(byte).to_string()
            }
            b' ' => "%20".to_owned(),
            _ => format!("%{:02X}", byte),
        })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc as StdArc;
    use std::time::Duration;

    use super::{
        ApiClient, ApiError, Authorization, ClientConfig, ClientTimeouts, HttpMethod, QueryParam,
        RawResponse, RequestContext, RequestSpec, StaticResponseTransport, Transport,
    };
    use async_trait::async_trait;
    use serde_json::json;
    use tokio::sync::Mutex as TokioMutex;

    #[tokio::test]
    async fn request_no_body_raw_returns_json_payload() {
        let config = ClientConfig::new("https://example.test").expect("config");
        let client = ApiClient::with_transport(
            config,
            StaticResponseTransport::json(200, json!({"ok": true})),
        );

        let response = client
            .request_no_body_raw(HttpMethod::Get, "/api/ping", vec![])
            .await
            .expect("raw response");

        assert_eq!(response.status, 200);
        assert_eq!(response.model, json!({"ok": true}));
    }

    #[tokio::test]
    async fn request_no_body_typed_deserializes_model() {
        let config = ClientConfig::new("https://example.test").expect("config");
        let client = ApiClient::with_transport(
            config,
            StaticResponseTransport::json(200, json!({"value": 7})),
        );

        let response = client
            .request_no_body::<TestPayload>(HttpMethod::Get, "/api/test", vec![])
            .await
            .expect("typed response");

        assert_eq!(response.model.value, 7);
        assert_eq!(response.raw_json, Some(json!({"value": 7})));
    }

    #[tokio::test]
    async fn request_sanitizes_nul_chars_when_enabled() {
        let config = ClientConfig::new("https://example.test")
            .expect("config")
            .with_nul_sanitization(true);
        let client = ApiClient::with_transport(
            config,
            StaticResponseTransport::text(200, "{\u{0}\"value\":1}"),
        );

        let response = client
            .request_no_body_raw(HttpMethod::Get, "/api/test", vec![])
            .await
            .expect("sanitized response");

        assert_eq!(response.model, json!({"value": 1}));
        assert_eq!(response.raw_text, "{\"value\":1}");
    }

    #[tokio::test]
    async fn request_returns_http_status_error_for_non_success() {
        let config = ClientConfig::new("https://example.test").expect("config");
        let client =
            ApiClient::with_transport(config, StaticResponseTransport::text(403, "denied"));

        let error = client
            .request_no_body_raw(HttpMethod::Get, "/api/test", vec![])
            .await
            .expect_err("http status error");

        match error {
            ApiError::HttpStatus {
                context,
                status,
                body,
            } => {
                assert_eq!(context, RequestContext::new(HttpMethod::Get, "/api/test"));
                assert_eq!(status, 403);
                assert_eq!(body, "denied");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn request_spec_builds_encoded_url() {
        let spec = RequestSpec {
            method: HttpMethod::Get,
            path: "/api/test",
            query: vec![
                QueryParam::new("orderBy", "A B"),
                QueryParam::new("symbol", "x/y"),
            ],
            body_json: None,
            authorization: None,
        };

        assert_eq!(
            spec.url("https://example.test"),
            "https://example.test/api/test?orderBy=A%20B&symbol=x%2Fy"
        );
    }

    #[tokio::test]
    async fn session_open_and_authenticated_request_use_expected_headers() {
        let config = ClientConfig::new("https://example.test")
            .expect("config")
            .with_session("app-key", "erp-sync")
            .expect("session config");
        let transport = RecordingTransport::new(vec![
            RawResponse {
                status: 200,
                body_text: "\"token-1\"".to_owned(),
                content_type: None,
            },
            RawResponse {
                status: 200,
                body_text: "{\"ok\":true}".to_owned(),
                content_type: Some("application/json".to_owned()),
            },
        ]);
        let recorded_requests = transport.requests();
        let client = ApiClient::with_transport(config, transport);

        let response = client
            .request_no_body_raw(HttpMethod::Get, "/api/ping", vec![])
            .await
            .expect("authenticated response");

        assert_eq!(response.model, json!({"ok": true}));
        let requests = recorded_requests.lock().await.clone();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].path, "/api/Sessions/OpenNewSession");
        assert_eq!(
            requests[0].authorization,
            Some(Authorization::Application("app-key".to_owned()))
        );
        assert_eq!(requests[1].path, "/api/ping");
        assert_eq!(
            requests[1].authorization,
            Some(Authorization::Session("token-1".to_owned()))
        );
    }

    #[tokio::test]
    async fn session_request_retries_once_after_401() {
        let config = ClientConfig::new("https://example.test")
            .expect("config")
            .with_session("app-key", "erp-sync")
            .expect("session config");
        let transport = RecordingTransport::new(vec![
            RawResponse {
                status: 200,
                body_text: "\"token-1\"".to_owned(),
                content_type: None,
            },
            RawResponse {
                status: 401,
                body_text: "expired".to_owned(),
                content_type: None,
            },
            RawResponse {
                status: 200,
                body_text: "\"token-2\"".to_owned(),
                content_type: None,
            },
            RawResponse {
                status: 200,
                body_text: "{\"ok\":true}".to_owned(),
                content_type: Some("application/json".to_owned()),
            },
        ]);
        let recorded_requests = transport.requests();
        let client = ApiClient::with_transport(config, transport);

        let response = client
            .request_no_body_raw(HttpMethod::Get, "/api/ping", vec![])
            .await
            .expect("retried response");

        assert_eq!(response.model, json!({"ok": true}));
        let requests = recorded_requests.lock().await.clone();
        assert_eq!(requests.len(), 4);
        assert_eq!(
            requests[1].authorization,
            Some(Authorization::Session("token-1".to_owned()))
        );
        assert_eq!(
            requests[3].authorization,
            Some(Authorization::Session("token-2".to_owned()))
        );
    }

    #[tokio::test]
    async fn a_second_401_is_returned_without_another_retry() {
        let transport = RecordingTransport::new(vec![
            RawResponse {
                status: 200,
                body_text: "\"token-1\"".into(),
                content_type: None,
            },
            RawResponse {
                status: 401,
                body_text: "expired".into(),
                content_type: None,
            },
            RawResponse {
                status: 200,
                body_text: "\"token-2\"".into(),
                content_type: None,
            },
            RawResponse {
                status: 401,
                body_text: "still expired".into(),
                content_type: None,
            },
        ]);
        let requests = transport.requests();
        let config = ClientConfig::new("https://example.test")
            .unwrap()
            .with_session("key", "test")
            .unwrap();
        let client = ApiClient::with_transport(config, transport);
        let error = client
            .request_no_body_raw(HttpMethod::Get, "/api/Test", vec![])
            .await
            .unwrap_err();
        assert_eq!(error.status_code(), Some(401));
        assert_eq!(requests.lock().await.len(), 4);
    }

    #[tokio::test]
    async fn failed_refresh_releases_the_lock_and_allows_a_later_open() {
        tokio::time::timeout(Duration::from_secs(5), async {
            let transport = RecordingTransport::new(vec![
                RawResponse {
                    status: 200,
                    body_text: "\"token-1\"".into(),
                    content_type: None,
                },
                RawResponse {
                    status: 401,
                    body_text: "expired".into(),
                    content_type: None,
                },
                RawResponse {
                    status: 500,
                    body_text: "open failed".into(),
                    content_type: None,
                },
                RawResponse {
                    status: 200,
                    body_text: "\"token-2\"".into(),
                    content_type: None,
                },
            ]);
            let config = ClientConfig::new("https://example.test")
                .unwrap()
                .with_session("key", "test")
                .unwrap();
            let client = ApiClient::with_transport(config, transport);
            let error = client
                .request_no_body_raw(HttpMethod::Get, "/api/Test", vec![])
                .await
                .unwrap_err();
            assert_eq!(error.status_code(), Some(500));
            assert_eq!(
                client.ensure_session().await.unwrap().as_deref(),
                Some("token-2")
            );
        })
        .await
        .expect("session lock was not released");
    }

    #[test]
    fn client_config_builder_applies_session_and_timeouts() {
        let config = ClientConfig::builder("https://example.test/")
            .nul_sanitization(true)
            .session("app-key", "device-name")
            .timeout(Duration::from_secs(45))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("config");

        assert_eq!(config.base_url, "https://example.test");
        assert!(config.sanitize_nul_chars);
        assert_eq!(
            config.timeouts.request_timeout,
            Some(Duration::from_secs(45))
        );
        assert_eq!(
            config.timeouts.connect_timeout,
            Some(Duration::from_secs(5))
        );
        assert_eq!(
            config
                .session
                .as_ref()
                .map(|session| session.application_key.as_str()),
            Some("app-key")
        );
    }

    #[test]
    fn client_config_defaults_include_timeouts() {
        let config = ClientConfig::new("https://example.test").expect("config");

        assert_eq!(
            config.timeouts.request_timeout,
            ClientTimeouts::default().request_timeout
        );
        assert_eq!(
            config.timeouts.connect_timeout,
            ClientTimeouts::default().connect_timeout
        );
    }

    #[derive(Debug, serde::Deserialize)]
    struct TestPayload {
        value: i32,
    }

    #[derive(Clone)]
    struct RecordingTransport {
        responses: StdArc<TokioMutex<VecDeque<RawResponse>>>,
        requests: StdArc<TokioMutex<Vec<RequestSpec>>>,
    }

    impl RecordingTransport {
        fn new(responses: Vec<RawResponse>) -> Self {
            Self {
                responses: StdArc::new(TokioMutex::new(VecDeque::from(responses))),
                requests: StdArc::new(TokioMutex::new(Vec::new())),
            }
        }

        fn requests(&self) -> StdArc<TokioMutex<Vec<RequestSpec>>> {
            StdArc::clone(&self.requests)
        }
    }

    #[async_trait]
    impl Transport for RecordingTransport {
        async fn execute(&self, request: RequestSpec) -> Result<RawResponse, ApiError> {
            self.requests.lock().await.push(request);
            self.responses
                .lock()
                .await
                .pop_front()
                .ok_or_else(|| ApiError::transport_other("no scripted response available"))
        }
    }

    #[test]
    fn api_error_exposes_request_context_and_status_code() {
        let error = ApiError::http_status(
            RequestContext::new(HttpMethod::Delete, "/api/orders/42"),
            401,
            "expired",
        );

        assert_eq!(
            error.request_context(),
            Some(RequestContext::new(HttpMethod::Delete, "/api/orders/42"))
        );
        assert_eq!(error.status_code(), Some(401));
        assert!(error.is_authentication_error());
    }
}
