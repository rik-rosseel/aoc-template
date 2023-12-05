fn main() {
    let input = include_str!("../../input-2.txt");
    println!("Part 2 result: {}", process(input).expect("Output"));
}

fn process(input: &str) -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../../test-input-2.txt");
        assert_eq!("todo!()", process(input)?);
        Ok(())
    }
}
