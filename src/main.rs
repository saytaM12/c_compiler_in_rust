use std::fs;

fn main() {

    let file_path = "source.c";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut chars =  contents.chars().peekable();

    while let Some(curr) = chars.next() {
        let next = chars.peek();

        if next.is_some() {
            let next = next.unwrap();

            if next.is_alphabetic() {
            }
        }

        if curr.is_alphabetic() {
        }
    }
}
