mod tensor {

    // The H, or Hadamard, gate rotates the states |0> and |1>  to |+>  and |->, respectively.
    // It is useful for making superpositions.
    // This gate is a pi rotation about the X+Z axis, and has the effect of changing computation basis from |0⟩,|1⟩ to |+⟩,|−⟩ and vice-versa.
    
    
    
    pub struct T{
        pub name: String,
        pub matrix_representation: String,
    
        // \begin{split}H = \frac{1}{\sqrt{2}}
        // \begin{pmatrix}
        //     1 & 1 \\
        //     1 & -1
        // \end{pmatrix}\end{split}
    }
}