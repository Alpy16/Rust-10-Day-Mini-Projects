struct UsernameValidator<'a> {
    // 'a is a lifetime parameter that indicates how long the references in this struct are valid
    username: &'a str,
    timebomb: &'a str,
    // timebomb is to showcase limited window, im using the same lifetime ('a) contract,
    // so both of them only live as long as the shortest one, as structs can only exist as long as their shortest reference with lifetime 'a
}

impl<'a> UsernameValidator<'a> {
    fn validate(&self) -> Result<(), String> {
        if self.username.len() < 3 {
            return Err(String::from("Username must be at least 3 characters long."));
        }
        if self.username.contains(' ') {
            return Err(String::from("Username cannot contain spaces."));
        }
        if self.timebomb != "valid" {
            return Err(String::from("Timebomb has expired."));
        }
        Ok(())
    }
    // impl block correctly uses the &self borrow to inspect the fields without taking ownership.
    // which also means its efficient and doesn't require cloning the username or timebomb, we can just read them directly.
}

fn main() {
    println!("Input Username:");
    let mut input_username = String::new();
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read input.");
    let cleaned_username = input_username.trim();

    let validator = UsernameValidator {
        username: cleaned_username,
        timebomb: "valid", // we can change this to something else to simulate expiration
    };
    match validator.validate() {
        Ok(_) => println!("Username '{}' is valid!", cleaned_username),
        Err(e) => println!("Validation error: {}", e),
    }
}

#[cfg(test)] //
mod tests {
    use super::*; // This imports everything from the code above (the struct and impl), which is not what im used to from solidity

    #[test] // This attribute marks a function as a test case
    fn test_happy_path() {
        let validator = UsernameValidator {
            username: "Alpi",
            timebomb: "valid",
        };
        // assert!(condition) will panic (fail) if the condition is false
        assert!(validator.validate().is_ok());
    }

    #[test]
    fn test_username_too_short() {
        let validator = UsernameValidator {
            username: "Al",
            timebomb: "valid",
        };
        let result = validator.validate();
        assert!(result.is_err());
        // We can check the exact error message
        assert_eq!(
            result.unwrap_err(),
            "Username must be at least 3 characters long."
        );
    }
}
