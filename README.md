# Rust Web API Example
Welcome to the Rust Web API Example repository! Dive into the source code of a simple yet powerful CRUD application crafted in Rust, seamlessly integrated with PostgreSQL for robust data management.

## Running the Project Locally
1. Start the PostgreSQL server in a Docker container:

   ```shell
    docker-compose up -d db
   ```
2. Install the SQLX-CLI if not already installed and apply the "up" migration script to the PostgreSQL database:

   ```shell
   # Install sqlx-cli
   cargo install sqlx-cli
   
   # Run migrations
   sqlx migrate run
   ```
3. Install the necessary crates and launch the web server:

   ```shell
   cargo r -r
   ```
## Running the Project in Docker
This project optimizes the final Docker image size by utilizing Ubuntu Chiselled as the base image. Therefore, it's essential to build this base Docker image initially before running the application.

**Building the base image:**
 ```shell
 docker build -t chiselled-ubuntu-base:latest . --build-arg ARCH=<your_arch> # example ARCH=arm64
 ```
**Running the application:**
```shell
docker compose up -d
```
## Testing
To test the create endpoint, use the following `curl` command:

   ```shell
   curl -sS -H 'Content-Type: application/json' -X POST -d '{"book":"The Lord of the Rings", "quote":"Sometimes to find the light, We must first touch the darkness."}' http://localhost:3000/quotes | jq
   ```

   To test the get endpoint, use the following `curl` command:
   ```shell
   curl http://localhost:8080/quotes | jq
   ```