# YPM
Your Package Manager

## What is YPM?
Your Package Manager is a lightweight package manager framework meant to be easily modifiable for private and public package managers.

All of the source is written in Rust for safety and performance :)

## Package
Each package in YPM, by default, is distributed as a binary file, however, a package uploaded to a server needs a an accompanying data file to ensure it's integrity and details. This is by default done with YAML:

```yaml
package:
  name: "carrier-pigeon"
  version: "1.6.4"
  description: "File sharing client utilizing the Dropbox SDK and a custom Metadata server solution (Iron Pigeon), meant for trusted students to share study and school-related materials with others."
  license: "All Rights Reserved"
  author:
    name: "Carrier Pigeon Dev"
    email: "carrierpigeon.dev@gmail.com"        # Optional
    phone: "123-456-7890"                       # Optional
    links:                                      # Optional
      - "https://www.example.com"
  binaries:
    target: "Linux"                             # Only one target is needed; this is specified by the POST request to the server
      path: "~/Projects/Carrier Pigeon/output/carrier-pigeon_1.6.4+linux"
      sha256: 5a6eba74179c077508b615566bd7875524b5b836d5d22d78a70563bc4ca9aa07
    target: "Windows"                           
      path: "~/Projects/Carrier Pigeon/output/carrier-pigeon_1.6.4+windows.exe"
      sha256: c465946d97ea4c3fcbf5f74b576353a8e2cdb67e142201dd7547cd8037b912c3
```

The path for the binaries must be valid paths for the operating system the YPM host is running on.\
(Only supports downloading packages, not uploading. Use `curl` or `Invoke-WebRequest` to get make the POST request if using default repository code.)

## Setting Up
Fork the repository. All server code is located in the `/ypm` Cargo project directory. To run, ensure you specify an argument for the `packages_root`. For example:
```
cargo run /packages
```
Ensure it is a global path. By default, the server runs on `localhost:41824`.

## Client
A client PowerShell script is available in `/client` as `client.ps1`. Change the paramaters of the script and run it to download a package to the cwd.