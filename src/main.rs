include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;

fn main() {
    unsafe {
        let area:Area = Area {high: 2, widht: 3};

        CalculationArea(area);
    }
}
