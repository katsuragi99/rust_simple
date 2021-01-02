use std::env;
use std::process;
use std::rc::Rc;
use std::cell::RefCell;
use file_reader::file_handle_returner;

fn main() {
   let _file_args: Vec<String> = env::args().collect();
   if _file_args.len() <= 1 {
       println!("Provide a filename");
       process::exit(-1);
   }
   let _file: &String = &_file_args[1];
   let handle = file_reader::file_handle_returner(_file.to_string()).unwrap_or_else(|err| {
       println!("Failed because of an error: {:?}", err);
       process::exit(-1);
   });
   println!("{}", handle);
}
