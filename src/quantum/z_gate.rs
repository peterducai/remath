pub mod z_gate {

    pub struct ZGate {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
    }

    pub static DESCRIPTION: &str = "The single-qubit Pauli-Z gate \\sigma_z";
    pub static LATEX: &str = "\\begin{split}Z = \\begin{pmatrix}
    1 & 0 \\\\
    0 & -1
    \\end{pmatrix}\\end{split}";

    //fn implementation_detail() {}

}