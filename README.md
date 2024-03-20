# About

EasyRandom is a library to help generating random patterns adhering to specific patterns that you want 

Suppose we want to generate random strings of pattern "aaaa bbbnnn ccccc" where

 - a -> lower case english alphabets 
 - b -> upper case english alphabets
 - c -> Both upper case and lower case english alphabets
 - n -> numbers from 0 to 9
 - ' ' -> space 
 
```rust
let mut sp_gen = EasyRandom::new();
let output=sp_gen.generate(String::from("aaaa bbbnnn ccccc"));

// We get the following random generated String
// poxy BOG737 ovKwQ
```
 
If we put any other string other then a,b,c,n we  get this string as output:
```rust
let  mut  sp_gen  =  EasyRandom::new();
let  output=sp_gen.generate(String::from("aaD")); // here 'D' is invalid
    
// We get this string 
// Please use the characters a(lowercase),b(uppercase),c(upper+lower cases),n(numbers) only to generate random strings
```
   

  

# Usage
You can add it from the terminal
```bash
cargo add easy_random 
```

You can also add this to TOML file
```
[dependencies]
easy_random = "0.1.3"
```
    	
Here is the full sample code
```rust
use easy_random::EasyRandom;

fn  main(){
    let  mut  sp_gen  =  EasyRandom::new();
	let  output=sp_gen.generate(String::from("aa bbb ccc nnnn"));
    println!("{}",output);
	// ks HCP GBq 6751
}
```


