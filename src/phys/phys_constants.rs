mod phys_constants {

    enum IpAddrKind {
        V4,
        V6,
    }

    pub struct PhysConst {
        pub name: String,
        pub adr: IpAddrKind,
    }


}