pub fn translate_multiplier_to_char(multiplier: i32) -> char {
    match multiplier {
        1 => 'S',
        2 => 'D',
        3 => 'T',
        _ => panic!("Invalid multiplier"),
    }
}
