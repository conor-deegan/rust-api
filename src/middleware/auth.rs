use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use log::{error, info};

pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    info!("hello from auth middleware");

    // Check for the presence of the "Authorization" header
    match req.headers().get(header::AUTHORIZATION) {
        Some(auth_header_value) => {
            // Here you might want to further validate the token
            info!("authorization token found: {:?}", auth_header_value);
        }
        None => {
            // Log the error and return an Unauthorized status code
            error!("no auth token");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // If the auth header is present and valid, continue to the next middleware or handler
    let response = next.run(req).await;
    Ok(response)
}
