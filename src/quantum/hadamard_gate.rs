pub mod hadamard_gate {

    pub struct HGate {
        pub set: String,
        pub operations: Vec<String>,
        pub axioms: Vec<String>,
    }

    pub static DESCRIPTION: &str = "This gate is a pi rotation about the X+Z axis, and has the effect of changing computation basis from  |0\rangle,|1\rangle
    to |+\rangle,|-\rangle
    and vice-versa.";
    pub static LATEX: &str = "\\begin{split}H = \\frac{1}{\\sqrt{2}} \
     \\begin{pmatrix} \
         1 & 1 \\\\ \
         1 & -1 \
    \\end{pmatrix}\\end{split}";

    //fn implementation_detail() {}

}