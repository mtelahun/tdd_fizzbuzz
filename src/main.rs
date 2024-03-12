#[derive(Debug)]
struct FizzBuzz {}
impl FizzBuzz {
    fn run(&self) -> String {
        String::from("")
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::FizzBuzz;

    #[test]
    fn given_fizzbuzz_when_given_empty_input_then_return_empty_output() {
        // Arrange
        let fizz_buzz = FizzBuzz {};

        // Act
        let res = fizz_buzz.run();

        // Assert
        assert!(res.is_empty())
    }
}
