use std::{sync::Mutex, thread::LocalKey};

use cached::{once_cell::sync::Lazy, Cached};

// Based on <https://github.com/jaemk/cached/blob/0ef35bcb26aa91d72adba012dfc58f985d2a2e70/src/macros.rs#L113-L143>.
// The macros in `cached` have some limitations. Most importantly, they cannot handle type
// parameters.
//
// Thread local storage is needed to pass specification tests. Without it several of the tests fail
// when run concurrently with others. We believe this happens because some of the tests are not
// created through real state transitions and thus violate the assumptions made in `BeaconStateKey`.
// The caches do not have to be cleared between tests, but that may be a consequence of how tests
// are divided between threads.
pub trait CachedExt<K, V: Clone> {
    fn get(&'static self, key: &K) -> Option<V>;

    fn set(&'static self, key: K, value: V);

    fn look_up(&'static self, key: K, fallback: impl FnOnce() -> V) -> V {
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = fallback();
        self.set(key, value.clone());
        value
    }

    fn try_look_up<E>(
        &'static self,
        key: K,
        fallback: impl FnOnce() -> Result<V, E>,
    ) -> Result<V, E> {
        if let Some(value) = self.get(&key) {
            return Ok(value);
        }
        let value = fallback()?;
        self.set(key, value.clone());
        Ok(value)
    }
}

impl<K, V: Clone, C: Cached<K, V>> CachedExt<K, V> for Lazy<Mutex<C>> {
    fn get(&self, key: &K) -> Option<V> {
        raw_get(self, key)
    }

    fn set(&self, key: K, value: V) {
        raw_set(self, key, value);
    }
}

impl<K, V: Clone, C: Cached<K, V>> CachedExt<K, V> for LocalKey<Mutex<C>> {
    fn get(&'static self, key: &K) -> Option<V> {
        self.with(|mutex| raw_get(mutex, key))
    }

    fn set(&'static self, key: K, value: V) {
        self.with(|mutex| raw_set(mutex, key, value));
    }
}

fn raw_get<K, V: Clone>(mutex: &Mutex<impl Cached<K, V>>, key: &K) -> Option<V> {
    mutex
        .lock()
        .expect("accessing the cache should not cause panics")
        .cache_get(key)
        .cloned()
}

fn raw_set<K, V>(mutex: &Mutex<impl Cached<K, V>>, key: K, value: V) {
    mutex
        .lock()
        .expect("accessing the cache should not cause panics")
        .cache_set(key, value);
}
