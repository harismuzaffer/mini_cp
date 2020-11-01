use std::error::Error;
use std::path::Path;
use std::fs;

pub struct Config {
    source: String,
    destination: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not engough arguments");
        }

        let source = args[1].clone();
        let destination = args[2].clone();

        Ok(Config {source, destination})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source = config.source;
    let destination = config.destination;

    println!("copying in process... {} to {}", source, destination);

    do_copy(&source, &destination)?;

    Ok(())
}

pub fn do_copy(source: &str, destination: &str) -> Result<(), Box<dyn Error>> {
   println!("copying in progress: {} to {}", source, destination);

   let new_destination: String;

   // if fs::metadata(destination).unwrap().is_dir() {
   //     new_destination = format!("{}/output", destination);
   // }

   let bytes = fs::copy(source, destination)?;
   println!("{} bytes copied!", bytes);
   Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use file_diff::diff;

    #[test]
    fn copy_single_destination_file_exists() {
        let source = "/Users/mammoth/Documents/batch1.csv";
        let destination = "/Users/mammoth/Downloads/cargofile.csv";
        
        do_copy(source, destination).expect("Unexpected error copying the file");

        assert!(diff(source, destination));

        if Path::new(destination).exists() {
            fs::remove_file(destination).expect("Unexpected error removing the file");
        } 
    }

    #[test]
    fn copy_single_no_destination_file() {
        let source = "/Users/mammoth/Documents/batch1.csv";
        let destination = "/Users/mammoth/Downloads/";
        
        do_copy(source, destination).expect("Unexpected error copying the file");

        if Path::new(&format!("{}/batch1.csv", destination)).exists() {
            fs::remove_file(&format!("{}/batch1.csv", destination)).expect("Unexpected error removing the file");
        } 

        assert!(diff(source, &format!("{}/batch1.csv", destination)));
    }
}
