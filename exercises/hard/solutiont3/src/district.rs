use serde_json::{Value, from_reader};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
}

pub fn process_districts() {
    let file = File::open("exercises/hard/solutiont3/district.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let json_data: Value = from_reader(reader).expect("Failed to parse JSON");

    let batches = json_data.as_object().expect("JSON data is not an object");
    let mut batch_numbers: Vec<i32> = batches.keys()
        .filter_map(|k| k.parse::<i32>().ok())
        .collect();
    batch_numbers.sort();

    let mut results = Vec::new();
    for batch_number in batch_numbers {
        let batch_key = batch_number.to_string();
        let cities_map = batches.get(&batch_key)
            .and_then(Value::as_object)
            .expect("Batch data is not an object");

        let mut all_cities = HashSet::new();
        for (city, neighbors) in cities_map {
            all_cities.insert(city.as_str());
            let neighbors_array = neighbors.as_array().unwrap();
            for neighbor in neighbors_array {
                let neighbor_str = neighbor.as_str().unwrap();
                all_cities.insert(neighbor_str);
            }
        }

        let mut cities: Vec<&str> = all_cities.into_iter().collect();
        cities.sort_unstable();
        let city_indices: HashMap<&str, usize> = cities.iter()
            .enumerate()
            .map(|(i, &city)| (city, i))
            .collect();

        let size = cities.len();
        let mut uf = UnionFind::new(size);

        for (city, neighbors) in cities_map {
            let city_str = city.as_str();
            let &city_id = city_indices.get(city_str).unwrap();
            let neighbors_array = neighbors.as_array().unwrap();
            for neighbor in neighbors_array {
                let neighbor_str = neighbor.as_str().unwrap();
                let &neighbor_id = city_indices.get(neighbor_str).unwrap();
                uf.union(city_id, neighbor_id);
            }
        }

        let mut roots = HashSet::new();
        for i in 0..size {
            roots.insert(uf.find(i));
        }
        results.push(roots.len().to_string());
    }

    println!("{}", results.join(","));
}