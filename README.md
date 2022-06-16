# disjoint-hash-set

A Rust implementation of a disjoint set / union-find data structure for incremental tracking of connected components identified by their hash.

Incorporates rank-based set joins and path compression to ensure the asymptotically optimal time complexity associated with union-find algorithms.

**See the [reference docs](https://docs.rs/disjoint-hash-set) for examples, typical usage, and detailed documentation.**

```rust
use disjoint_hash_set::DisjointHashSet;

let mut djhs = DisjointHashSet::new();
djhs.link("hello", "hi");
djhs.link("hello", "👋");
assert!(djhs.is_linked("hi", "👋"));

// `DisjointHashSet` can be built from an iterator of edges
let djhs = vec![("a", "b"), ("a", "c"), ("d", "e"), ("f", "f")]
    .into_iter()
    .collect::<DisjointHashSet<_>>();

// Consume djhs to iterate over each disjoint set
let sets = djhs.sets(); // looks like [{"a", "b", "c"}, {"d", "e"}, {"f"}]
assert_eq!(sets.count(), 3);
```

Issues, requests, contributions, and general feedback are welcome.