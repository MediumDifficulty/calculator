pub fn process(input: String) -> String {
    input.chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}