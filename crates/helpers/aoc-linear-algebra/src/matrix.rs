pub struct Matrix<T, const M: usize, const N: usize> {
    raw: [[T; M]; N]
}