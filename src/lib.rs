//
// Copyright (C) 2024 dtw.py.
//
// dtw.py is free software; you can redistribute it and/or modify it
// under the terms of the MIT License; see LICENSE file for more details.
//

use pyo3::prelude::*;

/// Compute the p-norm between two time-series.
///
/// The `p-norm`, also known as the `Minkowski space`, is a generalized norm
/// calculation that includes several types of distances based on the value
/// of `p`.
///
/// Common values of `p` include:
///
/// - `p = 1` for the Manhattan (city block) distance;
/// - `p = 2` for the Euclidean norm (distance).
///
/// More details about p-norms can be found on Wikipedia:
/// [Norm (mathematics) - p-norm](https://en.wikipedia.org/wiki/Norm_(mathematics)#p-norm)
///
/// # Arguments
///
/// * `a` - A `Vec<Vec<f64>>` with time-series values.
/// * `b` - A `Vec<Vec<f64>>` with time-series values.
/// * `p` - A `f64` value of the norm to use, determining the type of
///         distance calculated.
///
/// # Notes
///
/// - Both vectors `a` and `b` must have the same length.
/// - The implementation of this DTW distance calculation was adapted from the
///   `DTW_cpp` single header library ([GitHub](https://github.com/cjekel/DTW_cpp)).
///
/// # Returns
///
/// The `p-norm` value between vectors `a` and `b`.
fn p_norm(a: &[f32], b: &[f32], p: f32) -> f32 {
    let mut d: f32 = 0.0;

    for index in 0..a.len() {
        d += (a[index] - b[index]).abs().powf(p);
    }

    d.powf(1.0 / p)
}


/// Calculates the Dynamic Time Warping (DTW) distance between two time series.
///
/// DTW is a method used to measure the similarity between two sequences
/// which may vary in time or speed. It finds the optimal alignment
/// between the two sequences by warping the time axis to minimize
/// the distance between corresponding points.
///
/// # Arguments
///
/// * `a` - The first time series represented as a reference to a 2D vector of `f32` values.
/// * `b` - The second time series represented as a reference to a 2D vector of `f32` values.
/// * `p` - value of the p-norm (Minkowski space) used to calculate the distances between two points. Common values of `p` include:
///     - `p = 1` for the Manhattan (city block) distance;
///     - `p = 2` for the Euclidean norm (distance).
/// 
/// # Notes
/// 
/// More details about p-norms can be found on Wikipedia:
/// [Norm (mathematics) - p-norm](https://en.wikipedia.org/wiki/Norm_(mathematics)#p-norm)
///
/// # Returns
///
/// The DTW distance between the two time series as a single `f32` value.
///
/// # Examples
///
/// ```
/// let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
/// let b = vec![vec![2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0]];
/// let distance = dtw(&a, &b, 1.0);
/// println!("DTW: {}", distance);
/// ```
pub fn dtw_op(a: &[Vec<f32>], b: &[Vec<f32>], p: f32) -> f32 {
    let n: usize = a.len();
    let o: usize = b.len();

    let mut d: Vec<Vec<f32>> = vec![vec![0.0; o]; n];

    d[0][0] = p_norm(&a[0], &b[0], p);

    for (i, _) in b.iter().enumerate().skip(1) {
        d[i][0] = d[i - 1][0] + p_norm(&a[i], &b[0], p);
    }

    for (i, _) in b.iter().enumerate().skip(1) {
        d[0][i] = d[0][i - 1] + p_norm(&a[0], &b[i], p);
    }

    for (i, _) in b.iter().enumerate().skip(1) {
        for (j, _) in b.iter().enumerate().skip(1) {
            d[i][j] = p_norm(&a[i], &b[j], p) + (
                d[i - 1][j].min(d[i][j - 1]).min(d[i - 1][j - 1])
            );
        }
    }

    d[n - 1][o - 1]
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn _dtw(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> PyResult<f32> {
    Ok(dtw_op(&a, &b, 2.0))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_dtw, m)?)?;

    Ok(())
}
