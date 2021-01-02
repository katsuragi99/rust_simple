use std::{io, io::ErrorKind, io::Read};
use std::fs::File;

pub fn file_handle_returner(_file_args: String) -> Result<String, io::Error> {
    let _file_handle = File::open(_file_args);
    let mut _file_handle = match _file_handle {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => { 
                println!("File was not found");
                return Err(e);
            },
            _ => return Err(e),
        },
    };
    let mut _content_holder = String::new();
    _file_handle.read_to_string(&mut _content_holder);
    Ok(_content_holder)
}




#[cfg(test)]
pub mod tests { 
    #[test]
    fn this_will_run() {
        println!("");
    }
}
