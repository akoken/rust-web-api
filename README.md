# Rust Web API Example
Welcome to the Rust Web API Example repository! Dive into the source code of a simple yet powerful CRUD application crafted in Rust, seamlessly integrated with PostgreSQL for robust data management.

## Running the Project Locally
1. Start the PostgreSQL server in a Docker container:

   ```bash
   docker-compose up -d
   ```
2. Install the SQLX-CLI if not already installed and apply the "up" migration script to the PostgreSQL database:

   ```bash
   # Install sqlx-cli
   cargo install sqlx-cli
   
   # Run migrations
   sqlx migrate run
   ```
3. Install the necessary crates and launch the web server:

   ```bash
   cargo r -r
   ```

## Testing the Create Endpoint
To test the create endpoint, use the following `curl` command:

   ```bash
   curl -sS -H 'Content-Type: application/json' -X POST -d '{"book":"A Game of Thrones", "quote":"Fear cuts deeper than swords"}' http://localhost:3000/quotes | jq
   ```