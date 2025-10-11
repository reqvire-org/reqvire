use std::path::Path;
use reqvire::error::ReqvireError;

/// Starts an HTTP server serving static files from the given directory
pub fn serve_directory(
    directory: &Path,
    host: &str,
    port: u16,
) -> Result<(), ReqvireError> {
    use tiny_http::{Server, Response, Header};

    let addr = format!("{}:{}", host, port);
    let server = Server::http(&addr)
        .map_err(|e| ReqvireError::ProcessError(format!("Failed to start server: {}", e)))?;

    let url = format!("http://{}:{}", host, port);
    println!("🌐 Server running at: \x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, url);
    println!();
    println!("📖 Instructions:");
    println!("  • Open the link above in your browser");
    println!("  • Press Ctrl-C to stop server");
    println!();

    // Serve requests
    for request in server.incoming_requests() {
        let url_path = request.url().trim_start_matches('/');
        let file_path = if url_path.is_empty() {
            directory.join("index.html")
        } else {
            directory.join(url_path)
        };

        if file_path.exists() && file_path.is_file() {
            if let Ok(content) = std::fs::read(&file_path) {
                let content_type = match file_path.extension().and_then(|s| s.to_str()) {
                    Some("html") => "text/html; charset=utf-8",
                    Some("svg") => "image/svg+xml",
                    Some("css") => "text/css",
                    Some("js") => "application/javascript",
                    _ => "text/plain",
                };

                let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                    .unwrap();

                let response = Response::from_data(content).with_header(header);
                let _ = request.respond(response);
            }
        } else {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }

    Ok(())
}
