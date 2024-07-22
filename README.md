# YPM
Your Package Manager

## What is YPM?
Your Package Manager is a lightweight package manager framework meant to be easily modifiable for private and public package managers.

All of the source is written in Rust for safety and performance :)

## Package
Each package in YPM, by default, is distributed as a binary file, however, a package uploaded to a server needs a an accompanying data file to ensure it's integrity and details. This is by default done with YAML:
