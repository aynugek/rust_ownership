fn main() {
    let string1 = "Hakuna";
    let string2 = " Matata";

    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = string1.to_owned();
    result.push_str(string2);
    result
}
