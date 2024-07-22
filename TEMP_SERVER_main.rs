// don't have access to Cargo or any syntax analyzer; this is just a rough draft

// imports omitted

#[derive(Deserialize)]
struct PackageQuery:
    name: String,
    target: String,
    
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

async fn handle_package(Query(params): Query<PackageQuery>) {
    // get the name and target (platform) from the POST request (params)
    let name = &params.name;
    let target = &params.target;
    
    // search for the package in /packages by the name
    let package_data_path = format!("{}/{}/{}.yaml", PACKAGES_ROOT, name, name);
    
    // in the package's yaml, get the path of the binary according to the target
    let yaml_data = std::fs::read_to_string(package_path)
        .expect("Failed to read file");
    let package: Package = serde_yaml::from_str(&yaml_data)
        .expect("Failed to deserialize YAML");

    let binary_path = String::new()
    let binary_sha256 = String::new()
    for binary: Binary in package.binaries {
        if binary.target == target {
            binary_path = binary.path;
            binary_sha256 = binaty.sha256;
    }
    // check if the variables changed, otherwise handle this as an error
    
    // get the sha256 hash of the binary and compare it to the sha256 according to the target
    let sha256 = ...;  // uhh, impl this later
    if binary_sha256 != sha256 {
        // handle this as an error, the checksums/hashes should match
    }
    
    // if it matches, send a response to the client with the binary and the sha256 hash
    // ensure proper error handling and return proper HTTP response codes
}

async fn main() {
    // create a router to define the routing
    let router = Router::new()
        .route("/package", post(handle_package));

    // use port 3000 by default
    // this may be changed
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    // serve the service
    axum::serve(listener, router).await.unwrap();
}
