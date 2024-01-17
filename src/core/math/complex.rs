pub mod complex {
    pub struct Complex<T> {
        /// Real portion of the complex number
        pub re: T,
        /// Imaginary portion of the complex number
        pub im: T,
    }

    pub type Complex32 = Complex<f32>;
    pub type Complex64 = Complex<f64>;

    impl<T> Complex<T> {
        #[inline]
        pub const fn new(re: T, im: T) -> Self {
            Complex { re, im }
        }
    }
}