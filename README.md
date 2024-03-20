# About

EasyRandom is a library to help generating random patterns adhering to specific patterns that you want 

Suppose we want to generate random strings of pattern "aaaa bbbnnn ccccc" where

 - a -> lower case english alphabets 
 - b -> upper case english alphabets
 - c -> Both upper case and lower case english alphabets
 - n -> numbers from 0 to 9
 - ' ' -> space 

        let mut sp_gen = EasyRandom::new();
        let output=sp_gen.generate(String::from("aaaa bbbnnn ccccc"));
        
       // We get following random generated String
       // poxy BOG737 ovKwQ
 
If we put any other string other then a,b,c,n we  get this string as output:
	

		 let  mut  sp_gen  =  EasyRandom::new();
	     let  output=sp_gen.generate(String::from("aaD")); // here 'D' is invalid
         // We get this string 
         // Please use characters a,b,c,n only to generate

   

  

# Usage

  

```

use easy_random::EasyRandom;

fn  main(){

	let  mut  sp_gen  =  EasyRandom::new();
	let  output=sp_gen.generate(String::from("aa bbb ccc nnnn"));

	println!("{}",output);
	// ks HCP GBq 6751
}

```