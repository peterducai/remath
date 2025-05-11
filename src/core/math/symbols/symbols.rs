pub mod symbols {
    use lazy_static::lazy_static;

    lazy_static!{
        pub static ref SUM: String = String::from("\\sum");
        pub static ref INTEGRAL: String = String::from("\\int");
        pub static ref INFINITY: String = String::from("\\infty");
    }
}