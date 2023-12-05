fn main() {
    let input = include_str!("../../input-1.txt");
    println!("Part 1 result: {}", process(input).expect("Output"));
}

fn process(input: &str) -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../../test-input-1.txt");
        assert_eq!("todo!()", process(input)?);
        Ok(())
    }
}
