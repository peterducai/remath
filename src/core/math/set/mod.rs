pub enum SetType {
    Empty,  //The empty set (or null set) is the unique set that has no members. It is denoted ∅, ∅ \emptyset , { }, ϕ,or ϕ.
    Finite,
    Infinite //o describe an infinite set in roster notation, an ellipsis is placed at the end of the list, or at both ends, to indicate that the list continues forever. For example, {0, 1, 2, 3, 4, ...},
}

pub struct Set {
    items: Vec<String>,
    cardinality: u128,
}

    //natural numbers N
    //integers Z
    //rational numbers Q
    //real numbers R
    //complex numbers C



  //
  // The natural numbers
  // \mathbb {N}  are contained in the integers
  // \mathbb {Z} , which are contained in the rational numbers
  // \mathbb {Q} , which are contained in the real numbers
  // \mathbb {R} , which are contained in the complex numbers
  // \mathbb {C}
