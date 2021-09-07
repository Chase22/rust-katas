mod test;

fn fizz_buzz(number: u8) -> String {
    let mut output = String::new();
    if number % 3 == 0 {
        output += "fizz";
    }
    if number % 5 == 0 {
        output += "buzz"
    }

    if output.is_empty() {
        output = number.to_string()
    }

    return output
}
