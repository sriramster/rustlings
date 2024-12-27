fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let _len:usize = input.len();

    let mut start_idx:usize = 0;
    let mut end_idx:usize = _len;

    // Get the left index
    for i in input.chars() {
        if i == ' ' {
            start_idx += 1
        } else {
            break
        }
    }

    // Get the right index
    for q in input.chars().rev() {
        if q == ' ' {
            end_idx -= 1
        } else {
            break
        }
    }

    // Return slice
    &input[start_idx .. end_idx]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
