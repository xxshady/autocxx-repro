use autocxx::prelude::*;
include_cpp! {
    #include "repro.h"
    name!(repro)
}

fn main() {
    println!("test");
}
