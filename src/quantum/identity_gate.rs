pub mod identity_gate {

    pub struct IGate {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
        pub description: String
    }

    pub static DESCRIPTION: &str = "Identity gate corresponds to a single-qubit gate wait cycle, and should not be optimized or unrolled (it is an opaque gate).";
    pub static LATEX: &str = "\\begin{split}I = \\begin{pmatrix}
    1 & 0 \\\\
    0 & 1
    \\end{pmatrix}\\end{split}";

    //fn implementation_detail() {}

}