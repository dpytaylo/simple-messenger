use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::{self, Instant};

pub struct TtlMap<K, V> {
    inner: Arc<Mutex<HashMap<K, (Instant, V)>>>,
    ttl: Duration,
}

impl<K, V> Clone for TtlMap<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            ttl: self.ttl,
        }
    }
}

impl<K, V> TtlMap<K, V>
where
    K: Eq + Hash + Send + 'static,
    V: Clone + Send + 'static,
{
    pub fn new(ttl: Duration) -> Self {
        let inner: Arc<Mutex<HashMap<K, (Instant, V)>>> = Default::default();
        let this = Self { inner, ttl };

        let clear_interval = ttl.div_f64(10.0); // 10% of ttl
        let mut interval = time::interval(clear_interval);

        tokio::spawn({
            let this = this.clone();
            async move {
                loop {
                    interval.tick().await;
                    this.clear_expired();
                }
            }
        });

        this
    }

    pub fn insert(&self, key: K, value: V) {
        self.inner
            .lock()
            .unwrap()
            .insert(key, (Instant::now(), value));
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.lock().unwrap().get(key).map(|val| val.1.clone())
    }

    pub fn take(&self, key: &K) -> Option<V> {
        self.inner.lock().unwrap().remove(key).map(|val| val.1)
    }

    fn clear_expired(&self) {
        self.inner
            .lock()
            .unwrap()
            .retain(|_k, v| v.0.elapsed() <= self.ttl);
    }
}

#[cfg(test)]
mod tests {
    use tokio::time;

    use super::*;

    #[tokio::test]
    async fn test_ttl_map() {
        let map = TtlMap::new(Duration::ZERO);

        map.insert("key", "value");
        assert_eq!(map.get(&"key"), Some("value"));

        time::sleep(Duration::from_millis(10)).await;
        map.clear_expired();

        assert_eq!(map.get(&"key"), None);
    }
}
