#[cfg(test)]
use easy_random::EasyRandom;
mod tests {
    use super::*;

    #[test]
    fn test_include_alphabets() {
        let mut sp_gen = EasyRandom::new();
        sp_gen.exclude_chars(&['3','1','4','9','2']); //removing all the vowels from the generator
        for i in 0..10{
            let  output=sp_gen.generate(String::from("aaaaannnn")); 
            println!("{}",output);
        }
    }
}
