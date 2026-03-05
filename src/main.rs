use std::env::args;
use std::fs;
use std::path::Path;

fn main() {
    saulimages::init("cmdline");
    let args: Vec<String> = args().collect();
    let Some(operation) = args.get(1).map(|x|x.to_ascii_lowercase()) else {
        eprintln!("Specify an operation to perform!");
        return;
    };
    let Some(filename) = args.get(2) else {
        eprintln!("Specify a file to edit!");
        return;
    };
    let path = Path::new("inputs").join(filename);
    let buffer = 
        match fs::read(&path) {
            Ok(bytes) => {
                bytes
            }
            Err(e) => {
                eprintln!("Error opening file! {e}");
                eprintln!("Path: {}", path.display());
                return;
            }
        };
    let result = match operation.as_str() {
        "caption" => {
            if args.len() <= 3 {
                eprintln!("Provide a caption!");
                return;
            }
            let caption = args[3..].join(" ");
            saulimages::caption(buffer, &caption)
        }
        "papyrus" => {
            if args.len() <= 3 {
                eprintln!("Provide a caption!");
                return;
            }
            let caption = args[3..].join(" ");
            saulimages::papyrus(buffer, &caption)
        }
        "pugsley" => saulimages::pugsley(buffer),
        "nothing" => saulimages::nothing(buffer),
        "riodejaneiro" => saulimages::rio_de_janeiro(buffer),
        "burn" => saulimages::burn(buffer),
        _ => {
            eprintln!("Unknown operation!");
            return;
        }
    };

    match result {
        Ok(bytes) => {
            let location = Path::new("output").join(operation);
            fs::create_dir_all(&location).unwrap();
            fs::write(location.join(format!("{}.webp",path.file_stem().unwrap().to_string_lossy())), bytes).unwrap();
        }
        Err(e) => {
            eprintln!("Error in image operation: {e}");
            return;
        }
    }
}