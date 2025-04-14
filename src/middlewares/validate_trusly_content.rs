use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct TruslyPayload {
    truslycontent: Option<u8>,
}

pub async fn validate_trusly_content(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    // Convertimos el cuerpo en bytes
    let bytes = to_bytes(body, 100000)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Deserializamos el JSON desde los bytes
    let parsed: Result<TruslyPayload, _> = serde_json::from_slice(&bytes);

    match parsed {
        Ok(TruslyPayload {
            truslycontent: Some(1),
        }) => {
            let body = Body::from(bytes); // Usamos `Body` aquí, ya que necesitamos el tipo `Body`

            let req = Request::from_parts(parts, body);
            Ok(next.run(req).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
