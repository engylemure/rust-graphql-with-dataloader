const { pick } = require('ramda')
const knex = require('../knex/client')

module.exports.getUsers = ids => {
  const query = knex('users').select('users.*')
  if (ids) {
    query.whereIn('id', ids)
    return query.then(rows => {
      const users = rowsTObjectById(rows)
      return ids.map(id => users[id])
    })
  }
  return query.map(rowIntoUser)
}

module.exports.getMovies = ids => {
  const query = knex('movies').select('movies.*')
  if (ids) {
    query.whereIn('id', ids)
    return query.then(rows => {
      const movies = rowsTObjectById(rows)
      return ids.map(id => movies[id])
    })
  }
  return query.map(rowIntoMovie)
}


module.exports.getMovieIdsByCharactersIds = ids => {
  const query = knex('movie_characters').select('movie_characters.*')
  if (ids) {
    query.whereIn('character_id', ids)
    return query.then(rows => {
      const movieIdsByCharacterId = rows.reduce((acc, val) => {
        if (acc[val['character_id']]) {
          acc[val['character_id']].push(val['movie_id'])
        } else {
          acc[val['character_id']] = [val['movie_id']]
        }
        return acc
      }, {})
      return ids.map(id => movieIdsByCharacterId[id])
    })
  }
  return query
}

module.exports.getCharacterIdsByMoviesIds = ids => {
  const query = knex('movie_characters').select('movie_characters.*')
  if (ids) {
    query.whereIn('movie_id', ids)
    return query.then(rows => {
      const characterIdsByMovieIds = rows.reduce((acc, val) => {
        if (acc[val['movie_id']]) {
          acc[val['movie_id']].push(val['character_id'])
        } else {
          acc[val['movie_id']] = [val['character_id']]
        }
        return acc
      }, {})
      return ids.map(id => characterIdsByMovieIds[id])
    })
  }
  return query
}


module.exports.getCharacters = ids => {
  const query = knex('characters').select('characters.*')
  if (ids) {
    query.whereIn('id', ids)
    return query.then(rows => {
      const characters = rowsTObjectById(rows)
      return ids.map(id => characters[id])
    })
  }
  return query.map(rowIntoCharacter)
}


const rowIntoUser = ({ created_at, updated_at, ...row }) => ({
  ...row,
  createdAt: created_at.toISOString(),
  updatedAt: updated_at.toISOString()
})

const rowIntoMovie = ({ created_at, updated_at, released_at, ...row }) => ({
  ...row,
  createdAt: created_at.toISOString(),
  updatedAt: updated_at.toISOString(),
  releasedAt: released_at.toISOString()
})

const rowIntoCharacter = ({ created_at, updated_at, ...row }) => ({
  ...row,
  createdAt: created_at.toISOString(),
  updatedAt: updated_at.toISOString()
})

const rowsTObjectById = rows => rows.reduce((acc, row) => {
  acc[row.id] = row
  return acc
}, {})