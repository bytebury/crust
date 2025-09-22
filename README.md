# crust 🍕 
> Proudly built in the [🍕 state](https://portal.ct.gov/).

Build like bytebury. The official template that we use for our websites, skip all the bootstrapping. We are not suggesting that this is the right way to create an application, this is just _our way_ and we're sharing with everybody. We favor simplicity in developer experience over processes and tools. Hence, running locally is a single command: `./dev.sh` and deploying is done automatically through GHA. By keeping local development simple, as well as our architecture, we can move fast &mdash; relying on Rust's speed to match on the server. It's how we built [Fun Banking](https://fun-banking.com), and continue to build more.

## Some Background
This is the template that we use at bytebury. Our primary stack is [Axum](https://github.com/tokio-rs/axum), [Askama](https://github.com/askama-rs/askama), [HTMX](https://github.com/bigskysoftware/htmx), and [SQLite](https://sqlite.org/) through [SQLx](https://github.com/launchbadge/sqlx). We run all of our servers on [DigitalOcean](https://www.digitalocean.com/) on various server sizes, so you'll notice some deployment workflows for DigitalOcean (feel free to change that to your liking). We use [Stripe](https://stripe.com) as our payment partner and [Google](https://google.com) for our OAuth (extensible).

## Getting Started
> [!NOTE]
> Running `./dev` will run the application in watch mode for you.

1. You'll need Rust and Cargo installed
2. Clone the repository `git clone git@github.com:bytebury/crust.git`
3. Run the development server `cd ./crust && ./dev.sh` in your terminal

This will run all of your migrations as well as generate a `.env` file in your root directory. Open it up and change the environment variables to your liking. After that, you should be ready to start development.

## Creating a Migration
> [!NOTE]
> You will need sqlx installed locally to create migrations.

```sh
sqlx migrate add create_my_table
```

The next time the server runs, it will pick up the migration and automatically run it. Therefore, you should try to avoid running the application until you are ready to run the migration.
