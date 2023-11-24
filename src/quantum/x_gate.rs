pub mod x_gate {

    pub struct XGate {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
    }

    pub static DESCRIPTION: &str = "The single-qubit Pauli-X gate \\sigma_x";
    pub static LATEX: &str = "\\begin{split}X = \\begin{pmatrix}
    0 & 1 \\\\
    1 & 0
    \\end{pmatrix}\\end{split}";

    //fn implementation_detail() {}

}