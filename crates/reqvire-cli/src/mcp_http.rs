//! Shared host admission and browser-origin policy for every MCP HTTP endpoint.
use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, Method, StatusCode, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::str::FromStr;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// An explicitly permitted HTTP endpoint host, optionally restricted to a port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowedHost {
    host: String,
    port: Option<u16>,
}

impl FromStr for AllowedHost {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let invalid = || {
            "expected a concrete hostname or IP authority (host[:port], IPv6 in brackets), without a scheme, path, credentials, or wildcard".to_string()
        };
        // Reuse the same strict authority and port validation as origins.
        let origin = format!("http://{value}")
            .parse::<AllowedOrigin>()
            .map_err(|_| invalid())?;
        if origin
            .host
            .trim_matches(['[', ']'])
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| address.is_unspecified())
        {
            return Err(invalid());
        }
        let authority: axum::http::uri::Authority = value.parse().map_err(|_| invalid())?;
        Ok(Self {
            host: origin.host,
            port: authority.port_u16(),
        })
    }
}

impl std::fmt::Display for AllowedHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.port {
            Some(port) => write!(formatter, "{}:{port}", self.host),
            None => formatter.write_str(&self.host),
        }
    }
}

/// Keep listener authorities and browser origins as independent permissions.
#[derive(Clone, Default)]
pub struct HttpAccess {
    pub origins: OriginPolicy,
    hosts: Vec<AllowedHost>,
}

impl HttpAccess {
    pub fn new(origins: &[AllowedOrigin], hosts: &[AllowedHost]) -> Self {
        Self {
            origins: OriginPolicy::new(origins),
            hosts: hosts.to_vec(),
        }
    }

    pub fn for_listener(&self, host: &str, port: u16) -> Result<Self, String> {
        let mut access = self.clone();
        let host = listener_hostname(host);
        if !host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| address.is_unspecified())
        {
            access.hosts.push(endpoint_authority(host, port).parse()?);
        }
        Ok(access)
    }

    pub fn rmcp_allowed_hosts(&self) -> Vec<String> {
        ["localhost", "127.0.0.1", "[::1]"]
            .into_iter()
            .map(str::to_owned)
            .chain(self.hosts.iter().map(ToString::to_string))
            .collect()
    }
}

pub fn listener_hostname(host: &str) -> &str {
    host.strip_prefix('[')
        .and_then(|host| host.strip_suffix(']'))
        .unwrap_or(host)
}

pub fn endpoint_authority(host: &str, port: u16) -> String {
    let host = listener_hostname(host);
    if host.contains(':') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

/// A validated browser origin. Ports are compared using their effective value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowedOrigin {
    scheme: String,
    host: String,
    port: u16,
}

impl FromStr for AllowedOrigin {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let invalid = || {
            "expected an HTTP(S) origin (scheme://host[:port]) without credentials, path, query, fragment, or wildcard".to_string()
        };
        let uri: Uri = value.parse().map_err(|_| invalid())?;
        let scheme = uri.scheme_str().ok_or_else(invalid)?.to_ascii_lowercase();
        let authority = uri.authority().ok_or_else(invalid)?;
        let (_, raw_authority) = value.split_once("://").ok_or_else(invalid)?;
        if !matches!(scheme.as_str(), "http" | "https")
            || raw_authority != authority.as_str()
            || raw_authority.contains(['@', '*', '%', '\\'])
            || value.chars().any(char::is_whitespace)
        {
            return Err(invalid());
        }
        let host = authority.host().to_ascii_lowercase();
        if host.starts_with('[') {
            host.trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<std::net::Ipv6Addr>()
                .map_err(|_| invalid())?;
        } else {
            let domain = host.strip_suffix('.').unwrap_or(&host);
            if domain.len() > 253
                || domain.split('.').any(|label| {
                    label.is_empty()
                        || label.len() > 63
                        || !label.as_bytes()[0].is_ascii_alphanumeric()
                        || !label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
                        || !label
                            .bytes()
                            .all(|c| c.is_ascii_alphanumeric() || c == b'-')
                })
            {
                return Err(invalid());
            }
        }
        // `Authority::port()` returns None for malformed ports as well as
        // omitted ports, so validate the authored suffix before defaulting.
        let port = match raw_authority
            .strip_prefix(authority.host())
            .ok_or_else(invalid)?
        {
            "" => {
                if scheme == "https" {
                    443
                } else {
                    80
                }
            }
            suffix => {
                let port = suffix.strip_prefix(':').ok_or_else(invalid)?;
                if !port.bytes().all(|c| c.is_ascii_digit()) {
                    return Err(invalid());
                }
                port.parse::<u16>().map_err(|_| invalid())?
            }
        };
        Ok(Self { scheme, host, port })
    }
}

impl AllowedOrigin {
    fn rmcp_value(&self) -> String {
        // RMCP treats an omitted port as any port. The admission middleware
        // below enforces exact effective-port matching for configured origins.
        if (self.scheme == "https" && self.port == 443)
            || (self.scheme == "http" && self.port == 80)
        {
            format!("{}://{}", self.scheme, self.host)
        } else {
            format!("{}://{}:{}", self.scheme, self.host, self.port)
        }
    }
}

#[derive(Clone, Default)]
pub struct OriginPolicy {
    additional: Vec<AllowedOrigin>,
}

impl OriginPolicy {
    pub fn new(additional: &[AllowedOrigin]) -> Self {
        Self {
            additional: additional.to_vec(),
        }
    }

    fn allows(&self, value: &HeaderValue) -> bool {
        value
            .to_str()
            .ok()
            .and_then(|value| value.parse::<AllowedOrigin>().ok())
            .is_some_and(|origin| {
                matches!(origin.host.as_str(), "localhost" | "127.0.0.1" | "[::1]")
                    || self.additional.contains(&origin)
            })
    }

    pub fn rmcp_allowed_origins(&self) -> Vec<String> {
        [
            "http://localhost",
            "https://localhost",
            "http://127.0.0.1",
            "https://127.0.0.1",
            "http://[::1]",
            "https://[::1]",
        ]
        .into_iter()
        .map(str::to_owned)
        .chain(self.additional.iter().map(AllowedOrigin::rmcp_value))
        .collect()
    }

    pub fn cors_layer(&self) -> CorsLayer {
        let policy = self.clone();
        CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(move |origin, _| {
                policy.allows(origin)
            }))
            .allow_methods([Method::POST])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::HeaderName::from_static("mcp-protocol-version"),
                header::HeaderName::from_static("mcp-session-id"),
                header::HeaderName::from_static("last-event-id"),
            ])
            .expose_headers([
                header::HeaderName::from_static("mcp-session-id"),
                header::HeaderName::from_static("mcp-protocol-version"),
            ])
    }
}

/// Reject before CORS can short-circuit preflight or RMCP can execute a tool.
pub async fn validate_http_headers(
    State(policy): State<OriginPolicy>,
    mut request: Request,
    next: Next,
) -> Response {
    let mut origins = request.headers().get_all(header::ORIGIN).iter();
    if let Some(origin) = origins.next() {
        if origins.next().is_some() || !policy.allows(origin) {
            return (
                StatusCode::FORBIDDEN,
                "Forbidden: Origin header is not allowed",
            )
                .into_response();
        }
    }
    // RMCP compares explicit configured ports literally. Supply the HTTP
    // default for a bare Host so a listener on port 80 accepts normal clients.
    // Proxy forwarding headers are deliberately not used to infer a scheme.
    if let Some(authority) = request
        .headers()
        .get(header::HOST)
        .and_then(|host| host.to_str().ok())
        .and_then(|host| host.parse::<axum::http::uri::Authority>().ok())
        .filter(|authority| authority.as_str() == authority.host())
    {
        let port = if request.uri().scheme_str() == Some("https") {
            443
        } else {
            80
        };
        if let Ok(host) = format!("{authority}:{port}").parse() {
            request.headers_mut().insert(header::HOST, host);
        }
    }
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{to_bytes, Body},
        http::{header, Request, StatusCode},
    };
    use reqvire::exclusions::ExclusionSetBuilder;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    fn app() -> axum::Router {
        let exclusions = ExclusionSetBuilder::new()
            .build()
            .expect("build test exclusions");
        let origins = [
            "https://APP.example:443",
            "http://192.0.2.10:3000",
            "http://plain.example:80",
        ]
        .map(|value| value.parse().expect("parse configured test origin"));
        crate::mcp::router(false, &exclusions, &HttpAccess::new(&origins, &[]))
    }

    fn request(method: &str, origin: Option<&str>) -> Request<Body> {
        let mut request = Request::builder()
            .method(method)
            .uri("/mcp")
            .header(header::HOST, "127.0.0.1:8081")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .header("mcp-protocol-version", "2025-11-25");
        if let Some(origin) = origin {
            request = request.header(header::ORIGIN, origin);
        }
        request
            .body(Body::from(
                json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
                    "protocolVersion": "2025-11-25", "capabilities": {},
                    "clientInfo": {"name": "cors-test", "version": "1"}
                }})
                .to_string(),
            ))
            .expect("construct or read valid HTTP test data")
    }

    #[test]
    fn mcp_cors_origin_configuration_validation() {
        for invalid in [
            "",
            "*",
            "null",
            "file:///tmp/client",
            "ftp://app.example",
            "app.example",
            "https://user@app.example",
            "https://user:pass@app.example",
            "https://app.example/",
            "https://app.example/path",
            "https://app.example?q=1",
            "https://app.example#fragment",
            "https://*.example",
            "https://app.example:bad",
            "https://app.example:65536",
            "https://app.example:",
            "https://app.example https://other.example",
            " https://app.example",
            "https://app.example,https://other.example",
            "https://app.example\\other",
        ] {
            assert!(
                invalid.parse::<AllowedOrigin>().is_err(),
                "accepted invalid origin: {invalid}"
            );
        }
        assert_eq!(
            "https://APP.example:443"
                .parse::<AllowedOrigin>()
                .expect("parse valid test origin"),
            "https://app.example"
                .parse()
                .expect("parse valid test value")
        );
        assert_eq!(
            "http://app.example:80"
                .parse::<AllowedOrigin>()
                .expect("parse valid test origin"),
            "http://app.example"
                .parse()
                .expect("parse valid test value")
        );
        assert!("http://[2001:db8::1]:3000".parse::<AllowedOrigin>().is_ok());
    }

    #[tokio::test]
    async fn mcp_cors_configured_origins_initialize_and_discover_tools() {
        for origin in [
            None,
            Some("https://app.example"),
            Some("https://app.example:443"),
            Some("http://192.0.2.10:3000"),
            Some("http://plain.example"),
            Some("http://plain.example:80"),
            Some("http://localhost:5173"),
            Some("https://127.0.0.1:8443"),
            Some("http://[::1]:9000"),
        ] {
            let response = app()
                .oneshot(request("POST", origin))
                .await
                .expect("complete HTTP test request");
            assert_eq!(response.status(), StatusCode::OK, "{origin:?}");
            if let Some(origin) = origin {
                assert_eq!(
                    response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
                    origin
                );
                assert!(response.headers()[header::VARY]
                    .to_str()
                    .expect("construct or read valid HTTP test data")
                    .contains("origin"));
                let exposed = response.headers()[header::ACCESS_CONTROL_EXPOSE_HEADERS]
                    .to_str()
                    .expect("construct or read valid HTTP test data");
                assert!(
                    exposed.contains("mcp-session-id") && exposed.contains("mcp-protocol-version")
                );
            } else {
                assert!(!response
                    .headers()
                    .contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
            }
            let body = to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("complete HTTP test request");
            let body: Value = serde_json::from_slice(&body).expect("decode MCP response");
            assert_eq!(body["result"]["protocolVersion"], "2025-11-25");
        }
        let mut req = request("POST", Some("https://app.example"));
        *req.body_mut() = Body::from(
            json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}).to_string(),
        );
        let response = app()
            .oneshot(req)
            .await
            .expect("complete HTTP test request");
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("complete HTTP test request");
        let body: Value = serde_json::from_slice(&body).expect("decode MCP response");
        let tools = body["result"]["tools"]
            .as_array()
            .expect("tools response contains an array");
        assert!(tools.iter().any(|tool| tool["name"] == "reqvire.search"));
        assert!(!tools
            .iter()
            .any(|tool| tool["name"] == "reqvire.add_element"));
    }

    #[tokio::test]
    async fn mcp_cors_rejects_unlisted_and_malformed_origins_on_every_method() {
        for origin in [
            "https://other.example",
            "http://app.example",
            "https://app.example:444",
            "https://sub.app.example",
            "http://192.0.2.10:3001",
            "null",
            "file:///tmp/client",
            "https://app.example/path",
            "http://localhost/path",
            "https://user@localhost",
            "http://localhost.evil.example",
            "https://app.example https://other.example",
        ] {
            for method in ["OPTIONS", "POST", "GET", "DELETE"] {
                let response = app()
                    .oneshot(request(method, Some(origin)))
                    .await
                    .expect("complete HTTP test request");
                assert_eq!(
                    response.status(),
                    StatusCode::FORBIDDEN,
                    "{method}: {origin}"
                );
                assert!(!response
                    .headers()
                    .contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
            }
        }
        for method in ["OPTIONS", "POST"] {
            let mut req = request(method, Some("https://app.example"));
            req.headers_mut()
                .append(header::ORIGIN, HeaderValue::from_static("http://localhost"));
            assert_eq!(
                app()
                    .oneshot(req)
                    .await
                    .expect("complete HTTP test request")
                    .status(),
                StatusCode::FORBIDDEN
            );
            let mut req = request(method, None);
            req.headers_mut().insert(
                header::ORIGIN,
                HeaderValue::from_bytes(&[0xff]).expect("construct non-ASCII test header"),
            );
            assert_eq!(
                app()
                    .oneshot(req)
                    .await
                    .expect("complete HTTP test request")
                    .status(),
                StatusCode::FORBIDDEN
            );
        }
    }

    #[tokio::test]
    async fn mcp_cors_preflight_and_protocol_errors_use_same_policy() {
        let mut req = request("OPTIONS", Some("https://app.example"));
        req.headers_mut().insert(
            header::ACCESS_CONTROL_REQUEST_METHOD,
            HeaderValue::from_static("POST"),
        );
        req.headers_mut().insert(
            header::ACCESS_CONTROL_REQUEST_HEADERS,
            HeaderValue::from_static(
                "content-type,authorization,mcp-protocol-version,mcp-session-id,last-event-id",
            ),
        );
        let response = app()
            .oneshot(req)
            .await
            .expect("complete HTTP test request");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
            "https://app.example"
        );
        assert_eq!(
            response.headers()[header::ACCESS_CONTROL_ALLOW_METHODS],
            "POST"
        );
        let allowed = response.headers()[header::ACCESS_CONTROL_ALLOW_HEADERS]
            .to_str()
            .expect("construct or read valid HTTP test data");
        for name in [
            "content-type",
            "authorization",
            "mcp-protocol-version",
            "mcp-session-id",
            "last-event-id",
        ] {
            assert!(
                allowed.split(',').any(|value| value.trim() == name),
                "{allowed}"
            );
        }
        for method in ["GET", "DELETE"] {
            let response = app()
                .oneshot(request(method, Some("https://app.example")))
                .await
                .expect("construct or read valid HTTP test data");
            assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
            assert_eq!(
                response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
                "https://app.example"
            );
        }
        let mut req = request("POST", Some("https://app.example"));
        req.headers_mut().insert(
            "mcp-protocol-version",
            HeaderValue::from_static("unsupported"),
        );
        let response = app()
            .oneshot(req)
            .await
            .expect("complete HTTP test request");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
            "https://app.example"
        );
    }

    #[tokio::test]
    async fn mcp_cors_embedded_policy_does_not_leak_to_explorer() {
        let exclusions = ExclusionSetBuilder::new()
            .build()
            .expect("build test exclusions");
        let explorer = axum::Router::new().fallback(|| async { "explorer" });
        let router = crate::mcp::mount_read_only(
            explorer,
            false,
            &exclusions,
            std::sync::Arc::new(tokio::sync::RwLock::new(())),
            &HttpAccess::new(
                &["https://app.example"
                    .parse()
                    .expect("parse valid test value")],
                &[],
            ),
        );
        let response = router
            .clone()
            .oneshot(request("POST", Some("https://app.example")))
            .await
            .expect("construct or read valid HTTP test data");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
            "https://app.example"
        );
        let mut req = request("GET", Some("https://other.example"));
        *req.uri_mut() = "/".parse().expect("parse valid test value");
        let response = router
            .oneshot(req)
            .await
            .expect("complete HTTP test request");
        assert_eq!(response.status(), StatusCode::OK);
        assert!(!response
            .headers()
            .contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN));
        assert_eq!(
            to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("complete HTTP test request"),
            "explorer"
        );
    }

    #[tokio::test]
    async fn mcp_cors_default_origins_and_preflight() {
        let exclusions = ExclusionSetBuilder::new()
            .build()
            .expect("build test exclusions");
        let router = crate::mcp::router(false, &exclusions, &HttpAccess::default());
        for origin in [
            None,
            Some("http://localhost:5173"),
            Some("https://127.0.0.1:8443"),
            Some("http://[::1]:9000"),
        ] {
            assert_eq!(
                router
                    .clone()
                    .oneshot(request("POST", origin))
                    .await
                    .expect("initialize using default policy")
                    .status(),
                StatusCode::OK
            );
        }
        let response = router
            .clone()
            .oneshot(request("POST", Some("https://app.example")))
            .await
            .expect("reject unconfigured origin");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let response = router
            .oneshot(
                Request::builder()
                    .method("OPTIONS")
                    .uri("/mcp")
                    .header(header::HOST, "127.0.0.1:8081")
                    .header(header::ORIGIN, "http://localhost:5173")
                    .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
                    .header(
                        header::ACCESS_CONTROL_REQUEST_HEADERS,
                        "content-type,mcp-protocol-version",
                    )
                    .body(Body::empty())
                    .expect("construct or read valid HTTP test data"),
            )
            .await
            .expect("construct or read valid HTTP test data");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
            "http://localhost:5173"
        );
    }

    fn hosted_app(bind: &str, port: u16, aliases: &[&str], embedded: bool) -> axum::Router {
        let hosts: Vec<_> = aliases
            .iter()
            .map(|host| host.parse().expect("valid test hostname"))
            .collect();
        let access = HttpAccess::new(&[], &hosts)
            .for_listener(bind, port)
            .expect("configure listener host");
        let exclusions = ExclusionSetBuilder::new()
            .build()
            .expect("build test exclusions");
        if embedded {
            crate::mcp::mount_read_only(
                axum::Router::new().fallback(|| async { "explorer" }),
                false,
                &exclusions,
                std::sync::Arc::new(tokio::sync::RwLock::new(())),
                &access,
            )
        } else {
            crate::mcp::router(false, &exclusions, &access)
        }
    }

    fn request_to(host: &str) -> Request<Body> {
        let mut req = request("POST", None);
        req.headers_mut()
            .insert(header::HOST, host.parse().expect("valid test authority"));
        req
    }

    #[tokio::test]
    async fn mcp_host_explicit_listener_is_accepted() {
        for embedded in [false, true] {
            for bind in ["192.0.2.50", "mcp.example", "2001:db8::1", "[2001:db8::1]"] {
                let authority = endpoint_authority(bind, 8081);
                let response = hosted_app(bind, 8081, &[], embedded)
                    .oneshot(request_to(&authority))
                    .await
                    .expect("process explicit listener request");
                assert_eq!(
                    response.status(),
                    StatusCode::OK,
                    "{authority}, embedded={embedded}"
                );
                let response = hosted_app(bind, 8081, &[], embedded)
                    .oneshot(request_to(&endpoint_authority(bind, 8082)))
                    .await
                    .expect("process wrong-port request");
                assert_eq!(response.status(), StatusCode::FORBIDDEN);
            }
        }
        let response = hosted_app("192.0.2.50", 80, &[], false)
            .oneshot(request_to("192.0.2.50"))
            .await
            .expect("initialize on implicit HTTP port");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn mcp_host_aliases_are_independent_from_origins() {
        for embedded in [false, true] {
            let aliases = [
                "mcp.example",
                "restricted.example:8443",
                "192.0.2.51:8081",
                "[2001:db8::2]:9000",
            ];
            for authority in [
                "mcp.example",
                "MCP.EXAMPLE:443",
                "mcp.example:8081",
                "restricted.example:8443",
                "192.0.2.51:8081",
                "[2001:db8::2]:9000",
                "localhost:8081",
            ] {
                let response = hosted_app("0.0.0.0", 8081, &aliases, embedded)
                    .oneshot(request_to(authority))
                    .await
                    .expect("process public host request");
                assert_eq!(
                    response.status(),
                    StatusCode::OK,
                    "{authority}, embedded={embedded}"
                );
            }
            for authority in [
                "evil.example",
                "sub.mcp.example",
                "restricted.example:8444",
                "192.0.2.51:8082",
                "[2001:db8::2]:9001",
            ] {
                let mut req = request_to(authority);
                req.headers_mut()
                    .insert("x-forwarded-host", HeaderValue::from_static("mcp.example"));
                req.headers_mut()
                    .insert("forwarded", HeaderValue::from_static("host=mcp.example"));
                let response = hosted_app("0.0.0.0", 8081, &aliases, embedded)
                    .oneshot(req)
                    .await
                    .expect("reject unlisted host");
                assert_eq!(response.status(), StatusCode::FORBIDDEN, "{authority}");
            }
            let mut req = request_to("mcp.example");
            req.headers_mut().insert(
                header::ORIGIN,
                HeaderValue::from_static("https://mcp.example"),
            );
            assert_eq!(
                hosted_app("0.0.0.0", 8081, &aliases, embedded)
                    .oneshot(req)
                    .await
                    .expect("check independent origin policy")
                    .status(),
                StatusCode::FORBIDDEN
            );
            let mut req = request_to("app.example");
            req.headers_mut().insert(
                header::ORIGIN,
                HeaderValue::from_static("https://app.example"),
            );
            assert_eq!(
                app()
                    .oneshot(req)
                    .await
                    .expect("check independent host policy")
                    .status(),
                StatusCode::FORBIDDEN
            );
        }
    }

    #[tokio::test]
    async fn mcp_host_default_port_uses_request_uri_not_forwarded_scheme() {
        let mut req = request_to("tls.example");
        *req.uri_mut() = "https://tls.example/mcp".parse().expect("valid HTTPS URI");
        let response = hosted_app("0.0.0.0", 8081, &["tls.example:443"], false)
            .oneshot(req)
            .await
            .expect("process HTTPS authority");
        assert_eq!(response.status(), StatusCode::OK);
        let mut req = request_to("tls.example");
        req.headers_mut()
            .insert("x-forwarded-proto", HeaderValue::from_static("https"));
        let response = hosted_app("0.0.0.0", 8081, &["tls.example:443"], false)
            .oneshot(req)
            .await
            .expect("reject forwarded scheme inference");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn mcp_host_wildcard_listener_keeps_host_validation() {
        for bind in ["0.0.0.0", "::", "[::]"] {
            let response = hosted_app(bind, 8081, &[], false)
                .oneshot(request_to("unlisted.example:8081"))
                .await
                .expect("check wildcard listener");
            assert_eq!(response.status(), StatusCode::FORBIDDEN);
        }
    }

    #[test]
    fn mcp_host_configuration_and_ipv6_display() {
        for value in [
            "",
            "*",
            "*.example",
            "0.0.0.0",
            "[::]",
            "[::]:8081",
            "https://mcp.example",
            "user@mcp.example",
            "mcp.example/path",
            "mcp.example?x=1",
            "mcp.example#x",
            "mcp.example:bad",
            "mcp.example:65536",
            "mcp.example:",
            "mcp.example other.example",
            "::1",
            "mcp..example",
            "-mcp.example",
            "mcp_.example",
        ] {
            assert!(
                value.parse::<AllowedHost>().is_err(),
                "accepted invalid host: {value}"
            );
        }
        assert_eq!(
            "MCP.EXAMPLE:8443"
                .parse::<AllowedHost>()
                .expect("parse hostname")
                .to_string(),
            "mcp.example:8443"
        );
        assert_eq!(listener_hostname("[::1]"), "::1");
        assert_eq!(listener_hostname("::1"), "::1");
        assert_eq!(endpoint_authority("::1", 8081), "[::1]:8081");
        assert_eq!(endpoint_authority("[::1]", 8081), "[::1]:8081");
    }
}
