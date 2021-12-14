use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub data: Vec<Entry>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    pub patterns: Vec<String>,
    pub values: Vec<String>,
    found_patterns: [usize; 10],
    pub value_to_pattern_index: [usize; 10],
}

impl Entry {
    pub fn new(mut patterns: Vec<String>, values: Vec<String>) -> Entry {
        patterns.sort_by_key(|pattern| pattern.len());
        let mut entry = Entry {
            patterns,
            values,
            found_patterns: [1, 7, 4, 10, 10, 10, 10, 10, 10, 8],
            value_to_pattern_index: [0; 10],
        };
        entry.find_patterns();

        entry
    }

    fn find_patterns(&mut self) {
        // Count the frequency of each segment
        let mut segment_count = [0; 7];
        for pattern in &self.patterns {
            for c in pattern.chars() {
                segment_count[c as usize - 'a' as usize] += 1;
            }
        }

        let one = self.patterns[0].clone();
        let four = self.patterns[2].clone();
        let seven = self.patterns[1].clone();

        // Find segment a
        let mut segment_a = ' ';
        for c in seven.chars() {
            if !one.contains(c) {
                segment_a = c;
                break;
            }
        }

        let segment_b =
            ('a' as u8 + segment_count.iter().position(|&n| n == 6).unwrap() as u8) as char;
        let segment_e =
            ('a' as u8 + segment_count.iter().position(|&n| n == 4).unwrap() as u8) as char;
        let segment_f =
            ('a' as u8 + segment_count.iter().position(|&n| n == 9).unwrap() as u8) as char;

        // Find segment c
        let mut segment_c = ' ';
        for c in one.chars() {
            if c != segment_f {
                segment_c = c;
                break;
            }
        }

        // Find segment d
        let mut segment_d = ' ';
        for c in four.chars() {
            if c != segment_b && c != segment_c && c != segment_f {
                segment_d = c;
                break;
            }
        }

        // Find segment g
        let mut segment_g = ' ';
        for c in "abcdefg".chars() {
            if c != segment_a
                && c != segment_b
                && c != segment_c
                && c != segment_d
                && c != segment_e
                && c != segment_f
            {
                segment_g = c;
                break;
            }
        }

        // Find missing pattern: 0
        let pattern_str = [
            segment_a, segment_b, segment_c, segment_e, segment_f, segment_g,
        ]
        .iter()
        .sorted()
        .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 0;

        // Find missing pattern: 2
        let pattern_str = [segment_a, segment_c, segment_d, segment_e, segment_g]
            .iter()
            .sorted()
            .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 2;

        // Find missing pattern: 3
        let pattern_str = [segment_a, segment_c, segment_d, segment_f, segment_g]
            .iter()
            .sorted()
            .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 3;

        // Find missing pattern: 5
        let pattern_str = [segment_a, segment_b, segment_d, segment_f, segment_g]
            .iter()
            .sorted()
            .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 5;

        // Find missing pattern: 6
        let pattern_str = [
            segment_a, segment_b, segment_d, segment_e, segment_f, segment_g,
        ]
        .iter()
        .sorted()
        .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 6;

        // Find missing pattern: 9
        let pattern_str = [
            segment_a, segment_b, segment_c, segment_d, segment_f, segment_g,
        ]
        .iter()
        .sorted()
        .collect::<String>();
        self.found_patterns[self
            .patterns
            .iter()
            .position(|str| str == pattern_str.as_str())
            .unwrap()] = 9;

        for i in 0..10 {
            self.value_to_pattern_index[self.found_patterns[i]] = i;
        }
    }

    pub fn pattern_to_digit(&self, pattern: String) -> usize {
        let position = self.patterns.iter().position(|p| *p == pattern).unwrap();
        return self.found_patterns[position];
    }
}
