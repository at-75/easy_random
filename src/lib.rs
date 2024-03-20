struct EasyRandom{
    // given: String,
    // numeric_flag: i32,
    // alphabet_flag: i32,
    numeric_char:Vec<char>,
    alphabet_char:Vec<char>,
}
impl EasyRandom{
    fn new()-> Self{
        EasyRandom{
            numeric_char:Vec::new(),
            alphabet_char:Vec::new()
        }
    }
    fn include_alphabets(&mut self, chars: &[char]) {
        for &c in chars {
            if !self.alphabet_char.contains(&c) {
                self.alphabet_char.push(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_alphabets() {
        let mut random=EasyRandom::new();
        random.include_alphabets(&['a','b','c']);
        println!("Vector : {:?}",random.alphabet_char);
    }

}
