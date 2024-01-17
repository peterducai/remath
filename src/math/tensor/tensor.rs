#![crate_name = "doc"]

//! tensor
pub mod tensor {
    pub struct Tensor {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
        pub rows: i64,
        pub columns: i64,
    }

    /// create tensor
    pub fn remath_tensor(rows: i64, columns:i64) {
        println!("The value of x is: {rows} {columns}");
    }

    pub fn remath_tensor_add() {

    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
}




// \begin{split}H = \frac{1}{\sqrt{2}}
// \begin{pmatrix}
//     1 & 1 \\
//     1 & -1
// \end{pmatrix}\end{split}


// pub struct Tensor {
//     pub name: String,
//     pub matrix_representation: String,
//     pub latex: String,
//     pub content: String,  //a⊗b is known as an outer product of a and b.
// }
//
// impl TensorA for Tensor {
//     fn init(&self) {
//         self.latex = "$$ a \\otimes b = ab $$"
//         self.content = "va⊗b is known as an outer product of a and b."
//     }
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.name, self.matrix_representation, self.latex)
//     }
// }