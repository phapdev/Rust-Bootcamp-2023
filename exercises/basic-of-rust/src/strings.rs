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
    s.push('!');
    s
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = format!("{s1} {s2}"); // {có khoảng cách}
                                   //thay vì dùng dấu "+" thì dùng hàm format!() để kết hợp 2 chuỗi thành một string.
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    // let reversed: String = input.chars().rev().collect();
    // reversed
    let mut reversed = String::new();
    let mut index = input.len();

    while index > 0 {
        index -= 1;
        let char_index = input.chars().nth(index);
        if let Some(ch) = char_index {
            // Chưa hiểu về Some() . Dùng vì khi chỉ reversed.push(char_index) thì bị lỗi
            // reversed.push(char_index as char)
            reversed.push(ch);
        }
    }
    reversed
}

// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    // error 1 test
    // let word_lower = word.to_lowercase();
    // let reversed = word_lower.chars().rev().collect::<String>();
    // if word == reversed {
    //     true
    // }else {
    //     false
    // }

    let word_lower = word.to_lowercase(); // chuyen dau vao về dạng chữ thường
    let word_byte = word_lower.as_bytes(); 
    let lenght = word_byte.len();
    for i in 0..lenght / 2 {
        if word_byte[i] != word_byte[lenght - 1 - i] { 
            return false;
        }
    }
    true
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    //   string.chars().filter(|&c| c == ch).count() // lỗi 😂
    let mut count = 0;
    let chars = string.chars();

    for i in chars {
        if i == ch { // trùng kí tự thì cộng thêm 1
            count += 1;
        }
        if i == ' '{ // gặp khảong trắng thì kết thúc
            break; 
        }
    }
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
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }
}
