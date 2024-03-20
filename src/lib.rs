use rand::Rng;
use rand::thread_rng;
pub struct  EasyRandom{
    small_case_alphabets:Vec<char>,
    upper_case_alphabets:Vec<char>,
    all_case_alphabets:Vec<char>,
    numbers:Vec<char>,
}

impl EasyRandom{
    pub fn new()-> Self{
        EasyRandom{
            small_case_alphabets:('a'..='z').collect(),
            upper_case_alphabets:('A'..'Z').collect(),
            all_case_alphabets:  (b'a'..=b'z').chain(b'A'..=b'Z').map(|c| c as char).collect(),
            numbers:('0'..'9').collect(),
        }
    }
    fn generate_random_index(&mut self, i:usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..i)
    }

    pub fn exclude_chars(&mut self, chars: &[char]) {
        for &c in chars {
            let exclude_c = c; 
            self.small_case_alphabets.retain(|&x| x != exclude_c);
            self.upper_case_alphabets.retain(|&x| x != exclude_c);
            self.all_case_alphabets.retain(|&x| x != exclude_c);
            self.numbers.retain(|&x| x != exclude_c);
        }
    }
    pub fn exclude_chars_from_vec(&mut self, chars: Vec<char>) {
        for c in chars {
            let exclude_c = c; 
            self.small_case_alphabets.retain(|&x| x != exclude_c);
            self.upper_case_alphabets.retain(|&x| x != exclude_c);
            self.all_case_alphabets.retain(|&x| x != exclude_c);
            self.numbers.retain(|&x| x != exclude_c);
        }
    }
    pub fn generate(&mut self, given: String)->String{

        let mut pattern : String=String::new();
        for c in given.chars(){
            if c.eq(&'a'){
                let n=self.small_case_alphabets.len();
                let i =self.generate_random_index(n);
                let x=self.small_case_alphabets[i];
                pattern.push(x);
            }
            else if c.eq(&'b'){
                let n=self.upper_case_alphabets.len();
                let i =self.generate_random_index(n);
                let x=self.upper_case_alphabets[i];
                pattern.push(x);

            }
            else if c.eq(&'c'){
                let n=self.all_case_alphabets.len();
                let i =self.generate_random_index(n);
                let x=self.all_case_alphabets[i];
                pattern.push(x);

            }
            else if c.eq(&'n'){
                let n=self.numbers.len();
                let i =self.generate_random_index(n);
                let x=self.numbers[i];
                pattern.push(x);
            }
            else if c.eq(&' '){
                pattern.push(' ');
            }
            else{
                pattern= String::from("Please use the characters a(lowercase),b(uppercase),c(upper+lower cases),n(numbers) only to generate");
                break;
            }
        }
        return pattern;
    }
}

