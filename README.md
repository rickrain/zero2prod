[![Rust-CI](https://github.com/rickrain/zero2prod/actions/workflows/ci.yml/badge.svg)](https://github.com/rickrain/zero2prod/actions/workflows/ci.yml)

# Zero To Production In Rust

This repository contains my implementation of the solution presented in the book [Zero To Production In Rust](https://www.zero2prod.com/) by _Luca Palmieri_. Rather than forking [Luca's repository](https://github.com/LukeMathWalker/zero-to-production), I decided to start from scratch as I make my journey through his book. Although, I still find myself referencing his repository often.

## Run Locally

To run the solution locally, your local machine must meet the following requirements.

- Rust development environment
- Docker (for running an instance of PostgreSQL)
- psql CLI
- sqlx CLI
- (optional) pgAdmin 4 (SQL UI)

> Note: This has been tested on Ubuntu 22.04 LTS.

The solution includes the script `./scripts/init_db.sh` that will run an instance using Docker and also perform some configuration. Execute this script first as it is required to even build the solution.

> Note: If you already have PostgreSQL running on your machine, you may get an error when running `./scripts/init_db.sh` indicating that the port is already in use. If so, consider stopping your local instance using `systemctl stop postgresql`.

To run the tests, run `cargo test`.

To run the solution, run `cargo run` and then open a browser to the server address, which should be http://127.0.0.1:8000.

With a running instance of the solution, test out some of the routes using `curl`.

```bash
# Call the health check endpoint (HTTP GET)
curl -v http://127.0.0.1:8000/health_check

# Add a subscriber to the subcriptions table (HTTP POST)
curl -vd "name=john%20doe&email=john_doe%40gmail.com" -X POST http://127.0.0.1:8000/subscriptions
```



