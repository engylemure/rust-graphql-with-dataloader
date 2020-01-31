# rust-graphql-with-dataloader

A Rust GraphQL API with [actix-web](https://github.com/actix/actix-web), [juniper](https://github.com/graphql-rust/juniper), [diesel](https://github.com/diesel-rs/diesel) and [dataloader](https://github.com/cksac/dataloader-rs)

## Development

Consider that for running the project in development mode you should use these commands for starting the database and the Container for buildingand starting the application.
### Initial build for the Images.
```
docker-compose build
```
### Initialize the Database
```
docker-compose up -d db
```
### Initialize the API container attaching the bash command
```
docker-compose run api bash
```
### Run The Database Migrations 
```
diesel migration run
```
### At the bash inside the container
```
cargo build
```
### For running in the attached mode
```
cargo run
```

## Production 

For running in the production mode just execute the command:
```
docker-compose up -d 
```