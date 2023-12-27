# Simple Messenger

// TODO preview

This is my small pet project aimed at testing my skills in the following areas:
1. Rust development
2. Backend development ([Axum](https://github.com/tokio-rs/axum))
3. Relation databases ([PostgreSQL](https://www.postgresql.org/) using [SeaORM](https://www.sea-ql.org/SeaORM/))
4. NonSQL databases ([Redis](https://github.com/redis-rs/redis-rs))
5. Frontend ([Leptos](https://leptos.dev/))

## Description

A simple messenger fullstack application fully written in Rust.

#### User Account:
1. Account Creation and Login:
- [ ] Users can create accounts and log in using email or Google OAuth2 authentication.
- [ ] User authentication through email and Google OAuth2.

2. Profile Visibility:
- [ ] All users can view the profiles of other users within the application.

3. Profile Management:
- [ ] Users have the ability to edit their profiles, updating personal information.
- [ ] Users can delete their accounts.

#### Channels:
1. Channel Creation:
- [ ] Users can open or create new channels within the application.

2. Messaging:
- [ ] Users can send messages to channels.
- [ ] Users receive notifications for messages in channels they are part of.

## Dependencies

1. Rust Nightly (1.77.0-nightly) with necessary tools:
    - `rustup toolchain install nightly` -> `rustup default nightly`
    - `rustup target add wasm32-unknown-unknown`
    - `cargo install cargo-leptos`
    - `cargo install cargo-generate` ([should be installed automatically in future](https://github.com/leptos-rs/start-axum))
2. Node JS:
    - `npm install -D tailwindcss`

## Environment
Place your `.env` file in the current directory before launch.

> [!NOTE]
> If a variable has a `*_FILE` version, either can be used for password 
> storage. If both are used, the version with `*_FILE` will take priority.

.env
```env
# Optional. Defaults to "127.0.0.1:3000"
LEPTOS_SITE_ADDR = "127.0.0.1:3000"

# Optional. Defaults to "localhost:6379"
REDIS_HOST = "localhost:6379"

REDIS_PASSWORD = "password"
REDIS_PASSWORD_FILE = "./config/secrets/redis_password.txt"

# Optional. Defaults to "localhost:5432"
POSTGRES_HOST = "localhost:5432" 

POSTGRES_PASSWORD = "password"
POSTGRES_PASSWORD_FILE = "./config/secrets/postgres_password.txt"

# This uses Google OAuth2 for redirection to your website
# If you're using different values, don't forget to add them in
# the Authorized Redirect URIs within your Google Console app settings.
REDIRECT_URL = "http://localhost:3000"

# Google API OAuth2
# https://support.google.com/googleapi/answer/6158849
#
# And we need only the "userinfo.email" scope:
# https://developers.google.com/identity/protocols/oauth2/scopes#oauth2
#
# A good visual example of the creation steps in Google Console:
# https://clerk.com/blog/oauth2-react-user-authorization
GOOGLE_CLIENT_ID = "your_code.apps.googleusercontent.com"
GOOGLE_CLIENT_ID_FILE = "./config/secrets/google_client_id_file.txt"
GOOGLE_CLIENT_SECRET = "client_secret"
GOOGLE_CLIENT_SECRET_FILE = "./config/secrets/google_client_secret.txt"
```

Leptos has its own environment variables that you can modify. 
You can find more information [here](https://github.com/leptos-rs/cargo-leptos?tab=readme-ov-file#environment-variables).

## Execution
Prepare your `.env` file and place it in the project root directory. Afterward, run this command:
```bash
cargo leptos watch
```

## Deployment

To deploy using Docker Compose, you should load your secret tokens. Create the `./config/secrets` directory
, and create a separate file for each token (for example, `./config/secrets/postgres_password.txt`).
Place your sensitive data there (e.g., `password123`). The specific secrets you need to include can be found
at the end of the `compose.yml` file.

Also, create the `redis_password.acl` file (`./config/secrets/redis_password.acl`) with following content:
```text
user default on >password ~* &* +@all
```
Replace `password` with the value from your `redis_password.txt` file.

Afterward, simply run the following command:
```bash
docker compose up -d
```

## Deployment without Docker Compose
1. Prepare your `.env` file and place it in the project's root directory
2. Run `cargo leptos build --release`
3. The minimum files needed for running are:
    - The server binary located in `target/server/release`
    - The site directory and all files within located in `target/site`

    Copy these files to your remote server. The directory structure should be:

    ```text
    simple-messenger
    site/
    ```
