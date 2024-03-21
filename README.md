# About

EasyRandom is a library to help generating random patterns adhering to specific patterns that you want 

Suppose we want to generate random strings of pattern "aaaa bbbnnn ccccc" where

 -  a   -> lower case english alphabets 
 -  b   -> upper case english alphabets
 -  c   -> Both upper case and lower case english alphabets
 -  n   -> numbers from 0 to 9
 - ' ' -> space 
 - '_' -> underscore
 
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

We can exclude any characters if we want through passing a char array or a vector
```rust 
let mut sp_gen = EasyRandom::new();
sp_gen.exclude_chars(&['a','i','e','o','u']); //removing all the vowels from the generator
for i in 0..10{
    let  output=sp_gen.generate(String::from("aaaaa aaa__aa")); 
    println!("{}",output);
}
/*
tfzvy rfb__rq
bvnvt cfq__xl
tyhkq bwt__cd
twttm krw__th
wqwfc zqd__nq
srjcj wys__mg
qfktg bvs__gs
xkqgf smb__rl
xpgwv dcq__zd
kclkc ggf__vx
*/
//
```
You can exclude numbers also 
```rust 
let mut sp_gen = EasyRandom::new();
sp_gen.exclude_chars(&['3','1','4','9','2']); //removing all the vowels from the generator
for i in 0..10{
    let  output=sp_gen.generate(String::from("aaaaannnn")); 
    println!("{}",output);
}
/*
rjicw0005
xjgak6067
ihxfw8705
ptlxt8085
pvdcx8788
pkmga6707
xakoy6080
amxyl8608
bkbaz6700
uuluz5785
*/
```

# Usage
You can add it from the terminal
```bash
cargo add easy_random 
```

You can also add this to TOML file
```
[dependencies]
easy_random = "0.2.4"
```
    	
Here is the full sample code
```rust
use easy_random::EasyRandom;

fn  main(){
    let mut sp_gen = EasyRandom::new();
    // _ and ' ' are passed as it is from pattern
    let output1 = sp_gen.generate(String::from("aa bbb cc__c nnnn"));
    // ks HCP GB__q 6751

    // You can pass a &[char] to exclude characters like vowels
    sp_gen.exclude_chars(&['a','e','i','o','u']); 
    // -----------------------OR-------------------------------
    //  sp_gen.exclude_chars_from_vec(vec!['a','e','i','o','u']);
    let output2 = sp_gen.generate(String::from("aaaaabbbb")); 
    // rynzfVVSG

    // You can use s in the pattern to add you own custom chars
    sp_gen.add_custom_chars_from_vec(vec!['♠', '♣', '♥', '♦', '←', '↑', '→', '↓', '↔']);
    // ------------------- OR ---------------------------------
    // sp_gen.add_custom_chars(&['♠', '♣', '♥', '♦', '←', '↑', '→', '↓', '↔']);
    
    let output3 = sp_gen.generate(String::from("aaaa s aaaa"));
    // wtjw ♠ slxf
}
```

# Todo
- ~~function exclude some characters~~
- ~~Add special characters~~


