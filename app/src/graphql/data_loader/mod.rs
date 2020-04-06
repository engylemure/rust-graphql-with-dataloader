use dataloader::{
    cached::{Item, Loader as CachedLoader},
    Loader,
};
use std::collections::BTreeMap;

pub mod character;
pub mod movie;
pub mod movie_character;
pub mod user;

type _DataLoader<K, V, B> = Loader<K, V, (), B>;
type CachedDataLoader<K, V, B> = CachedLoader<K, V, (), B, Cache<K, V, B>>;
type Cache<K, V, F> = BTreeMap<K, Item<K, V, (), F>>;
