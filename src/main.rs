fn splitter(string: &str) -> String {
    let chars: Vec<char> = string.chars().collect();
    let mut result = Vec::<String>::new();
    let mut last_mismatch = 0;
    for i in 1..chars.len() {
        if chars[i-1] != chars[i] {
            let temp_result: String = chars[last_mismatch..i].iter().map(|c| c).collect();
            result.push( temp_result );
            last_mismatch = i;
        }
        if i == chars.len() -1{
            let temp_result: String = chars[last_mismatch..chars.len()].iter().map(|c| c).collect();
            result.push( temp_result );
        }
    }
    return result.join(", ");
}

fn main() {
    let test_string = "gHHH5YY++///\\";

    println!("input string: {}", test_string);
    println!("output string: {}", splitter(test_string));
}
