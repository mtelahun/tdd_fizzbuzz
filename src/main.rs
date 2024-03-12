#[derive(Debug, Default)]
struct FizzBuzz {
    inner: String,
}

impl FizzBuzz {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn run(&mut self, input: i32) -> String {
        self.inner = format!("{}{input}", self.inner);

        self.inner.clone()
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
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(1);

        // Assert
        assert_eq!(
            String::from("1"),
            res,
            "Input of number 1 gives output of string \"1\""
        );
    }

    #[test]
    fn given_fizzbuzz_with_input_1_when_second_input_is_1_then_return_string_11() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();
        let _ = fizz_buzz.run(1);

        // Act
        let res = fizz_buzz.run(1);

        // Assert
        assert_eq!(
            String::from("11"),
            res,
            "Two inputs of 1-each gives output of string\"11\""
        );
    }
}
