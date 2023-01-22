pub mod set {

    pub enum SetType {
        Empty,  //empty set, ∅
        Finite,
        Infinite
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
}