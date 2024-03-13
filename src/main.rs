use std::{io, ops::Div};

pub trait Modulo<T: Div> {
    fn modulo(&self, modulo: i32) -> T;
}

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

    pub fn run(&mut self, input: i32) -> String {
        let current: String;
        if input.modulo(3) == 0 && input.modulo(5) == 0 {
            current = String::from("FizzBuzz")
        } else if input.modulo(3) == 0 {
            current = String::from("Fizz")
        } else if input.modulo(5) == 0 {
            current = String::from("Buzz")
        } else {
            current = format!("{input}")
        };
        self.inner = format!("{}{current}", self.inner);

        self.inner.clone()
    }

    pub fn prompt<R, W>(&self, mut reader: R, mut writer: W) -> Result<i32, String>
    where
        R: io::BufRead,
        W: io::Write,
    {
        write!(writer, "Please type a number: ").expect("failed to write output");
        writer.flush().expect("failed to flush prompt");

        let mut input = String::new();
        reader.read_line(&mut input).expect("failed to read input");
        let res = input.trim().parse();
        match res {
            Ok(number) => Ok(number),
            Err(_) => Err(String::from("invalid input")),
        }
    }
}

impl Modulo<i32> for i32 {
    fn modulo(&self, modulo: i32) -> i32 {
        self % modulo
    }
}

fn main() {
    let mut fizz_buzz = FizzBuzz::new();
    loop {
        let mut res = fizz_buzz.prompt(io::stdin().lock(), io::stdout());
        while res.is_err() {
            println!("{}", res.err().unwrap());
            res = fizz_buzz.prompt(io::stdin().lock(), io::stdout());
        }
        let number = res.unwrap();
        let res = fizz_buzz.run(number);
        println!("{res}")
    }
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
            "Two inputs of 1,1 gives output of string\"11\""
        );
    }

    #[test]
    fn given_new_fizzbuzz_when_prompt_returns_number_1_then_run_returns_string_1() {
        // Arrange
        let mut writer = Vec::new();
        let reader = io::Cursor::new(b"1");
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let number = fizz_buzz
            .prompt(reader, &mut writer)
            .expect("failed to read input");
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

    #[test]
    fn given_new_fizzbuzz_when_input_is_invalid_then_prompt_returns_error() {
        // Arrange
        let mut writer = Vec::new();
        let reader = io::Cursor::new(b"a");
        let fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.prompt(reader, &mut writer);

        // Assert
        assert!(
            res.is_err(),
            "given invalid input, the prompt returns an error"
        );
    }

    #[test]
    fn given_fizzbuz_when_input_sequence_is_valid_invalid_valid_then_output_is_only_valid_input() {
        // Arrange
        let mut writer = Vec::new();
        let valid_reader = io::Cursor::new(b"1");
        let invalid_reader = io::Cursor::new(b"a");
        let mut fizz_buzz = FizzBuzz::new();
        let number = fizz_buzz
            .prompt(valid_reader.clone(), &mut writer)
            .expect("failed to read input");
        let _: String = fizz_buzz.run(number);
        let res = fizz_buzz.prompt(invalid_reader, &mut writer);
        assert!(res.is_err(), "invalid input returns error");

        // Act
        let number = fizz_buzz
            .prompt(valid_reader, &mut writer)
            .expect("failed to read input");
        let res: String = fizz_buzz.run(number);

        // Assert
        assert_eq!(
            res, "11",
            "given three consecutive inputs: 1, a, 1, the output is string \"11\""
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_fizzbuzz_input_is_3_then_output_is_string_Fizz() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(3);

        // Assert
        assert_eq!(
            String::from("Fizz"),
            res,
            "Input of number 3 gives output of string \"Fizz\""
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_fizzbuzz_input_is_multiple_of_3_then_output_is_string_Fizz() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(6);

        // Assert
        assert_eq!(
            String::from("Fizz"),
            res,
            "Input of number 6 gives output of string \"Fizz\""
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_fizzbuzz_input_is_5_then_output_is_string_Buzz() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(5);

        // Assert
        assert_eq!(
            String::from("Buzz"),
            res,
            "Input of number 5 gives output of string \"Buzz\""
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_fizzbuzz_input_is_multiple_of_5_then_output_is_string_Buzz() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(10);

        // Assert
        assert_eq!(
            String::from("Buzz"),
            res,
            "Input of number 10 gives output of string \"Fizz\""
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn given_fizzbuzz_input_is_multiple_of_3_and_5_then_output_is_string_FizzBuzz() {
        // Arrange
        let mut fizz_buzz = FizzBuzz::new();

        // Act
        let res = fizz_buzz.run(15);

        // Assert
        assert_eq!(
            String::from("FizzBuzz"),
            res,
            "Input of number 15 gives output of string \"FizzBuzz\""
        );
    }
}
