use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::FromIterator,
};

/// A disjoint set / union-find data structure suitable for incremental
/// tracking of connected component identified by their hash.
///
/// The total number of components does not need to be known in advance.
/// Connections between components and the components themselves can be added
/// as they are discovered.
///
/// Employs rank-based set joins and path compression resulting in the
/// asymptotically optimal time complexity associated with union-find
/// algorithms.
///
/// ## Examples
/// ```
/// use disjoint_hash_set::DisjointHashSet;
/// let mut djhs = DisjointHashSet::new();
/// djhs.link("hello", "hi");
/// djhs.link("hello", "👋");
/// assert!(djhs.is_linked("hi", "👋"));
///
/// // `DisjointHashSet` can be built from an iterator of edges
/// let djhs = vec![("a", "b"), ("a", "c"), ("d", "e"), ("f", "f")]
///     .into_iter()
///     .collect::<DisjointHashSet<_>>();
///
/// // Consume djhs to iterate over each disjoint set
/// let sets = djhs.sets(); // looks like [{"a", "b", "c"}, {"d", "e"}, {"f"}]
/// assert_eq!(sets.count(), 3);
/// ```
#[derive(Debug)]
pub struct DisjointHashSet<K> {
    ids: HashMap<K, PointerId>,
    data: Vec<ParentPointer>,
}

impl<K: Eq + Hash> DisjointHashSet<K> {
    /// Creates an empty `DisjointHashSet`.
    pub fn new() -> Self {
        Self { ids: HashMap::new(), data: Vec::new() }
    }

    /// Check if the value has already been inserted.
    ///
    /// # Example
    /// ```
    /// use disjoint_hash_set::DisjointHashSet;
    /// let mut djhs = DisjointHashSet::new();
    /// assert!(!djhs.contains(&"a"));
    /// djhs.insert(&"a");
    /// assert!(djhs.contains(&"a"));
    /// ```
    pub fn contains<T: Borrow<K>>(&self, val: T) -> bool {
        self.id(val.borrow()).is_some()
    }

    /// Insert the value as a new disjoint set with a single member. Returns
    /// true if the value was not already present.
    ///
    /// ```
    /// use disjoint_hash_set::DisjointHashSet;
    /// let mut djhs = DisjointHashSet::new();
    /// assert!(djhs.insert(&"a"));
    /// assert!(!djhs.insert(&"a"));
    /// ```
    pub fn insert(&mut self, val: K) -> bool {
        (!self.contains(&val)).then(|| self.insert_unchecked(val)).is_some()
    }

    /// Checks if the two keys are members of the same set.
    /// This will not implicitly add values that were not already present.
    /// ```
    /// use disjoint_hash_set::DisjointHashSet;
    /// let mut djhs = DisjointHashSet::new();
    ///
    /// djhs.link("a", "b");
    /// djhs.link("a", "c");
    /// assert!(djhs.is_linked("b", "c"));
    /// assert!(!djhs.is_linked("a", "d"));
    /// ```
    pub fn is_linked<T: Borrow<K>>(&mut self, val1: T, val2: T) -> bool {
        let (id1, id2) = (
            self.id(val1.borrow()).map(|id| self.find(id)),
            self.id(val2.borrow()).map(|id| self.find(id)),
        );

        id1.is_some() && id2.is_some() && id1 == id2
    }

    /// Link the respective sets of the two provided values. This will insert
    /// non-existent values in the process.
    /// ```
    /// use disjoint_hash_set::DisjointHashSet;
    /// let mut djhs = DisjointHashSet::new();
    ///
    /// djhs.link("a", "b");
    /// assert!(djhs.contains("a"));
    /// assert!(djhs.contains("b"));
    /// assert!(djhs.is_linked("a", "b"));
    /// ```
    pub fn link(&mut self, val1: K, val2: K) {
        let ids = (self.id_or_insert(val1), self.id_or_insert(val2));
        let roots = (self.find(ids.0), self.find(ids.1));

        if roots.0 != roots.1 {
            let ranks = (self.get(roots.0).rank, self.get(roots.1).rank);

            if ranks.0 < ranks.1 {
                self.get_mut(roots.0).parent = roots.1;
            } else {
                self.get_mut(roots.1).parent = roots.0;

                if ranks.0 == ranks.1 {
                    self.get_mut(roots.0).rank += 1;
                };
            }
        }
    }

    /// Consumes the DisjointHashSet and returns an iterator of HashSets for
    /// each disjoint set.
    ///
    /// ```
    /// use disjoint_hash_set::DisjointHashSet;
    /// use std::{collections::HashSet, iter::FromIterator};
    ///
    /// let edges = vec![("a", "a"), ("b", "c"), ("d", "e"), ("e", "f")];
    /// let mut sets = DisjointHashSet::from_iter(edges).sets().collect::<Vec<_>>();
    /// sets.sort_by(|set_a, set_b| set_a.len().cmp(&set_b.len()));
    ///
    /// let expected_sets: Vec<HashSet<&str>> = vec![
    ///     HashSet::from_iter(vec!["a"]),
    ///     HashSet::from_iter(vec!["b", "c"]),
    ///     HashSet::from_iter(vec!["d", "e", "f"]),
    /// ];
    ///
    /// assert_eq!(sets, expected_sets);
    /// ```
    pub fn sets(mut self) -> impl Iterator<Item = HashSet<K>> {
        let mut sets = HashMap::new();

        let roots: Vec<PointerId> =
            (0..self.data.len()).map(|id| self.find(PointerId(id))).collect();

        self.ids.into_iter().for_each(|(val, id)| {
            sets.entry(roots[id.0]).or_insert_with(|| HashSet::new()).insert(val);
        });

        sets.into_iter().map(|(_, set)| set)
    }

    fn find(&mut self, id: PointerId) -> PointerId {
        let parent_id = self.get(id).parent;
        if id == parent_id {
            parent_id
        } else {
            let root_id = self.find(parent_id);
            self.get_mut(id).parent = root_id;
            root_id
        }
    }

    fn id(&self, value: &K) -> Option<PointerId> {
        self.ids.get(value).copied()
    }

    fn id_or_insert(&mut self, value: K) -> PointerId {
        self.id(&value).unwrap_or_else(|| self.insert_unchecked(value))
    }

    fn insert_unchecked(&mut self, value: K) -> PointerId {
        let id = PointerId(self.data.len());
        self.ids.insert(value, id);
        self.data.push(ParentPointer { parent: id, rank: 0 });
        id
    }

    fn get(&self, id: PointerId) -> &ParentPointer {
        &self.data[id.0]
    }

    fn get_mut(&mut self, id: PointerId) -> &mut ParentPointer {
        &mut self.data[id.0]
    }
}

#[derive(Debug)]
struct ParentPointer {
    parent: PointerId,
    rank: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PointerId(usize);

impl<V: Eq + Hash> FromIterator<(V, V)> for DisjointHashSet<V> {
    fn from_iter<I: IntoIterator<Item = (V, V)>>(links: I) -> Self {
        let mut djhs = DisjointHashSet::new();
        links.into_iter().for_each(|(a, b)| djhs.link(a, b));
        djhs
    }
}
