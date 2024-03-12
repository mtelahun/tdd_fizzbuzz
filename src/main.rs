#[derive(Debug)]
struct FizzBuzz {}
impl FizzBuzz {
    fn run(&self, input: i32) -> String {
        format!("{input}")
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::FizzBuzz;

    #[test]
    fn given_fizzbuzz_when_input_is_1_then_return_string_1() {
        // Arrange
        let fizz_buzz = FizzBuzz {};

        // Act
        let res = fizz_buzz.run(1);

        // Assert
        assert_eq!(
            String::from("1"),
            res,
            "Input of number 1 gives output of string \"1\""
        );
    }
}
