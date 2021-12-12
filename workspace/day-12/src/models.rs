use std::error::Error;
use itertools::Itertools;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CaveGraph {
    pub caves: Vec<Cave>,
    pub start_index: usize,
    pub end_index: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
    pub cave_type: CaveType,
    pub cave_name: String,
    pub tunnel_to_cave_index: Vec<usize>
}

#[derive(Debug, Eq, PartialEq)]
pub enum CaveType {
    Start,
    Small,
    Big,
    End,
}

impl CaveGraph {
    pub fn parse_string(content: String) -> Result<CaveGraph, Box<dyn Error>> {
        let mut cave_graph = CaveGraph::default();

        let edges = content.lines()
            .map(|line|{
                let parts = line.split('-').collect::<Vec<&str>>();
                vec![parts[0], parts[1]]
            })
            .collect::<Vec<_>>();

        // Index the unique caves
        let cave_names = edges.clone().into_iter().flatten().sorted().dedup().collect::<Vec<_>>();
        for (cave_index, &cave_name) in cave_names.iter().enumerate() {
            let cave_type = match cave_name {
                "start" => {
                    cave_graph.start_index = cave_index;
                    CaveType::Start
                },
                "end" => {
                    cave_graph.end_index = cave_index;
                    CaveType::End
                },
                n if n.chars().next().unwrap().is_uppercase() => CaveType::Big,
                _ => CaveType::Small,
            };

            let mut tunnel_to_cave_index = Vec::new();
            for edge in &edges {
                let start = edge[0];
                let end = edge[1];

                if start == cave_name {
                    tunnel_to_cave_index.push(cave_names.iter().position(|&name| name == end).unwrap());
                }
                else if end == cave_name {
                    tunnel_to_cave_index.push(cave_names.iter().position(|&name| name == start).unwrap())
                }
            }

            cave_graph.caves.push(Cave {
                cave_type,
                cave_name: cave_name.to_string(),
                tunnel_to_cave_index
            });
        }

        Ok(cave_graph)
    }

    pub fn find_all_paths_count(&self) -> usize {
        let mut possible_path_count = 0;

        // A state is the state of a node visit, with the index of the visited node, and history of already visited nodes (for small ones)
        let mut states = Vec::with_capacity(100);
        states.push((self.start_index, vec![false; self.caves.len()]));
        while states.len() > 0 {
            let (current_index, mut visited_caves) = states.pop().unwrap();
            match &self.caves[current_index] {
                Cave { cave_type: CaveType::End, cave_name, tunnel_to_cave_index } => {
                    possible_path_count += 1;
                }
                Cave { cave_type, cave_name, tunnel_to_cave_index } if *cave_type == CaveType::Small || *cave_type == CaveType::Start => {
                    visited_caves[current_index] = true;
                    for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                        if !visited_caves[next_cave_index] {
                            states.push((next_cave_index, visited_caves.clone()));
                        }
                    }
                }
                Cave { cave_type: CaveType::Big, cave_name, tunnel_to_cave_index } => {
                    for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                        if !visited_caves[next_cave_index] {
                            states.push((next_cave_index, visited_caves.clone()));
                        }
                    }
                }
                _ => {}
            }
        }

        possible_path_count
    }

    pub fn find_all_paths_with_twice_small_visit_count(&self) -> usize {
        let mut possible_path_count = 0;

        // A state is the state of a node visit, with the index of the visited node, and history of already visited nodes (for small ones)
        let mut states = Vec::with_capacity(100);
        states.push((self.start_index, vec![false; self.caves.len()], false));
        while states.len() > 0 {
            let (current_index, mut visited_caves, twice_visit_done) = states.pop().unwrap();
            match &self.caves[current_index] {
                Cave { cave_type: CaveType::End, cave_name, tunnel_to_cave_index } => {
                    possible_path_count += 1;
                }
                Cave { cave_type, cave_name, tunnel_to_cave_index } if *cave_type == CaveType::Small || *cave_type == CaveType::Start => {
                    if twice_visit_done {
                        visited_caves[current_index] = true;
                        for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                            if !visited_caves[next_cave_index] {
                                states.push((next_cave_index, visited_caves.clone(), true));
                            }
                        }
                    }
                    else {
                        visited_caves[current_index] = true;
                        for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                            if next_cave_index != self.start_index {
                                if visited_caves[next_cave_index] {
                                    states.push((next_cave_index, visited_caves.clone(), true));
                                }
                                else {
                                    states.push((next_cave_index, visited_caves.clone(), false));
                                }
                            }
                        }
                    }
                }
                Cave { cave_type: CaveType::Big, cave_name, tunnel_to_cave_index } => {
                    if twice_visit_done {
                        for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                            if !visited_caves[next_cave_index] {
                                states.push((next_cave_index, visited_caves.clone(), true));
                            }
                        }
                    }
                    else {
                        for &next_cave_index in &self.caves[current_index].tunnel_to_cave_index {
                            if next_cave_index != self.start_index {
                                if visited_caves[next_cave_index] {
                                    states.push((next_cave_index, visited_caves.clone(), true));
                                }
                                else {
                                    states.push((next_cave_index, visited_caves.clone(), false));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        possible_path_count
    }
}


#[cfg(test)]
mod tests {
    use crate::models::{Cave, CaveGraph, CaveType};

    #[test]
    fn parse_example_case() {
        let content = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
".to_string();

        let input = CaveGraph::parse_string(content).unwrap();

        assert_eq!(input, CaveGraph {
            caves: vec![
                Cave { cave_type: CaveType::Big, cave_name: "A".to_string(), tunnel_to_cave_index: vec![5, 2, 1, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "b".to_string(), tunnel_to_cave_index: vec![5, 0, 3, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "c".to_string(), tunnel_to_cave_index: vec![0] },
                Cave { cave_type: CaveType::Small, cave_name: "d".to_string(), tunnel_to_cave_index: vec![1] },
                Cave { cave_type: CaveType::End, cave_name: "end".to_string(), tunnel_to_cave_index: vec![0, 1] },
                Cave { cave_type: CaveType::Start, cave_name: "start".to_string(), tunnel_to_cave_index: vec![0, 1] },
            ],
            start_index: 5,
            end_index: 4
        });
    }

    #[test]
    fn part_1_example_case() {
        let input = CaveGraph {
            caves: vec![
                Cave { cave_type: CaveType::Big, cave_name: "A".to_string(), tunnel_to_cave_index: vec![5, 2, 1, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "b".to_string(), tunnel_to_cave_index: vec![5, 0, 3, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "c".to_string(), tunnel_to_cave_index: vec![0] },
                Cave { cave_type: CaveType::Small, cave_name: "d".to_string(), tunnel_to_cave_index: vec![1] },
                Cave { cave_type: CaveType::End, cave_name: "end".to_string(), tunnel_to_cave_index: vec![0, 1] },
                Cave { cave_type: CaveType::Start, cave_name: "start".to_string(), tunnel_to_cave_index: vec![0, 1] },
            ],
            start_index: 5,
            end_index: 4
        };

        assert_eq!(10, input.find_all_paths_count());
    }

    #[test]
    fn part_2_example_case() {
        let input = CaveGraph {
            caves: vec![
                Cave { cave_type: CaveType::Big, cave_name: "A".to_string(), tunnel_to_cave_index: vec![5, 2, 1, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "b".to_string(), tunnel_to_cave_index: vec![5, 0, 3, 4] },
                Cave { cave_type: CaveType::Small, cave_name: "c".to_string(), tunnel_to_cave_index: vec![0] },
                Cave { cave_type: CaveType::Small, cave_name: "d".to_string(), tunnel_to_cave_index: vec![1] },
                Cave { cave_type: CaveType::End, cave_name: "end".to_string(), tunnel_to_cave_index: vec![0, 1] },
                Cave { cave_type: CaveType::Start, cave_name: "start".to_string(), tunnel_to_cave_index: vec![0, 1] },
            ],
            start_index: 5,
            end_index: 4
        };

        assert_eq!(36, input.find_all_paths_with_twice_small_visit_count());
    }
}
