# understanding_actix_cookies
A small repo to understand an issue with Actix Cookies in some development & production settings.

## The problem
For some yet not understood reason, the cookies are being set in the backend but not being transmitted to the frontend in some combinations of servers and clients. This repo is an attempt to understand the problem and find a solution. If we figure out anything, you will see it explained here.

The issue seems to be related to specifically using the Firefox browser and the `actix-web` server. The cookies are being set in the server, but they are not being transmitted to the client. This is not happening with other browsers. [This has likely something to do with Firefox cookie security settings](https://support.mozilla.org/en-US/kb/introducing-total-cookie-protection-standard-mode), but we are not sure what they expect us to do.

## Starting the backend
To start the backend, use `cargo-watch`:

```bash
cd backend
cargo watch -q -c -w src/ -x run
```

If you don't have it installed you can install it by running:

```bash
cargo install cargo-watch
```

This one will run on port `9090`.

## Starting the frontend
The frontend is based on `Yew` and `Trunk`. To start the frontend, use `trunk`:

```bash
cd frontend
trunk serve --port 4000 --proxy-backend http://localhost:9090/api/ --open
```

If you don't have it installed you can install it by running:

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

This one will run on port `4000`.

## Starting the fake Oauth server
The fake Oauth server is a simple `actix-web` server that simulates an Oauth server. To start it, use `cargo-watch`:

```bash
cd oauth
cargo watch -q -c -w src/ -x run
```

This one will run on port `9999`.