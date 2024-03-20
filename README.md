# About
EasyRandom is a library to help generating pattern of specific types 

# Usage 

```
use easy_random::EasyRandom;

fn main() { 
    let mut str_pattern_generator=EasyRandom::new();
    str_pattern_generator.exclude_chars(&['a','b','C','D','0','1','2','3']);
    let generated_string=str_pattern_generator.generate(String::from("abc abc abbccccc nnnn"));
    println!("{}",generated_string);
}
```