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
    target: "Linux"                             # Optional (If Windows has a target)
      path: "~/Projects/Carrier Pigeon/output/carrier-pigeon_1.6.4+linux"
      sha256: PLACEHOLDER
    target: "Windows"                           # Optional (If Linux has a target)
      path: "~/Projects/Carrier Pigeon/output/carrier-pigeon_1.6.4+windows.exe"
      sha256: PLACEHOLDER

  

```
