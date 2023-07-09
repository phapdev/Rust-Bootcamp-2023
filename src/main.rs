fn main() {
    assert_eq!(count_char_occurrences("hello", 'l'), 2);
}

fn count_char_occurrences(string: &str, ch: char) -> i32 {
   let num:i32 = (string.chars().filter(|&c| c == ch).count()) as i32;
   println!("{num}");
   num
}
