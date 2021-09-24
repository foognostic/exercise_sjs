use crate::duo::Duo;
use std::collections::HashMap;

#[cfg(test)]
use crate::support;

#[derive(Clone, Debug)]
pub struct Performance {
    pub duo_performances: HashMap<Duo, usize>,
}

impl Performance {
    pub fn new() -> Self {
        Performance {
            duo_performances: HashMap::new(),
        }
    }

    pub fn watch(&mut self, duo: Duo) {
        *self.duo_performances.entry(duo).or_insert(0) += 1
    }

    pub fn report(&self, floor: usize) -> Vec<String> {
        let mut v = Vec::new();

        for (duo, performances) in self.duo_performances.iter() {
            if *performances >= floor {
                v.push(format!("{},{}", duo.stage_left, duo.stage_right));
            }
        }

        v
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
    fn test_equality() {
        let duo_a = Duo::new("metallica", "sf orchestra");
        let duo_b = Duo::new("sf orchestra", "metallica");

        assert_eq!(duo_a, duo_b);
    }
}
