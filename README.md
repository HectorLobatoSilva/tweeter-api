# Tweeter API example

A simple project to save tweets, add likes to tweet and remove, maded with rust and actix web

### Routes

-   tweets - > GET, POST
-   tweets/{id} -> GET, DELETE
-   tweets/{id}/likes -> GET, POST, DELETE

## Pre-requisites

-   Postgresql
-   diesel_cli

## Usage

The application needs .env file, run next on terminal.

```sql
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
```

Replace username and password for youre user credentials, and "diesel_demo" for your own database

## Documentation

-   [Diesel](https://diesel.rs/guides/getting-started)
-   [Actix-web](https://actix.rs/docs/getting-started/)

## License

MIT
