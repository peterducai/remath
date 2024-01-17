pub mod version {
    pub struct Matrix {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
        pub rows: i64,
        pub columns: i64,
    }

    pub fn get_version(rows: i64, columns:i64) {
        println!("The value of x is: {rows} {columns}");
    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
}