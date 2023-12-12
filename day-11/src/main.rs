use std::fs;


fn read_input(filename: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filename).expect("Input file not found");
    content.lines().map(|line| line.chars().collect()).collect()
}

fn expand_input(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_input: Vec<Vec<char>> = Vec::new();

    for row in input {
        expanded_input.push(row.clone());
        if row.iter().all(|&c| c == '.') {
            expanded_input.push(row.clone());
        } 
    }

    let mut column = 0;
    while column < expanded_input[0].len() {
        let mut expand_column = true;
        for row in &expanded_input {
            if row[column] == '#' {
                expand_column = false;
                break;
            }
        }

        if expand_column {
            for row in &mut expanded_input {
                row.insert(column, '.');
            }
            column += 1;
        }
        column += 1;
        
    }

    expanded_input
}

// Part 1 solution can be done using part 2 solution, more efficiently too.
fn part_1(input: &Vec<Vec<char>>) -> u64 {
    let expanded_input = expand_input(input);

    let mut sum = 0;

    // Start iterating from top left corner, moving right, then down.
    // The way to only count a pair once, is by starting to iterate from the galaxy onwards.
    for y_1 in 0..expanded_input.len() {
        for x_1 in 0..expanded_input[0].len() {
            if expanded_input[y_1][x_1] != '#' {
                continue;
            }

            for y_2 in y_1..expanded_input.len() {
                let start_x = if y_1 == y_2 { x_1 + 1 } else { 0 };
                for x_2 in start_x..expanded_input[0].len() {
                    if expanded_input[y_2][x_2] != '#' {
                        continue;
                    }
                    sum += (x_1.abs_diff(x_2) + y_1.abs_diff(y_2)) as u64;

                }
            }

        }
    }

    sum
}

fn get_expanded_columns(input: &Vec<Vec<char>>) -> Vec<usize> {
    let mut expanded_columns = Vec::<usize>::new();

    for column in 0..input[0].len() {
        let mut expand_column = true;
        for row in input {
            if row[column] == '#' {
                expand_column = false;
                break;
            }
        }

        if expand_column {
            expanded_columns.push(column);
        }
    }

    expanded_columns
}

fn get_expanded_rows(input: &Vec<Vec<char>>) -> Vec<usize> {
    let mut expanded_rows = Vec::<usize>::new();

    for row in 0..input.len() {
        if input[row].iter().all(|&c| c == '.') {
            expanded_rows.push(row);
        } 
    }

    expanded_rows
}


fn part_2(input: &Vec<Vec<char>>, expansion: u64) -> u64 {
    let expanded_columns = get_expanded_columns(input); 
    let expanded_rows = get_expanded_rows(input); 

    let mut sum = 0;

    let mut galaxies = Vec::<(usize, usize)>::new(); 
    for y_1 in 0..input.len() {
        for x_1 in 0..input[0].len() {
            if input[y_1][x_1] == '#' {
                galaxies.push((x_1, y_1));
            }
        }
    }

    // The way to only count a pair once, is by starting to iterate from the galaxy onwards.
    // Start iterating from top left corner, moving right, then down.
    for (i, &(x_1, y_1)) in galaxies.iter().enumerate() {
        for j in (i+1)..galaxies.len() {
            let (x_2, y_2) = galaxies[j];
            sum += (x_1.abs_diff(x_2) + y_1.abs_diff(y_2)) as u64;

            let expanded_count = 
            expanded_columns.iter()
                .filter(|&c| *c > x_1 && *c < x_2)
                .count() 
            + expanded_rows.iter()
                .filter(|&r| *r > y_1 && *r < y_2)
                .count();

            sum += expansion * expanded_count as u64;

        }

    }
    sum
}

fn main() {
    let input = read_input("input");

    println!("Part 1: {}", part_1(&input));
    println!("Part 1 with part 2 algorithm: {}", part_2(&input, 1));
    println!("Part 2: {}", part_2(&input, 1000000-1));

}
