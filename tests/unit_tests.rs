#[cfg(test)]
use easy_random::EasyRandom;
mod tests {
    use super::*;
    #[test]
    fn test_include_alphabets() {
        let mut sp_gen = EasyRandom::new();
        sp_gen.exclude_chars(&['a','e','i','o','u']); //removing all the vowels from the generator
        for _i in 0..10{
            let  output=sp_gen.generate(String::from("aaaaannnn")); 
            println!("{}",output);
        }
    }
    #[test]
    fn test_custom_characters(){
        let mut sp_gen=EasyRandom::new();
        sp_gen.add_custom_chars(&['♠','x','♣']);
        let output:String=sp_gen.generate(String::from("ssssss"));
        println!("{}",output);
        assert!(true);
    }
    #[test]
    fn test_custom_characters_from_vec(){
        let mut sp_gen=EasyRandom::new();
        sp_gen.add_custom_chars_from_vec(vec!['♠','x','♣']);
        sp_gen.add_custom_chars(&['`']);
        sp_gen.exclude_chars(&['♠']);
        let output:String=sp_gen.generate(String::from("ssssss"));
        println!("{}",output);
        assert!(true);
    }
    #[test]
    fn test_invalid_char(){
        let mut sp_gen= EasyRandom::new();
        let output:String = sp_gen.generate(String::from("xxasxax"));
        println!("{}",output);
        assert!(true);
    }
}
