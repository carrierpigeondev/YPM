use std::fmt::format;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::post, Json, Router, Extension};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_yml;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use base64::{alphabet, Engine, engine, engine::general_purpose};
use sha2::digest::Update;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Deserialize)]
struct PackageQuery {
    name: String,
    target: String,
}

// have structs for the entire package even though it is not entirely read in case they will be used
// in future versions or forks of ypm
#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    license: String,
    author: Author,
    binaries: Vec<Binary>,
}

#[derive(Deserialize)]
struct Author {
    name: String,
    email: Option<String>,
    phone: Option<String>,
    links: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Binary {
    target: String,
    path: String,
    sha256: String,
}

async fn handle_package(Extension(packages_root): Extension<Arc<String>>, Query(params): Query<PackageQuery>) -> impl IntoResponse {
    // get the name and target from the parameters of the query
    let name = &params.name;
    let target = &params.target;

    // format the path based on a const path and the name
    let package_data_path = PathBuf::from(&*packages_root)
        .join(name)
        .join("package.yaml")
        .to_string_lossy()
        .to_string();

    // open up the package's yaml file, send 404 to client if not found
    let yaml_data = match std::fs::read_to_string(&package_data_path) {
        Ok(data) => data,
        Err(e) => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Package not found",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // deserialize the yaml file into a Package struct, send 500 to client if failed
    let package: Package = match serde_yml::from_str(&yaml_data) {
        Ok(pkg) => pkg,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "Failed to deserialize package YAML file",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // get the binary by iterating the binaries available in the package and checking if it matches
    let binary = package.binaries.iter().find(|&b| b.target == *target);

    // get the path to the binary and its sha256, send 404 to client if no binary for target matches
    let (binary_path, binary_sha256) = match binary {
        Some(bin) => (&bin.path, &bin.sha256),
        None => {
            return (StatusCode::NOT_FOUND, Json(json!({
                "error": "Binary for the provided target not found",
                "details": "None"
            }))).into_response();
        }
    };

    // open the binary file, send 500 to the client if the file cannot be opened
    let mut file = match File::open(binary_path) {
        Ok(f) => f,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "Failed to open binary",
                "details": e.to_string()
            }))).into_response();
        }
    };

    // get the sha256 checksum of the binary file
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => Update::update(&mut hasher, &buffer),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "Failed to open binary",
                "details": e.to_string()
            }))).into_response();
        }
    }

    // save the sha256 as a hexidecimal string
    let sha256 = format!("{:x}", hasher.finalize());

    // verify the sha256, send 500 to client if the checksum is not valid
    if *binary_sha256.to_ascii_lowercase() != sha256 {  // lowercase so it is case-insensitive
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "error": "SHA256 checksum of package is incorrect",
            "details": "None"
        }))).into_response();
    }

    let extension = Path::new(&binary_path).extension().map_or("".to_string(), |ext| format!(".{}", ext.to_string_lossy()));

    let e = base64::engine::general_purpose::STANDARD.encode(&buffer);

    println!("Sending the following content: {}", json!({
        "binary_content_LENGTH": base64::engine::general_purpose::STANDARD.encode(&buffer).len(),
        "file_name": format!("{}{}", name, extension),
        "sha256": binary_sha256
    }));

    // return the content and sha256
    (StatusCode::OK, Json(json!({
        "binary_content": base64::engine::general_purpose::STANDARD.encode(&buffer),
        "file_name": format!("{}{}", name, extension),
        "sha256": binary_sha256
    }))).into_response()
}

#[tokio::main]
async fn main() {
    // collect the args and get the package root
    let args: Vec<String> = std::env::args().collect();
    let packages_root = args.get(1)
        .expect("Specify the directory for packages");
    let shared_root = Arc::new(packages_root.clone());

    // create a router to define the routing
    let router = Router::new()
        .route("/package", post(handle_package))
        .layer(Extension(shared_root));

    // use port 41824 by default
    let listener = tokio::net::TcpListener::bind("localhost:41824").await.unwrap();

    // serve the service
    axum::serve(listener, router).await.unwrap();
}
