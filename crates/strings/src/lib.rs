pub fn safe_substring(s: &str, start_char_index: usize, end_char_index: usize) -> String {
    let start_byte_index = s.char_indices()
        .nth(start_char_index)
        .map_or_else(|| s.len(), |(index, _)| index);
    let end_byte_index = s.char_indices()
        .nth(end_char_index)
        .map_or_else(|| s.len(), |(index, _)| index);

    s[start_byte_index..end_byte_index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", safe_substring("你好sdfs", 0, 1))
    }
}
