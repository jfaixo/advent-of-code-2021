use std::collections::HashSet;
use std::error::Error;
use ndarray::{arr1, Array1, Array2};
use crate::consts::{compute_rotation_matrixes};

#[derive(Debug)]
pub struct Scanners {
    scanners: Vec<ScannerData>,
    rotation_matrixes: Vec<Array2<i32>>,
}

impl Scanners {
    pub fn parse_string(content: String) -> Result<Scanners, Box<dyn Error>> {
        let mut scanners = Scanners {
            scanners: vec![],
            rotation_matrixes: compute_rotation_matrixes(),
        };

        let lines = content.lines().collect::<Vec<_>>();
        let mut current_scanner = None;
        for i in 0..lines.len() {
            match lines[i] {
                l if l.starts_with("---") => {
                    current_scanner = Some(ScannerData::default());
                }
                l if l.is_empty() => {
                    scanners.scanners.push(current_scanner.unwrap().clone());
                    current_scanner = None;
                }
                l => {
                    let coordinates = l.split(',')
                        .map(|n| {
                            n.parse::<i32>()
                        })
                        .collect::<Result<Vec<i32>, _>>()?;
                    match current_scanner.clone() {
                        None => {}
                        Some(mut scanner) => {
                            scanner.beacons.push(arr1(&[coordinates[0], coordinates[1], coordinates[2]]));
                            current_scanner = Some(scanner);
                        }
                    }
                }
            }
        }

        Ok(scanners)
    }

    pub fn solve(&self) -> (usize, i32) {
        let mut unique_beacons = HashSet::<Array1<i32>>::from_iter(self.scanners[0].beacons.clone());

        let mut remaining_scanners_to_map = Vec::new();
        for i in 1..self.scanners.len() {
            remaining_scanners_to_map.push(i);
        }

        let mut offsets = vec![arr1(&[0, 0, 0])];
        while remaining_scanners_to_map.len() > 0 {
            for i in 0..remaining_scanners_to_map.len() {
                let scanner_index = remaining_scanners_to_map[i];
                match self.find_scanner_mapping(&unique_beacons, scanner_index) {
                    None => {}
                    Some(mapping) => {
                        remaining_scanners_to_map.remove(i);

                        for beacon in &self.scanners[scanner_index].beacons {
                            let v = self.rotation_matrixes[mapping.rotation_matrix_index].dot(beacon) + mapping.offset.clone();
                            unique_beacons.insert(v);
                        }

                        offsets.push(mapping.offset.clone());
                        break;
                    }
                }
            }
        }

        eprintln!("{}", unique_beacons.len());

        let mut max_distance = 0;
        for i in 0..offsets.len() {
            for j in 0..offsets.len() {
                if i != j {
                    let distance = (offsets[i][0] - offsets[j][0]).abs() + (offsets[i][1] - offsets[j][1]).abs() + (offsets[i][2] - offsets[j][2]).abs();
                    if distance > max_distance {
                        max_distance = distance;
                    }
                }
            }
        }

        eprintln!("distance: {}", max_distance);

        (unique_beacons.len(), max_distance)
    }

    /// Returns a pair with the rotation matrix index and the offset if any mapping exists
    fn find_scanner_mapping(&self, unique_beacons: &HashSet<Array1<i32>>, scanner_j: usize) -> Option<ScannersMapping> {
        let beacons_i = unique_beacons;
        let beacons_j = &self.scanners[scanner_j].beacons;

        for unique_beacon in beacons_i {
            for j in 0..beacons_j.len() {
                // Test each rotation of each couple
                for rotation_matrix_index in 0..self.rotation_matrixes.len() {
                    let offset = unique_beacon - self.rotation_matrixes[rotation_matrix_index].dot(&beacons_j[j]);
                    if self.overlaps(unique_beacons, scanner_j, rotation_matrix_index, &offset) {
                        return Some(ScannersMapping {
                            rotation_matrix_index,
                            offset
                        });
                    }
                }
            }
        }

        None
    }

    fn overlaps(&self, unique_beacons: &HashSet<Array1<i32>>, scanner_j: usize, rotation_matrix_index: usize, offset: &Array1<i32>) -> bool {
        let mut overlap_count = 0;
        for j in 0..self.scanners[scanner_j].beacons.len() {
            let v = self.rotation_matrixes[rotation_matrix_index].dot(&self.scanners[scanner_j].beacons[j]) + offset;
            if unique_beacons.contains(&v) {
                overlap_count += 1;

                if overlap_count >= 12 {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Default)]
struct ScannerData {
    beacons: Vec<Array1<i32>>
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct ScannersMapping {
    rotation_matrix_index: usize,
    offset: Array1<i32>
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use ndarray::{arr1, Array1};
    use crate::consts::{compute_rotation_matrixes, EXAMPLE_CASE_STRING};
    use crate::models::ScannersMapping;
    use crate::Scanners;

    #[test]
    fn parse_example_case() {
        let input = Scanners::parse_string(EXAMPLE_CASE_STRING.to_string()).unwrap();

        assert_eq!(input.scanners.len(), 5);
        assert_eq!(input.scanners[0].beacons.len(), 25);
        assert_eq!(input.scanners[1].beacons.len(), 25);
        assert_eq!(input.scanners[2].beacons.len(), 26);
        assert_eq!(input.scanners[3].beacons.len(), 25);
        assert_eq!(input.scanners[4].beacons.len(), 26);
        assert_eq!(input.scanners[4].beacons[25], arr1(&[30,-46,-14]));
    }

    #[test]
    fn test_rotation_matrixes() {
        let v = arr1(&[1, 2, 3]);

        let matrixes = compute_rotation_matrixes();

        assert_eq!(matrixes.len(), 24);

        let rotated_v = matrixes.iter().map(|r| r.dot(&v)).collect::<Vec<Array1<i32>>>();

        // X rotation
        assert_eq!(rotated_v[0], arr1(&[1, 2, 3]));
        assert_eq!(rotated_v[1], arr1(&[1, -3, 2]));
        assert_eq!(rotated_v[2], arr1(&[1, -2, -3]));
        assert_eq!(rotated_v[3], arr1(&[1, 3, -2]));
        // Z 90 + X rotation
        assert_eq!(rotated_v[4], arr1(&[-2, 1, 3]));
        assert_eq!(rotated_v[5], arr1(&[-2, -3, 1]));
        assert_eq!(rotated_v[6], arr1(&[-2, -1, -3]));
        assert_eq!(rotated_v[7], arr1(&[-2, 3, -1]));
        // Z 180 + X rotation
        assert_eq!(rotated_v[8], arr1(&[-1, -2, 3]));
        assert_eq!(rotated_v[9], arr1(&[-1, -3, -2]));
        assert_eq!(rotated_v[10], arr1(&[-1, 2, -3]));
        assert_eq!(rotated_v[11], arr1(&[-1, 3, 2]));
        // Z 270 + X rotation
        assert_eq!(rotated_v[12], arr1(&[2, -1, 3]));
        assert_eq!(rotated_v[13], arr1(&[2, -3, -1]));
        assert_eq!(rotated_v[14], arr1(&[2, 1, -3]));
        assert_eq!(rotated_v[15], arr1(&[2, 3, 1]));
        //Y 90 + X rotation
        assert_eq!(rotated_v[16], arr1(&[3, 2, -1]));
        assert_eq!(rotated_v[17], arr1(&[3, 1, 2]));
        assert_eq!(rotated_v[18], arr1(&[3, -2, 1]));
        assert_eq!(rotated_v[19], arr1(&[3, -1, -2]));
        //Y 270 + X rotation
        assert_eq!(rotated_v[20], arr1(&[-3, 2, 1]));
        assert_eq!(rotated_v[21], arr1(&[-3, -1, 2]));
        assert_eq!(rotated_v[22], arr1(&[-3, -2, -1]));
        assert_eq!(rotated_v[23], arr1(&[-3, 1, -2]));
    }

    #[test]
    fn example_case_overlap_0_1() {
        let input = Scanners::parse_string(EXAMPLE_CASE_STRING.to_string()).unwrap();
        let beacons = HashSet::<Array1<i32>>::from_iter(input.scanners[0].beacons.clone());
        assert_eq!(input.find_scanner_mapping(&beacons, 1), Some(ScannersMapping {
            rotation_matrix_index: 10,
            offset: arr1(&[68, -1246, -43])
        }));
    }

    #[test]
    fn example_case_solve() {
        let input = Scanners::parse_string(EXAMPLE_CASE_STRING.to_string()).unwrap();
        let (beacons_count, max_distance) = input.solve();

        assert_eq!(beacons_count, 79);
        assert_eq!(max_distance, 3621);
    }
}
