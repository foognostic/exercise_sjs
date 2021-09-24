mod duo;
mod performance;
mod support;

use crate::duo::Duo;
use crate::performance::Performance;

pub fn run(input_path: &str) {
    let concerts = support::read_test_data(input_path).unwrap();
    // let concerts = support::read_test_data("day50-star1/sample.txt").unwrap();
    let mut performance = Performance::new();

    for concert in concerts {
        let bands = support::csv_to_vec(&concert);
        let duos = Duo::combinations(bands);
        for duo in duos {
            performance.watch(duo);
        }
    }

    println!("{}", performance.report(50).join("\n"));
    // println!("{:#?}", performance);
}
