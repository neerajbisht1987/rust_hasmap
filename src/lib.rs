use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
struct Bucket<K,V>
{
    val:Vec<(K,V)>,
}

struct HashMap<K,V>
where K:Hash
//where K:std::hash::Hasher + std::cmp::PartialOrd
{
    buckets: Vec<Vec<(K,V)>>,
}

impl <K,V> HashMap<K,V>
where K:Hash
{
    pub fn new() ->Self
    {
        HashMap {
        buckets :vec![],
        }
        //list :Vec<Bucket<K,V>>!(Vec::new<K,V>());
    }
}
impl <K,V> HashMap<K,V>
where K: Hash + Eq
{
    pub fn insert(& mut self, key:K,value : V ) -> Option<V>
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = hasher.finish() as usize % self.buckets.len();
        let bucket = &mut self.buckets[bucket];
        // if let Some(&mut (ref ekey, ref mut evalue)) = bucket.iter_mut().find(|&mut (ref ekey, _)| ekey == key)        
        // {
        //     use std::mem;
        //     return Some(mem::replace(evalue,value));
        // }
        // else
        // {
        //     bucket.push((key,value));
        // }
        for &mut (ref ekey,ref mut evalue) in bucket
        {
            if ekey == &key
            {
                use std::mem;
                return Some(mem::replace(evalue,value));

            }

        }
        None
    }


    fn resize(&mut self)
    {
        match self.buckets.len() {
            0 => 1,
            n => n*2
        };
    }

}


#[cfg(test)]
mod test 
{
    use super::*;
    #[test]
    fn insert()
    {
        let mut  map = HashMap::new();
        map.insert("foo",42);
        map.insert("bar",41);
        map.insert("few",43);
    }
}