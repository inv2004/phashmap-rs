#![feature(specialization)]
#![feature(iterator_flatten)]

#![feature(test)]

extern crate test;

extern crate indexmap;

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::hash::Hash;

pub struct PHashMap<K,V,S = RandomState>{
    kv: Vec<Vec<(K,V)>>,
    hash_builder: S,
    stat: usize
}

impl<K: Hash + Eq + Clone,V : Clone,S: BuildHasher + Default> PHashMap<K,V,S> {
    pub fn new() -> PHashMap<K,V,S> {
        Self::with_capacity(16)
    }

    pub fn with_capacity(size: usize) -> PHashMap<K,V,S> {
        PHashMap{
            kv: vec![vec![]; size],
            hash_builder: S::default(),
            stat: 0
        }
    }

    fn get_i(&self, k: &K) -> usize {
        let mut hasher = self.hash_builder.build_hasher();
        k.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.kv.len()
    }

    fn push(&mut self, kv: (K, V)) {
        let i = self.get_i(&kv.0);

        self.kv[i].push(kv);
        self.stat += 1;
    }

    fn rehash(&mut self) {
        let len = self.kv.len();
        let len = if len == 0 { 1 } else { 2 * len };
        let mut h = Self::with_capacity(len);

        self.kv.iter().flatten()
            .for_each(|x| {
                h.push(x.clone());
            });
        
        *self = h;
    }

    pub fn insert(&mut self, k: K, v: V) {
        if self.stat >= 3 * self.kv.len() / 4 {
            self.rehash();
        }

        self.push((k, v));
    }

    pub fn get(&self, k: K) -> Option<&V> {
        let i = self.get_i(&k);
        self.kv[i].iter().find(|(x,_)| *x == k).map(|(_,y)| y)
    }

    pub fn update(&mut self, k: K, v: V) {
        let i = self.get_i(&k);
        self.kv[i].iter_mut().find(|(x,_)| *x == k).map(|(_,y)| {*y = v});
    }

    pub fn get_mut_def(&mut self, k: K, v: V) -> &mut V {
        let i = self.get_i(&k);
        if let Some(x) = self.kv[i].iter().position(|(x,_)| *x == k) {
            &mut self.kv[i][x].1
        } else {
            let len = self.kv[i].len();
            self.kv[i].push((k,v));
            &mut self.kv[i][len].1
        }
    }

    // pub fn values(&self) -> impl Iterator<Item=&V> {
    //     self.kv.iter().flatten()
    // }

    // pub fn entry(&mut self, k: K) -> Entry<V> {
    //     let i = self.get_i(&k);
    //     if let Some(x) = self.keys[i].iter().position(|ref x| **x == k) {
    //         Entry::Occupied(OccupiedEntry{val: &self.vals[i][x]})
    //     } else {
    //         Entry::Vacant(VacantEntry{val: &self.vals[i]})
    //     }
    // }

}

impl<K: Hash + Eq + Clone,V : Clone,S: BuildHasher + Default> Default for PHashMap<K,V,S> {
    fn default() -> Self {
        Self::new()
    }
}

// impl<K: Hash + Eq + Clone,V : Clone,S: BuildHasher + Default> IntoIterator for PHashMap<K,V,S> {
//     type Item = (K, V);
//     type IntoIter = std::iter::Zip<std::iter::Flatten<std::vec::IntoIter<std::vec::Vec<K>>>, std::iter::Flatten<std::vec::IntoIter<std::vec::Vec<V>>>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.kv.into_iter().flatten().zip(self.vals.into_iter().flatten())
//     }
// }

// pub enum Entry<'a, V: 'a> {
//     Occupied(OccupiedEntry<'a, V>),
//     Vacant(VacantEntry<'a, V>),
// }

// impl<'a, V: 'a> Entry<'a, V> {
//     pub fn or_insert(self, v: V) -> &'a mut V {
//         match self {
//             Entry::Occupied(entry) => entry.into_mut(),
//             Entry::Vacant(entry) => entry.insert(v)
//         }
//     }
// }

// pub struct OccupiedEntry<'a, V: 'a> {
//     val: &'a V
// }

// impl<'a, V: 'a> OccupiedEntry<'a, V> {
//     pub fn into_mut(self) -> &'a mut V {
//         &mut self.val
//     }
// }

// pub struct VacantEntry<'a, V: 'a> {
//     val: &'a Vec<V>
// }

// impl<'a, V: 'a> VacantEntry<'a, V> {
//     pub fn insert(self, v: V) -> &'a mut V {
//         self.val.push(v);
//         &mut self.val[0]
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;
    use std::collections::HashMap;
    use indexmap::IndexMap;

    #[bench]
    fn bench_std_hashmap_insert(b: &mut Bencher) {
        b.iter(|| {
            let mut h = HashMap::new();
            for x in 0..10000 {
                h.insert(x, 1);
            }
            h
        });
    }

    #[bench]
    fn bench_phashmap_insert(b: &mut Bencher) {
        b.iter(|| {
            let mut h: PHashMap<u16, u16> = PHashMap::with_capacity(20000);
            for x in 0..10000 {
                h.insert(x, 1);
            }
            h
        });
    }

    #[bench]
    fn bench_indexmap_insert(b: &mut Bencher) {
        b.iter(|| {
            let mut h = IndexMap::new();
            for x in 0..10000 {
                h.insert(x, 1);
            }
            h
        });
    }
}
