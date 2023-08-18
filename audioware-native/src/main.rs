#![allow(dead_code)]

mod first;
mod fourth;
mod second;
mod third;

#[allow(unused_imports)]
use first::first_test;
#[allow(unused_imports)]
use fourth::fourth_test;
#[allow(unused_imports)]
use second::second_test;
#[allow(unused_imports)]
use third::third_test;

pub fn main() -> Result<(), anyhow::Error> {
    // first_test()
    // second_test()
    // third_test()
    fourth_test()
}
