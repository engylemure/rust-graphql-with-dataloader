const DataLoader = require('dataloader')
const { getMovies, getCharacters, getUsers, getMovieIdsByCharactersIds, getCharacterIdsByMoviesIds } = require('./data')

module.exports.buildLoaders = () => {
    const movieById = new DataLoader(ids => getMovies(ids))
    const characterById = new DataLoader(ids => getCharacters(ids))
    const movieIdsByCharacterId = new DataLoader(ids => getMovieIdsByCharactersIds(ids))
    const characterIdsByMovieId = new DataLoader(ids => getCharacterIdsByMoviesIds(ids))
    return {
        movieById,
        characterById,
        movieIdsByCharacterId,
        characterIdsByMovieId
    }
}
