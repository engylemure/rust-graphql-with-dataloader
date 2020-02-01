## setup database

### create the postgres container

```
docker-compose up -d
```

### run all migrations

```
npm run migrate:run
```

### fill data into database

```
npm run seed:run
```

## run api

### start the api using nodemon for auto restart

```
npx nodemon index.js
```

### open GraphiQL tool to play with it

```
http://localhost:4000
```
