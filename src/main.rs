use easy_random::EasyRandom;

fn main(){
    let mut sp_gen = EasyRandom::new();
    let output=sp_gen.generate(String::from("aa bbb ccc nnnn"));
    println!("{}",output);

}