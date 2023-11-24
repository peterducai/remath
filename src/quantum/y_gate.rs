pub mod y_gate {

    pub struct YGate {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
    }

    pub static DESCRIPTION: &str = "The single-qubit Pauli-Y gate \\sigma_y";
    pub static LATEX: &str = "\\begin{split}Y = \\begin{pmatrix}
    0 & -i \\\\
    i & 0
    \\end{pmatrix}\\end{split}";

    //fn implementation_detail() {}

}