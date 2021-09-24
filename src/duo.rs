use combination::*;
use std::hash::Hash;

#[cfg(test)]
use crate::support;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Duo {
    pub stage_left: String,
    pub stage_right: String,
}

impl Duo {
    pub fn new(band_a: &str, band_b: &str) -> Self {
        let stage_left;
        let stage_right;

        if band_a < band_b {
            stage_left = String::from(band_a);
            stage_right = String::from(band_b);
        } else {
            stage_left = String::from(band_b);
            stage_right = String::from(band_a);
        }

        Duo {
            stage_left,
            stage_right,
        }
    }

    pub fn combinations(bands: Vec<&str>) -> Vec<Duo> {
        if bands.len() < 2 {
            return vec![];
        }

        let mut duos = Vec::new();
        for mut combination in bands.combine_at(2) {
            let some_band = combination.pop().unwrap();
            let some_other_band = combination.pop().unwrap();
            duos.push(Duo::new(some_band, some_other_band));
        }

        duos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let combinations = Duo::combinations(support::csv_to_vec("a,b,c,d"));
        assert_eq!(6, combinations.len());
    }

    #[test]
    fn test_hashing() {
        let duo_a = Duo::new("metallica", "sf orchestra");
        let duo_b = Duo::new("sf orchestra", "metallica");

        assert_eq!(duo_a, duo_b);
    }
}
