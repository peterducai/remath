// The Heaviside step function, or the unit step function,
// usually denoted by H or θ (but sometimes u, 1 or 𝟙), is a step function named
// after Oliver Heaviside, the value of which is zero for negative arguments
// and one for positive arguments.
// https://en.wikipedia.org/wiki/Heaviside_step_function
pub mod heaviside_stepfunction {
    pub struct H {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
        pub rows: u16,
        pub columns: u16,
    }

    pub fn remath_heaviside_function (rows: u16, columns:u16) {
        println!("The value of x is: {rows} {columns}");
    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
}