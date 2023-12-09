use std::fs;

#[derive(Debug)]
struct Sequence {
    main_sequence: Vec<i64>,
    previous_sequences_vec: Vec<Vec<i64>>
}

fn read_input(filename: &str) -> Vec<Vec<i64>> {
    let content = fs::read_to_string(filename).expect("Input file did not exist");
    content.lines()
        .map(|line| 
            line.split(" ")
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        ).collect()
}

fn get_sequences(sequence_vec: &Vec<Vec<i64>>) -> Vec<Sequence> {
    let mut sequence_vec: Vec<Sequence> = sequence_vec.iter().map(|vec| Sequence { 
        main_sequence: vec.to_vec(), 
        previous_sequences_vec: vec![] 
    })
    .collect();

    for sequence in &mut sequence_vec {
        sequence.previous_sequences_vec.push(sequence.main_sequence.clone());

        let mut prev_seq = &sequence.main_sequence;
        loop {
            let mut iter = prev_seq.iter();
            let mut prev = iter.next().unwrap();
            
            let next_seq: Vec<i64> = iter.map(|num|{
                let value = num - prev;
                
                prev = num;

                value
            }).collect();

            let last_seq =  next_seq.iter().all(|num| *num == 0);       
            sequence.previous_sequences_vec.push(next_seq);

            prev_seq = &sequence.previous_sequences_vec.last().unwrap();

            if last_seq {
                break;
            }
        }
    }

    sequence_vec
}

fn part_1(sequence_vec: &Vec<Sequence>) -> i64 {
    sequence_vec.iter().map(|sequence| {
        sequence.previous_sequences_vec.iter()
            .map(|seq_vec| *seq_vec.last().unwrap())
            .sum::<i64>()
    }).sum()
}

fn part_2(sequence_vec: &Vec<Sequence>) -> i64 {
    sequence_vec.iter().map(|sequence| {
        println!("Seq: {:?}", sequence);
        sequence.previous_sequences_vec.iter()
            .rev()
            .map(|seq_vec| *seq_vec.first().unwrap())
            .fold(0, |running, current| {
                println!("... {}", -running + current);
                -running + current
            })
    }).sum()
}

fn main() {
    let sequences = read_input("input");
    let sequences = get_sequences(&sequences);
    
    println!("Part 1: {}", part_1(&sequences));
    println!("Part 2: {}", part_2(&sequences));
}
