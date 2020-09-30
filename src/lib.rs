// Copyright (C) 2020 Joseph R. Quinn
// SPDX-License-Identifier: MIT

use std::ops::{ Add, Index, IndexMut };
use std::cmp::PartialOrd;

/// Generic implementation of the Floyd-Warshall algorithm
/// for finding the shortest paths in a weighted graph.
pub fn floyd_warshall<T, N, M>(graph: &mut M, size: usize)
where
    T: Add<Output=T> + PartialOrd + Copy,
    N: IndexMut<usize, Output=T> + Sized,
    M: IndexMut<usize, Output=N> + Sized,
    <N as Index<usize>>::Output: Sized
{
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                if graph[i][j] > graph[i][k] + graph[k][j] {
                    graph[i][j] = graph[i][k] + graph[k][j]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::floyd_warshall;

    #[test]
    fn test_floyd_warshall_vector() {
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
}
