// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    color.to_string()
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    s
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + " " + &s2;
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    let mut result = String::from("");
    for i in 0..input.len() {
        result.push((input.as_bytes()[input.len() - i - 1]) as char);
    }
    result
}

// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(k: &str) -> bool {
    let word = k.to_lowercase();
    for i in 0..(word.len() / 2) {
        if word.as_bytes()[i] != word.as_bytes()[word.len() - i - 1] {
            return false;
        }
    }
    true
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(k: &str, ch: char) -> usize {
    let string = k.to_lowercase();
    let char_to_bytes = ch as u8;
    let mut count = 0;
    for i in string.chars() {
        println!("{},{}-----------", char_to_bytes as char, i);
        if i as u8 == char_to_bytes {
            count += 1;
        }
    }
    println!("{}", count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }

    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }
}
