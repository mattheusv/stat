use std::process::exit;
use std::env;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Path is required\n");
        println!("Usage: {} PATH", args[0]);
        exit(1);
    }

    let path = Path::new(&args[1]);
    let files = count_file_types(path);

    match files {
        Ok(files) => {
            println!("{0: <10} | {1: <10}", "Extension", "Count");
            for (key, value) in files.iter() {
                println!("{0: <10} | {1: <10}", key, value);
            }
        }
        Err(err) => println!("Error to list files: {}", err),
    };
}

fn count_file_types(dir: &Path) -> Result<HashMap<String, i32>, io::Error> {
    let mut map = HashMap::new();

    for entry in list_dir_entries(dir)? {
        let extension = entry.path().extension().unwrap_or(OsStr::new("other"));
        let count = map
            .entry(extension.to_str().unwrap_or("other").to_string())
            .or_insert(0);
        *count += 1;
    }

    Ok(map)
}

fn list_dir_entries(dir: &Path) -> Result<Vec<DirEntry>, io::Error> {
    let mut files: Vec<DirEntry> = vec![];

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
        files.push(entry);
    }
    Ok(files)
}
