use std::fs;

pub fn process_folders(folder_name: &str, recursive: bool) {
    let folder = match fs::read_dir(folder_name) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{}:", folder_name);

    for entry in folder {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            let folder_path = match path.to_str() {
                Some(p) => p,
                None => {
                    println!("{}", path.display());
                    continue;
                }
            };
            if recursive {
                process_folders(folder_path, recursive);
            }
        } else {
            let filename = match entry.file_name().into_string() {
                Ok(f) => f,
                _ => {
                    continue;
                }
            };
            println!("{}", filename);
        }
    }
}
