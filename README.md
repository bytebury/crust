# crust 🍕 
> Proudly built in the [🍕 state](https://portal.ct.gov/).

Build like bytebury. The official template that we use for our websites, skip all the bootstrapping. We are not suggesting that this is the right way to create an application, this is just our way and we're sharing with everybody.

## Some Background
This is the template that we use at bytebury. Our primary stack is [Axum](https://github.com/tokio-rs/axum), [Askama](https://github.com/askama-rs/askama), [HTMX](https://github.com/bigskysoftware/htmx), and [SQLite](https://sqlite.org/) through [SQLx](https://github.com/launchbadge/sqlx). We run all of our servers on [DigitalOcean](https://www.digitalocean.com/) on various server sizes, so you'll notice some deployment workflows for DigitalOcean (feel free to change that to your liking). We use [Stripe](https://stripe.com) as our payment partner and [Google](https://google.com) for our OAuth (extensible).

## Getting Started
> [!NOTE]
> Running `./dev` will run the application in watch mode for you.

1. You'll need Rust and Cargo installed
2. Clone the repository `git clone git@github.com:bytebury/crust.git`
3. Run the development server `cd ./cust && ./dev.sh` in your terminal

This will run all of your migrations as well as generate a `.env` file in your root directory. Open it up and change the environment variables to your liking. After that, you should be ready to start development.
