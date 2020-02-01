use dataloader::{Loader, cached::{ Loader as CachedLoader, Item}};
use std::collections::BTreeMap;

pub mod user;
pub mod character;
pub mod movie;
pub mod movie_character;

type _DataLoader<K, V, B> = Loader<K, V, (), B>;
type CachedDataLoader<K, V, B> = CachedLoader<K, V, (), B, Cache<K, V, B>>;
type Cache<K, V, F> = BTreeMap<K, Item<K, V, (), F>>;