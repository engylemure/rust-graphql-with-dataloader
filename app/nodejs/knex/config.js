
const common = {
  client: 'mysql2',
  connection: {
    host: 'db',
    user: 'root',
    password: 'root',
    database: 'graphql',
  },
  pool: {
    min: 2,
    max: 10,
  },
}

module.exports = {
  development: common,

  // staging: common,

  // production: common,
}
