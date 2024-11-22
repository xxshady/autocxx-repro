use autocxx::prelude::*;
include_cpp! {
    #include "repro.h"
    name!(repro)
}

pub fn main() {
    println!("test");
}
