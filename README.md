# disjoint-hash-set

A disjoint set / union-find data structure suitable for incremental
tracking of connected component identified by their hash.

The total number of components does not need to be known in advance.
Connections between components and the components themselves can be added
as they are discovered.

Employs rank-based set joins and path compression resulting in the
asymptotically optimal time complexity associated with union-find
algorithms.

## Examples

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

## Documentation

For more examples and documentation, see the [reference docs](https://docs.rs/disjoint-hash-set).
