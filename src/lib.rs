use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
struct Bucket<K, V> {
    val: Vec<(K, V)>,
}

struct HashMap<K, V>
where
    K: Hash, //where K:std::hash::Hasher + std::cmp::PartialOrd
{
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        HashMap {
            buckets: vec![],
            items: 0,
        }
        //list :Vec<Bucket<K,V>>!(Vec::new<K,V>());
    }
}
impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || (self.items > 3 * self.buckets.len() / 4) {
            self.resize();
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = hasher.finish() as usize % self.buckets.len();
        let bucket = &mut self.buckets[bucket];
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                use std::mem;
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
       None
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => 1,
            n => n * 2,
        };

        //within `(K, V)`, the trait `std::clone::Clone` is not implemented for `V`
        //let mut new_buckets = vec![Vec::new(); target_size];
        let mut new_buckets= Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_|Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {     
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let bucket = hasher.finish() as usize % new_buckets.len();
                let bucket = &mut new_buckets[bucket];
                bucket.push((key, value));
            
        }
        std::mem::replace(& mut self.buckets, new_buckets);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.insert("bar", 41);
        map.insert("few", 43);
    }
}
