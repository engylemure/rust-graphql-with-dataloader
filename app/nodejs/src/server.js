const express = require('express')
const graphql = require('express-graphql')
const schema = require('./graphql-schema')
const { buildLoaders } = require('./loaders')

const app = express()

app.use(
  graphql(request => ({
    schema,
    graphiql: true,
    context: { loaders: buildLoaders() },
  })),
)

app.listen(4000)
