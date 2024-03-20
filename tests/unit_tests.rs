#[cfg(test)]
use easy_random::EasyRandom;
mod tests {
    use super::*;

    #[test]
    fn test_include_alphabets() {
        let mut str_pattern_generator=EasyRandom::new();
        str_pattern_generator.exclude_chars(&['a','b','C','D','0','1','2','3']);
        str_pattern_generator.generate(String::from("ll uuuu XX ppsasd"));
    }
}
