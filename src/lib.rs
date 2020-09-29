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
    fn test_floyd_warshall() {
        let graph1 = vec![
            vec![0.0, f32::INFINITY, -2.0, f32::INFINITY],
            vec![4.0, 0.0, 3.0, f32::INFINITY],
            vec![f32::INFINITY, f32::INFINITY, 0.0, 2.0],
            vec![f32::INFINITY, -1.0, f32::INFINITY, 0.0]
        ];

        let expected1 = vec![
            vec![0.0, -1.0, -2.0, 0.0],
            vec![4.0, 0.0, 2.0, 4.0],
            vec![5.0, 1.0, 0.0, 2.0],
            vec![3.0, -1.0, 1.0, 0.0]
        ];

        let len1 = graph1.len();
        let mut calculated1 = graph1.clone();

        floyd_warshall(&mut calculated1, len1);

        assert_eq!(expected1, calculated1);

        let graph2 = vec![
            vec![0.0, 5.0, f32::INFINITY, 10.0],
            vec![f32::INFINITY, 0.0, 3.0, f32::INFINITY],
            vec![f32::INFINITY, f32::INFINITY, 0.0, 1.0],
            vec![f32::INFINITY, f32::INFINITY, f32::INFINITY, 0.0]
        ];

        let expected2 = vec![
            vec![0.0, 5.0, 8.0, 9.0],
            vec![f32::INFINITY, 0.0, 3.0, 4.0],
            vec![f32::INFINITY, f32::INFINITY, 0.0, 1.0],
            vec![f32::INFINITY, f32::INFINITY, f32::INFINITY, 0.0]
        ];

        let len2 = graph2.len();
        let mut calculated2 = graph2.clone();

        floyd_warshall(&mut calculated2, len2);

        assert_eq!(expected2, calculated2);
    }
}
