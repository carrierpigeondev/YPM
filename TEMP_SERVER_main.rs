// don't have access to Cargo or any syntax analyzer; this is just a rough draft

use axum::{
    Router,
    routing::post,
}

async fn handle_package() {

}

async fn main() {
    let service = Router::new()
        .route("/package", post(handle_package));
}
