use std::error::Error;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;

pub struct Config {
    source: String,
    destination: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not engough arguments");
        }

        // we get the source file and destination file/dir here
        let source = args[1].clone();
        let destination = args[2].clone();

        Ok(Config {source, destination})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source = config.source;
    let destination = config.destination;

    // driver function does the copying
    do_copy(&source, &destination)?;

    Ok(())
}

pub fn do_copy(source: &str, destination: &str) -> std::io::Result<()> {
   println!("copying in progress: {} to {}", source, destination);

   // Destination can be a file or a directory.
   // If it is a directory
   //   1: create the destination file with the same name as the source file
   //   2: copy content from source to destination file
   //   3: TODO in case of failure, undo the creation of destination file
   // If it is a file
   //   1: If file exists, just copy from source to destination otherwise panics 
   match fs::metadata(destination) {
       Ok(res) => {
           if res.is_dir() {
               let destination_file = create_destination_file(source, destination);
               fs::copy(source, destination_file)?;
           }
           else if res.is_file() {
               fs::copy(source, destination)?;
           }
       }
       Err(error) => {
           match error.kind() {
               ErrorKind::NotFound => {
                   panic!("Destination can either be an existing file or a directory");
               },
               _ => {
                   println!("Error not caught");
                   todo!();
               }
           }
       }
   }

   println!("copying finished");
   Ok(())
}

fn create_destination_file(source_file: &str, destination_dir: &str) -> String {
    let source_file_name = Path::new(source_file).file_name();
    match source_file_name {
        Some(file_name) => {
            let destination_file_name = format!("{}/{}", destination_dir, file_name.to_str().unwrap());
            fs::File::create(&destination_file_name)
                .expect("Problem while creating destination file");
            return  destination_file_name;
        },
        None => {
            panic!("{} not a valid file", source_file);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use file_diff::diff;

    #[test]
    fn copy_single_when_destination_is_file() {
        let source = "/Users/mammoth/Documents/batch1.csv";
        let destination = "/Users/mammoth/Downloads/cargofile.csv";
        
        do_copy(source, destination).expect("Unexpected error copying the file");

        assert!(diff(source, destination));

        if Path::new(destination).exists() {
            fs::remove_file(destination).expect("Unexpected error removing the file");
        } 
    }

    #[test]
    fn copy_single_when_destination_is_dir() {
        let source = "/Users/mammoth/Documents/batch1.csv";
        let destination = "/Users/mammoth/Downloads/";
        
        do_copy(source, destination).expect("Unexpected error copying the file");

        assert!(diff(source, &format!("{}/batch1.csv", destination)));

        if Path::new(&format!("{}/batch1.csv", destination)).exists() {
            fs::remove_file(&format!("{}/batch1.csv", destination)).expect("Unexpected error removing the file");
        } 
    }
}
