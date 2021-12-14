use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Polymerization {
    pub template: String,
    pub pairs: HashMap<String, char>,
    pub elements: HashSet<char>,
}

impl Polymerization {
    pub fn parse_string(content: String) -> Result<Polymerization, Box<dyn Error>> {
        let mut polymerization = Polymerization::default();

        let lines = content.lines().collect::<Vec<_>>();

        polymerization.template = lines[0].to_string();
        for (index, c) in lines[0].chars().enumerate() {
            polymerization.elements.insert(c);
        }


        for &line in lines[2..].iter() {
            polymerization.pairs.insert(line[0..2].to_string(), line.chars().nth(6).unwrap());
            polymerization.elements.insert(line.chars().nth(0).unwrap());
            polymerization.elements.insert(line.chars().nth(1).unwrap());
            polymerization.elements.insert(line.chars().nth(6).unwrap());
        }

        Ok(polymerization)
    }

    pub fn polymerize(&self, steps: usize) -> usize {
        let mut pair_counts = HashMap::new();

        // Initialize our elements count
        let mut element_counts : HashMap<char, usize> = HashMap::from_iter(
            self.elements.iter().map(|elt| (*elt, 0))
        );
        for c in self.template.chars() {
            if let Some(n) = element_counts.get_mut(&c) {
                *n += 1;
            }
        }

        for i in 1..self.template.len() {
            match pair_counts.get_mut(&self.template[i-1..=i].to_string()) {
                Some(n) => {
                    *n += 1;
                }
                None => {
                    pair_counts.insert(self.template[i-1..=i].to_string(), 1);
                }
            }
        }

        for step in 0..steps {
            eprintln!("step {}", step);

            let old_pairs = pair_counts.clone();

            for (pair, &count) in &old_pairs {
                let new_char = self.pairs[pair];

                let new_first_pair = format!("{}{}", pair.chars().nth(0).unwrap(), new_char);
                let new_second_pair = format!("{}{}", new_char, pair.chars().nth(1).unwrap());

                if let Some(n) = pair_counts.get_mut(pair) {
                    *n -= count;
                }

                match pair_counts.get_mut(&new_first_pair) {
                    None => {
                        pair_counts.insert(new_first_pair, count);
                    }
                    Some(n) => {
                        *n += count;
                    }
                }
                match pair_counts.get_mut(&new_second_pair) {
                    None => {
                        pair_counts.insert(new_second_pair, count);
                    }
                    Some(n) => {
                        *n += count;
                    }
                }

                match element_counts.get_mut(&new_char) {
                    None => {}
                    Some(n) => { *n += count; }
                }
            }
        }

        let mut max_count = 0;
        let mut min_count = usize::MAX;

        for (_, count) in element_counts {
            if count > max_count {
                max_count = count;
            }
            if count < min_count {
                min_count = count;
            }
        }

        max_count - min_count
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::models::{Polymerization};

    #[test]
    fn parse_example_case() {
        let content = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"
        .to_string();

        let input = Polymerization::parse_string(content).unwrap();

        assert_eq!(input, Polymerization {
            template: "NNCB".to_string(),
            pairs: HashMap::from_iter(vec![
                ("CH".to_string(), 'B'),
                ("HH".to_string(), 'N'),
                ("CB".to_string(), 'H'),
                ("NH".to_string(), 'C'),
                ("HB".to_string(), 'C'),
                ("HC".to_string(), 'B'),
                ("HN".to_string(), 'C'),
                ("NN".to_string(), 'C'),
                ("BH".to_string(), 'H'),
                ("NC".to_string(), 'B'),
                ("NB".to_string(), 'B'),
                ("BN".to_string(), 'B'),
                ("BB".to_string(), 'N'),
                ("BC".to_string(), 'B'),
                ("CC".to_string(), 'N'),
                ("CN".to_string(), 'C'),
            ]),
            elements: HashSet::from_iter(vec![
                'N', 'C', 'B', 'H'
            ])
        })
    }

    #[test]
    fn part_1_example_case() {
        let input = Polymerization {
            template: "NNCB".to_string(),
            pairs: HashMap::from_iter(vec![
                ("CH".to_string(), 'B'),
                ("HH".to_string(), 'N'),
                ("CB".to_string(), 'H'),
                ("NH".to_string(), 'C'),
                ("HB".to_string(), 'C'),
                ("HC".to_string(), 'B'),
                ("HN".to_string(), 'C'),
                ("NN".to_string(), 'C'),
                ("BH".to_string(), 'H'),
                ("NC".to_string(), 'B'),
                ("NB".to_string(), 'B'),
                ("BN".to_string(), 'B'),
                ("BB".to_string(), 'N'),
                ("BC".to_string(), 'B'),
                ("CC".to_string(), 'N'),
                ("CN".to_string(), 'C'),
            ]),
            elements: HashSet::from_iter(vec![
                'N', 'C', 'B', 'H'
            ])
        };

        assert_eq!(input.polymerize(10), 1588);
    }

    #[test]
    fn part_2_example_case() {
        let input = Polymerization {
            template: "NNCB".to_string(),
            pairs: HashMap::from_iter(vec![
                ("CH".to_string(), 'B'),
                ("HH".to_string(), 'N'),
                ("CB".to_string(), 'H'),
                ("NH".to_string(), 'C'),
                ("HB".to_string(), 'C'),
                ("HC".to_string(), 'B'),
                ("HN".to_string(), 'C'),
                ("NN".to_string(), 'C'),
                ("BH".to_string(), 'H'),
                ("NC".to_string(), 'B'),
                ("NB".to_string(), 'B'),
                ("BN".to_string(), 'B'),
                ("BB".to_string(), 'N'),
                ("BC".to_string(), 'B'),
                ("CC".to_string(), 'N'),
                ("CN".to_string(), 'C'),
            ]),
            elements: HashSet::from_iter(vec![
                'N', 'C', 'B', 'H'
            ])
        };

        assert_eq!(input.polymerize(40), 2188189693529);
    }
}
