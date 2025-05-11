// tensor
pub mod tensor {
    
    //Unlike a matrix, a tensor adapts to changes in the coordinate system,
    //making it crucial in fields like physics and machine learning, where
    //transformations are common. When coordinates shift, tensors transform
    //accordingly to maintain the same representation in a new system, while
    //matrices cannot automatically adapt.
    //Any rank-2 tensor can be represented as a matrix, but not every matrix is really a rank-2 tensor.
    //In this sense general nonsquare matrices aren't tensors, nor are the matrices that represent just some sets of linear equations instead of objects that are bound to geometry of the physical space.
    // 
    // Tensors are very commonly symmetric (Hermitian).
    pub struct Tensor {
        pub latex: String, //T, \mathbf{T}, \mathsf{T}, \mathcal{T}, \mathbb{T}
        pub set: String,
        pub contravariant_indices: Vec<String>, //^ superscript
        pub covariant_indices: Vec<String>, //_ subscript
        pub order: i64,
        pub columns: i64,
    }

    /// create tensor
    pub fn remath_tensor(rows: i64, columns:i64) {
        println!("The value of x is: {rows} {columns}");
    }

    pub fn remath_tensor_add() {

    }
    
    //A particularly important tensor is the “copy” tensor, also known as the “diagonal”, “kronecker delta” or “spider” tensor. The simplest version is the all-ones vector, which we
    // write as −. That is i = 1. The general order-n tensor is 1 on the diagonal, 0 everywhere
    // else:
    // i,j,k,... =
    // (
    // 1 if i = j = k = . . .
    // 0 otherwise
    // Or, using Iversonian notation,1
    // i,j,k,... = [i = j = k = . . . ]. We see the order-2 copytensor, − − = I, is just the identity matrix, so we can simply remove it from graphs like
    // this:
    // −A− −B− = −A−B−
    pub fn kronecker_delta() {
    //\delta^i_j = \begin{cases} 1 & \text{if } i = j \\ 0 & \text{if } i \neq j \end{cases}
    }
    
    pub fn metric_tensor() {
        //g_{\mu\nu} = \begin{pmatrix} -1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix}
    }
    
    pub fn levi_civita_symbol() {
        //\varepsilon_{ijk} = \begin{cases} +1 & \text{if } (i,j,k) \text{ is an even permutation of } (1,2,3) \\ -1 & \text{if } (i,j,k) \text{ is an odd permutation of } (1,2,3) \\ 0 & \text{if any index is repeated} \end{cases}
        
    }

    //fn remath_matrix_2d<T>(_s: SGen<T>) {}
    
    
    // Transposition
    // In classical matrix notation, transposition flips the indices of a matrix, so that
    // (A^T)ij = Aji.
    // In tensor diagram notation, we have two choices depending on whether we want the
    // position of the edges to be significant. With significant edge positions, we typically
    // let the “left edge” be the first index, and the “right edge” be the second index. Thus
    // transposition requires flipping the edges:
    // ( A )
    // T = A = AT .
    // A fun notation used by some authors is flipping the tensor upside down, A
    // , as a
    // simpler way to flip the left and right edges
    pub fn transpose() {
        
    }
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