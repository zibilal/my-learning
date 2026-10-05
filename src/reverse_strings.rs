fn reverse_string_four(input: &str) -> String{
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::with_capacity(chars.len());
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        result.push(chars[i]);
    }

    result
}

#[test]
fn test_reverse_string_four() {
    assert_eq!(reverse_string_four("hello world"), "dlrow olleh");
}