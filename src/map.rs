use std::fmt::Debug;

use rustc_hash::FxHashMap as HashMap;

pub trait AsHashKey {
    type Key: std::hash::Hash;

    fn as_key(&self) -> Self::Key;
}

#[derive(Default)]
pub struct ConnectionMap<L, R>
where
    L: AsHashKey,
    R: AsHashKey,
{
    left_to_right: HashMap<L::Key, R::Key>,
    right_to_left: HashMap<R::Key, L::Key>,
    left_to_pair: HashMap<L::Key, (L, R)>,
}

/*
 * LKey -> (L, R)
 * RKey -> (L, R)
 * RKey -> LKey
 * LKey -> RKey
 * =>
 * RKey -> LKey -> (L, R)
 */

impl<L, R> ConnectionMap<L, R>
where
    L: AsHashKey,
    R: AsHashKey,
{
    pub fn new() -> Self {
        Self {
            left_to_right: HashMap::default(),
            right_to_left: HashMap::default(),
            left_to_pair: HashMap::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InsertAlreadyExistsError {
    Left,
    Right,
}
impl<L, R> ConnectionMap<L, R>
where
    L: AsHashKey,
    R: AsHashKey,
{
    fn do_assertions(&self) {
        debug_assert_eq!(self.left_to_pair.len(), self.left_to_right.len());
        debug_assert_eq!(self.left_to_pair.len(), self.right_to_left.len());
    }

    pub fn len(&self) -> usize {
        self.do_assertions();
        self.left_to_pair.len()
    }
}

impl<L, R> ConnectionMap<L, R>
where
    L: AsHashKey,
    R: AsHashKey,
    <L as AsHashKey>::Key: Eq + Clone,
    <R as AsHashKey>::Key: Eq + Clone,
{
    pub fn insert(&mut self, left: L, right: R) -> Result<(), InsertAlreadyExistsError> {
        let left_key = left.as_key();
        let right_key = right.as_key();

        if let Some(old) = self
            .left_to_right
            .insert(left_key.clone(), right_key.clone())
        {
            return Err(InsertAlreadyExistsError::Left);
        }

        if let Some(old) = self.right_to_left.insert(right_key, left_key.clone()) {
            return Err(InsertAlreadyExistsError::Right);
        }

        if let Some(old) = self.left_to_pair.insert(left_key, (left, right)) {
            panic!("Map invariant broken!");
        }

        self.do_assertions();

        Ok(())
    }

    pub fn get_by_left_key(&self, left_key: &<L as AsHashKey>::Key) -> Option<&(L, R)> {
        self.left_to_pair.get(left_key)
    }

    pub fn get_by_right_key(&self, right_key: &<R as AsHashKey>::Key) -> Option<&(L, R)> {
        let left_key = self.right_to_left.get(right_key)?;
        self.get_by_left_key(left_key)
    }

    pub fn get_by_left(&self, left: &L) -> Option<&(L, R)> {
        let left_key = left.as_key();
        self.get_by_left_key(&left_key)
    }

    pub fn get_by_right(&self, right: &R) -> Option<&(L, R)> {
        let right_key = right.as_key();
        self.get_by_right_key(&right_key)
    }

    pub fn remove_by_left_key(&mut self, left_key: &<L as AsHashKey>::Key) -> Option<(L, R)> {
        self.do_assertions();

        let right_key = self.left_to_right.remove(left_key)?;
        self.right_to_left
            .remove(&right_key)
            .expect("right_key to exist");
        let pair = self
            .left_to_pair
            .remove(left_key)
            .expect("left_key to exist");

        self.do_assertions();

        Some(pair)
    }

    pub fn remove_by_right_key(&mut self, right_key: &<R as AsHashKey>::Key) -> Option<(L, R)> {
        self.do_assertions();

        let left_key = self.right_to_left.remove(right_key)?;
        self.left_to_right
            .remove(&left_key)
            .expect("left_key to exist");
        let pair = self.left_to_pair.remove(&left_key).expect("left to exist");

        self.do_assertions();

        Some(pair)
    }

    pub fn remove_by_left(&mut self, left: &L) -> Option<(L, R)> {
        let left_key = left.as_key();
        self.remove_by_left_key(&left_key)
    }

    pub fn remove_by_right(&mut self, right: &R) -> Option<(L, R)> {
        let right_key = right.as_key();
        self.remove_by_right_key(&right_key)
    }
}

impl<L, R> ConnectionMap<L, R>
where
    L: AsHashKey + Debug,
    R: AsHashKey + Debug,
    <L as AsHashKey>::Key: Debug,
    <R as AsHashKey>::Key: Debug,
{
    pub fn dump(&self) {
        println!("====DUMP====");

        if self.len() == 0 {
            println!("(empty)");
            println!("============");
        } else {
            println!("total: {}", self.len());
        }

        println!("left_to_right:");
        for (left_key, right_key) in &self.left_to_right {
            println!("\t{:?} -> {:?}", left_key, right_key);
        }

        println!("right_to_left:");
        for (right_key, left_key) in &self.right_to_left {
            println!("\t{:?} -> {:?}", right_key, left_key);
        }

        println!("left_to_pair:");
        for (left_key, (left, right)) in &self.left_to_pair {
            println!("\t{:?} -> ({:?}<>{:?})", left_key, left, right);
        }

        println!("============");
    }
}

impl<L: AsHashKey, R: AsHashKey> ConnectionMap<L, R> {
    pub fn iter(&self) -> impl Iterator<Item = &(L, R)> {
        self.left_to_pair.values()
    }
}

impl<L: AsHashKey + Debug, R: AsHashKey + Debug> Debug for ConnectionMap<L, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (left, right) in self.iter() {
            write!(f, "{:?}<>{:?}", left, right)?;
        }
        Ok(())
    }
}

impl<L: AsHashKey, R: AsHashKey> IntoIterator for ConnectionMap<L, R> {
    type Item = (L, R);

    type IntoIter = std::collections::hash_map::IntoValues<<L as AsHashKey>::Key, (L, R)>;

    fn into_iter(self) -> Self::IntoIter {
        self.left_to_pair.into_values()
    }
}
