# Simple Messenger

// TODO preview

This is my small pet project aimed at testing my skills in the following areas:
1. Rust development
2. Backend development ([Axum](https://github.com/tokio-rs/axum))
3. Relation databases ([PostgreSQL](https://www.postgresql.org/) using [SeaORM](https://www.sea-ql.org/SeaORM/))
4. Frontend ([Leptos](https://leptos.dev/))

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

1. Rust Nightly with necessary tools:
    - `rustup toolchain install nightly` -> `rustup default nightly`
    - `rustup target add wasm32-unknown-unknown`
    - `cargo install cargo-leptos`

2. Node JS:
    - `npm install -D tailwindcss`

## Environment

Place your `.env` file in the current directory before launch.

.env
```env
# Optional. Defaults to "localhost:3000"
LEPTOS_SITE_ADDR = "localhost:3000"

# This uses Google OAuth2 for redirection to your website
# If you're using different values, don't forget to add them in
# the Authorized Redirect URIs within your Google Console app settings.
HOST_URL = "http://localhost:3000"

# Optional. Defaults to "localhost:6379"
REDIS_HOST = "localhost:6379"
REDIS_PASSWORD = ""

# Optional. Defaults to "localhost:5432"
POSTGRES_HOST = "localhost:5432" 
POSTGRES_PASSWORD = ""

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

The project can be deployed using Docker Compose.

```bash
docker compose up -d
```

