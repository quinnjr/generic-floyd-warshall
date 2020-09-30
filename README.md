# Generic Floyd-Warshall Algorithm

`generic-floyd-warshall`is intended to work with the core array-like primatives and types in the Rust language, like Vectors, so that the array-like types can be used to form matrices and then properly handle the execution of the Floyd-Warshall algorithm on them.

As well, `generic-floyd-warshall` is intended to be used with user-defined types that implement `std::ops::{ Add, Index, IndexMut }` and `std::cmp::PartialOrd`.

```rust
fn main() {
    let graph = vec![
        vec![0.0, f32::INFINITY, -2.0, f32::INFINITY],
        vec![4.0, 0.0, 3.0, f32::INFINITY],
        vec![f32::INFINITY, f32::INFINITY, 0.0, 2.0],
        vec![f32::INFINITY, -1.0, f32::INFINITY, 0.0]
    ];

    let expected = vec![
        vec![0.0, -1.0, -2.0, 0.0],
        vec![4.0, 0.0, 2.0, 4.0],
        vec![5.0, 1.0, 0.0, 2.0],
        vec![3.0, -1.0, 1.0, 0.0]
    ];

    let len = graph.len();
    let mut calculated = graph.clone();

    floyd_warshall(&mut calculated, len);

    assert_eq!(expected, calculated);
}
```

This crate is in an experimental phase. Critique and suggestions for improvement are welcome.
