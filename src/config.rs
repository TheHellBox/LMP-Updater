use std::collections::HashMap;
use std::path::Path;

pub struct Config {
    data: HashMap<String, String>
}

impl Config{
    pub fn new(path: &'static str) -> Config{
        use std::fs::File;
        use std::io::Read;
        use std::fs;
        use std::io::BufReader;
        use std::io::BufRead;

        let file = File::open(path).unwrap();
        let mut file = BufReader::new(file);

        let mut vals: HashMap<String, String> = HashMap::new();

        for line in file.lines() {
            let l = line.unwrap();
            let conf: Vec<&str> = l.split(" = ").collect();
            vals.insert(conf[0].to_string(), conf[1].to_string());
        }


        Config{
            data: vals
        }
    }

    pub fn get(&self, value: &'static str) -> &String{
        self.data.get(value).unwrap()
    }
}

pub fn create(path: &'static str){
    use std::fs::File;
    use std::io::Read;
    use std::fs;
    use std::io::Write;
    let mut newfile = File::create( path ).unwrap();
    let mut default = b"repo = http://127.0.0.1/LMP-releases/
default_target = client
dir = ./
".to_vec();
    let _ = newfile.write_all(&default);
}
