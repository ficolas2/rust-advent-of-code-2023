

fn get_expanded_tile(c: &char) -> ((char, char), (char, char)) {
    match c {
        '|' => (
            (' ', '|'), 
            (' ', '|')),
        '─' => (
            (' ', ' '), 
            ('─', '─')),
        '└' => (
            (' ', '|'), 
            (' ', '└')),
        '┘' => (
            (' ', '|'), 
            ('─', '┘')),
        '┐' => (
            (' ', ' '), 
            ('─', '┐')),
        '┌' => (
            (' ', ' '), 
            (' ', '┌')),
        'S' => (
            ('x', 'x'), 
            ('x', 'x')),
        '.' => (
            (' ', ' '), 
            (' ', '.')),
        _ => panic!("Invalid character"),
    }
}

pub fn expand(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_map: Vec<Vec<char>> = Vec::new();

    for (y, line) in map.iter().enumerate() {
        expanded_map.push(Vec::new());
        expanded_map.push(Vec::new());
        for (_, c) in line.iter().enumerate() {
            let ((a, b), (c, d)) = get_expanded_tile(c);
            let top = &mut expanded_map[y*2];
            top.push(a);
            top.push(b);
            let bot = &mut expanded_map[y*2 + 1];
            bot.push(c);
            bot.push(d);
        } 
    }

    expanded_map
    
}
