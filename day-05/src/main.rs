use std::fs;

struct MapEntry {
    dest_start: u64,
    source_start: u64,
    range_length: u64
}

impl MapEntry{
    pub fn source_end(&self) -> u64 {
        self.source_start + self.range_length
    }
}

fn read_input(filename: &str) -> (Vec<u64>, Vec<Vec<MapEntry>>) {
    let contents = fs::read_to_string(filename).expect("Input file not found.");
    let line_vec: Vec<&str> = contents.lines().collect();
    let mut line_iter = line_vec.iter();

    // Read seeds
    let mut seeds_iter = line_iter.next().unwrap().trim().split(" ").into_iter();
    seeds_iter.next(); // Throw away the "seeds:" string
    let seeds: Vec<u64> = seeds_iter.map(|str| str.parse::<u64>().unwrap()).collect();
    
    // Read maps
    let mut maps: Vec<Vec<MapEntry>> = Vec::new();
    for line in line_iter {
        if line.is_empty(){
            continue;
        }

        // Start a new map
        if line.chars().next().unwrap().is_alphabetic() {
            maps.push(Vec::new());
            continue;
        }

        // Read map
        let mut iter = line.split(" ").into_iter();
        // Fuck this, O(log(n)) instead of O(n), indexing the map.
        maps.last_mut().unwrap().push(MapEntry {
            dest_start: iter.next().unwrap().parse().unwrap(),
            source_start: iter.next().unwrap().parse().unwrap(),
            range_length: iter.next().unwrap().parse().unwrap(),
        });
        
    }


    (seeds, maps)
}

fn part_1(seed_vec: &Vec<u64>, maps: &Vec<Vec<MapEntry>>) -> u64 {
    seed_vec.iter().map(|&seed|{
        maps.iter().fold(seed, |prev, map_entry_vec|{

            for map_entry in map_entry_vec {
                if prev >= map_entry.source_start && prev < map_entry.source_end() {
                    return prev - map_entry.source_start + map_entry.dest_start;
                }
            }

            prev
        })
    }).min().unwrap()
}

// Bruteforce solution. Super shitty.
fn part_2(seed_vec: &Vec<u64>, maps: &Vec<Vec<MapEntry>>) -> u64 {
    let mut min: u64 = u64::MAX;
    let mut total = 0;

    for (i, range) in seed_vec.chunks(2).enumerate() {
        println!("Range {}", i);

        for seed in range[0]..(range[1]+range[0]) {
            total +=1;
            let mut prev = seed;
            'maps:
                for map_entry_vec in maps {

                    for map_entry in map_entry_vec {
                        if prev >= map_entry.source_start && prev < map_entry.source_end() {
                            prev = prev - map_entry.source_start + map_entry.dest_start;
                            continue 'maps;
                        }
                    }
                };

            if prev < min {
                min = prev;
            }
        }

    }
    println!("total {}", total);

    min
}


fn main() {
    let (seeds, maps) = read_input("test_input"); 

    println!("Part 1: {}", part_1(&seeds, &maps));
    println!("Part 2: {}", part_2(&seeds, &maps));
}
