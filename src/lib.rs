pub fn get_diamond(c: char) -> Vec<String> {
    let length = (c as u8 - b'A' + 1) as usize;
    let mut diamond: Vec<String> = Vec::with_capacity(length * 2 - 1);

    // Generating the top half of the diamond
    for r in 0..length {
        let mut row = vec![' '; length * 2 - 1];
        let letter = (b'A' as usize + r) as u8 as char;
        let idx = length - r - 1;
        row[idx] = letter; // left side of the diamond
        row[length * 2 - idx - 2] = letter; // right side of the diamond
        diamond.push(row.iter().collect());
    }

    // Generating the bottom half of the diamond
    for r in (0..length - 1).rev() {
        diamond.push(diamond[r].clone());
    }

    diamond
}
