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

impl<L, R> ConnectionMap<L, R>
where
    L: AsHashKey,
    R: AsHashKey,
    <L as AsHashKey>::Key: Eq + Clone,
    <R as AsHashKey>::Key: Eq + Clone,
{
    pub fn insert(&mut self, left: L, right: R) -> Option<(L, R)> {
        let left_key = left.as_key();
        let right_key = right.as_key();

        self.left_to_right
            .insert(left_key.clone(), right_key.clone());
        self.right_to_left.insert(right_key, left_key.clone());
        self.left_to_pair.insert(left_key, (left, right))
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
        self.left_to_pair.remove(left_key)
    }

    pub fn remove_by_right_key(&mut self, right_key: &<R as AsHashKey>::Key) -> Option<(L, R)> {
        let left_key = self.right_to_left.get(right_key)?;
        self.left_to_pair.remove(left_key)
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

impl<L: AsHashKey, R: AsHashKey> IntoIterator for ConnectionMap<L, R> {
    type Item = (L, R);

    type IntoIter = std::iter::Map<
        std::collections::hash_map::IntoIter<<L as AsHashKey>::Key, (L, R)>,
        fn((<L as AsHashKey>::Key, (L, R))) -> (L, R),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.left_to_pair
            .into_iter()
            .map(|(_, connection)| connection)
    }
}
