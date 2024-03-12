use std::io;

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

    fn prompt<R, W>(&self, mut reader: R, mut writer: W) -> i32
    where
        R: io::BufRead,
        W: io::Write,
    {
        write!(writer, "Please type a number: ").expect("failed to write output");
        writer.flush().expect("failed to flush prompt");

        let mut input = String::new();
        reader.read_line(&mut input).expect("failed to read input");
        let number: i32 = input.parse().expect("you failed to enter a number");

        number
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::FizzBuzz;

    #[test]
    fn given_new_fizzbuzz_when_input_is_1_then_return_string_1() {
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
    fn given_new_fizzbuzz_with_input_1_when_second_input_is_1_then_return_string_11() {
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

    #[test]
    fn given_new_fizzbuzz_when_prompt_returns_number_1_then_run_returns_string_1() {
        // Arrange
        let mut writer = Vec::new();
        let reader = io::Cursor::new(b"1");
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let number = fizz_buzz.prompt(reader, &mut writer);
        let res = fizz_buzz.run(number);

        // Assert
        assert_eq!(
            res, "1",
            "given the number is 1, the output is string \"1\""
        );
        assert_eq!(
            writer, b"Please type a number: ",
            "fizzbuzz prompt's for a number"
        );
    }
}
