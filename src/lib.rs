use rand::Rng;
use rand::thread_rng;
pub struct  EasyRandom{
    sma:Vec<char>, //smallcase
    uca:Vec<char>, //uppercase
    aca:Vec<char>, //mixedcase
    cc:Vec<char>,  //specialchars
    nms:Vec<char>, //numbers
}

impl EasyRandom{
    pub fn new()-> Self{
        EasyRandom{
            sma:('a'..='z').collect(),
            uca:('A'..'Z').collect(),
            aca:  (b'a'..=b'z').chain(b'A'..=b'Z').map(|c| c as char).collect(),
            nms:('0'..'9').collect(),
            cc:Vec::new(),
        }
    }
    fn generate_random_index(&mut self, i:usize) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..i)
    }
    pub fn add_custom_chars(&mut self,chars:&[char]){
        self.cc.extend(chars);
    }
    pub fn add_custom_chars_from_vec(&mut self,chars:Vec<char>){
        self.cc.extend(chars);
    }
    pub fn exclude_chars(&mut self, chars: &[char]) {
        for &c in chars {
            let exclude_c = c; 
            self.sma.retain(|&x| x != exclude_c);
            self.uca.retain(|&x| x != exclude_c);
            self.aca.retain(|&x| x != exclude_c);
            self.nms.retain(|&x| x != exclude_c);
        }
    }
    pub fn exclude_chars_from_vec(&mut self, chars: Vec<char>) {
        for c in chars {
            let exclude_c = c; 
            self.sma.retain(|&x| x != exclude_c);
            self.uca.retain(|&x| x != exclude_c);
            self.aca.retain(|&x| x != exclude_c);
            self.nms.retain(|&x| x != exclude_c);
        }
    }
    pub fn generate(&mut self, given: String)->String{

        let mut pattern : String=String::new();
        for c in given.chars(){
            if c.eq(&'a'){
                let n=self.sma.len();
                let i =self.generate_random_index(n);
                let x=self.sma[i];
                pattern.push(x);
            }
            else if c.eq(&'b'){
                let n=self.uca.len();
                let i =self.generate_random_index(n);
                let x=self.uca[i];
                pattern.push(x);

            }
            else if c.eq(&'c'){
                let n=self.aca.len();
                let i =self.generate_random_index(n);
                let x=self.aca[i];
                pattern.push(x);

            }
            else if c.eq(&'n'){
                let n=self.nms.len();
                let i =self.generate_random_index(n);
                let x=self.nms[i];
                pattern.push(x);
            }
            else if c.eq(&'s'){
                let n=self.cc.len();
                if n.eq(&0){
                    pattern=String::from("Please add custom chars before using 's' ");
                    break;
                }
                let i =self.generate_random_index(n);
                let x=self.cc[i];
                pattern.push(x);
            }
            else if c.eq(&' '){
                pattern.push(' ');
            }
            else if c.eq(&'_'){
                pattern.push('_');
            }            
            else{
                pattern= String::from("Please use these characters only:
                                         a(lowercase),
                                         b(uppercase),
                                         c(upper+lower cases),
                                         s(custom chars)
                                         n(nms) only to generate");
                break;
            }
        }
        return pattern;
    }
}

