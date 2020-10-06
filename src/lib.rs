// Copyright (C) 2020 Joseph R. Quinn
// SPDX-License-Identifier: MIT

#![feature(const_generics)]

#![allow(incomplete_features)]

use std::cmp::PartialOrd;
use std::ops::{ Add, IndexMut };

/// Generic implementation of the Floyd-Warshall algorithm
/// for finding the shortest paths in a weighted graph.
pub fn floyd_warshall<N, M, V>(graph: &N, size: usize) -> N
where
    V: Add<Output=V> + PartialOrd + Copy + Clone + Sized,
    M: IndexMut<usize, Output=V> + Clone + Sized,
    N: IndexMut<usize, Output=M> + Clone + Sized
{
    let mut result_graph = graph.clone();

    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                if output_graph[i][k].parse<f32>() == f32::INFINITY || output_graph[k][j].parse<f32>() == f32::INFINITY {
                    continue;
                }

                if result_graph[i][j] > result_graph[i][k] + result_graph[k][j] {
                    result_graph[i][j] = result_graph[i][k] + result_graph[k][j];
                }
            }
        }
    }

    result_graph
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let length = graph.len();

        let calculated = floyd_warshall(&graph, length);

        assert_eq!(expected, calculated);
    }
}
