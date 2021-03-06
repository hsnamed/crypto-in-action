// Copyright (c) BohuTANG
// Code is licensed with BSD

use std::mem;

/// Implements Euclidean algorithm with non-recursive
///
/// ```text
/// Algorithm:
/// gcd(37,14)
/// 37 = 14(2) + 9
/// 14 = 9(1)  + 5
/// 9  = 5(1)  + 4
/// 5  = 4(1)  + 1
/// ```
/// # Examples
///
/// ```rust
/// use algebra::gcd;
///
/// fn main() {
///     let g = gcd::gcd(37, 14);
///     println!("{}", g);
/// }
/// ```
///
pub fn gcd(mut a: i8, mut b: i8) -> i8 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b)
    }
    a
}

/// Implements Extended Euclidean algorithm with non-recursive
///
/// Given integers a and b, compute integers a and b such that
/// ```text
/// a * x + b * y == gcd(a, b).
/// ```
///
/// # Examples
///
/// ```rust
/// use algebra::gcd;
///
/// fn main() {
///     let (g, x, y) = gcd::xgcd(37, 14);
///     println!("{},{},{}", g,x,y);
/// }
/// ```
///
pub fn xgcd(a: i8, b: i8) -> (i8, i8, i8) {
    let (mut sj, mut sj_last) = (0, 1);
    let (mut tj, mut tj_last) = (1, 0);
    let (mut rj, mut rj_last) = (b, a);

    while rj != 0 {
        let quotient = rj_last / rj;
        rj_last -= quotient * rj;
        sj_last -= quotient * sj;
        tj_last -= quotient * tj;
        mem::swap(&mut rj, &mut rj_last);
        mem::swap(&mut sj, &mut sj_last);
        mem::swap(&mut tj, &mut tj_last);
    }
    (rj_last, sj_last, tj_last) // (gcd, coeff_a, coeff_b)
}
