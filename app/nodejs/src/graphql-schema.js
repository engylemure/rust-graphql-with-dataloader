const {makeExecutableSchema} = require('graphql-tools')
const {getUsers, getCharacters, getMovies} = require('./data')

// helps editor highlight support
const gql = input => input[0]

const typeDefs = gql`
# A Character Movie
type Character {
  id: Int!
  name: String!
  createdAt: DateTimeUtc!
  updatedAt: DateTimeUtc!
  deleted: Boolean!
  movies: [Movie!]
}

# DateTime
scalar DateTimeUtc

input LoginInput {
  email: String!
  password: String!
}

# A Movie
type Movie {
  id: Int!
  name: String!
  releasedAt: DateTimeUtc!
  createdAt: DateTimeUtc!
  updatedAt: DateTimeUtc!
  deleted: Boolean!
  characters: [Character!]
}

type Mutation {
  register(input: RegisterInput!): Token!
  login(input: LoginInput!): Token!
}

type Query {
  users: [User!]!
  movies: [Movie!]!
  characters: [Character!]!
  # Get the authenticated User
  me: User!
}

input RegisterInput {
  # Email for the User that is being registered
  email: String!
  # Name for the user. length constraints : (min = 1, max = 255)
  name: String!
  # Password for the authentication. length constraints : (min = 6)
  password: String!
}

# The token object with user information
type Token {
  bearer: String
  user: User!
}

# A user
type User {
  id: Int!
  uuid: Uuid
  email: String!
  createdAt: DateTimeUtc!
  updatedAt: DateTimeUtc!
  deleted: Boolean!
}

# Uuid
scalar Uuid

`

const resolvers = {
    Query: {
        users: async (parent, args, context, info) => {
          return getUsers()
        },
        movies: async (parent, args, context, info) => {
            return getMovies()
        },
        characters: async (parent, args, context, info) => {
            return getCharacters()
        }
    },
    Movie: {
        characters: async (parent, args, context, info) => {
            const characterIds = await context.loaders.characterIdsByMovieId.load(parent.id)
            return context.loaders.characterById.loadMany(characterIds ? characterIds : [])
        }
    },
    Character: {
        movies: async (parent, args, context, info) => {
            const movieIds = await context.loaders.movieIdsByCharacterId.load(parent.id)
            return context.loaders.movieById.loadMany(movieIds ? movieIds : [])
        }
    }
}

module.exports = makeExecutableSchema({
    typeDefs,
    resolvers,
})
