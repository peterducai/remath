pub mod matrix2d {
    pub struct Matrix {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
        pub rows: u16,
        pub columns: u16,
    }

    pub fn remath_matrix_2d(rows: u16, columns:u16) {
        println!("The value of x is: {rows} {columns}");
    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
}