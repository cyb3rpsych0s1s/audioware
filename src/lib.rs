use red4ext_rs::prelude::*;

define_plugin! {
    name: "audioware",
    author: "Roms1383",
    version: 1:0:0,
    on_register: {
        register_function!("SumInts", sum_ints);
    }
}

fn sum_ints(ints: Vec<i32>) -> i32 {
    ints.iter().sum()
}
