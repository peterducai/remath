use crate::math;
use crate::engine;

pub static LATEX: &str = "\\begin{split}H = \\frac{1}{\\sqrt{2}} \
     \\begin{pmatrix} \
         1 & 1 \\\\ \
         1 & -1 \
    \\end{pmatrix}\\end{split}";

    // \begin{split}H = \frac{1}{\sqrt{2}}
    // \begin{pmatrix}
    // \end{pmatrix}\end{split}
    //     1 & 1 \\
    //     1 & -1

pub fn public_function() {
    println!("matrix func `public_function()`");
    //matrix::matrix_public_function();
    engine::get_version();
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

struct H{
    pub name: String,
    pub matrix_representation: String,
}