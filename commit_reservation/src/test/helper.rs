#[cfg(test)]
pub mod functions{
    use std::fs::File;
    
    pub fn make_test_file() -> bool{
        match File::create("test.txt"){
            Ok(..) => true,
            _ => false
        }
    }

}