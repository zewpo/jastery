// build.rs
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;


fn main() {
    let code = generate_code();

    let dest_path = Path::new("src").join("generated").join("generated_assets.rs");
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).expect("failed to create directories");
    }
    let mut file = File::create(&dest_path).unwrap();
    file.write_all(code.as_bytes()).unwrap();
}


fn generate_code() -> String {

    // Collect the paths of all PNG and TTF files in the `assets` directory
    let assets_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let paths: Vec<PathBuf> = WalkDir::new(&assets_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .file_name()
                    .map_or(false, |name| 
                            name.to_string_lossy().contains('-') 
                            && !name.to_string_lossy().contains(' '))
                && entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "png" || ext == "ttf")
        })
        .map(|entry| entry.path().to_owned())
        .collect();

    // Generate the code for the HashMap
    let mut code = String::new();
    code.push_str("use std::collections::HashMap;\n");
    code.push_str("use lazy_static::lazy_static;\n");
    code.push_str("lazy_static! {\n");
    
    code.push_str("    pub static ref ASSET_DATA: HashMap<&'static str, Vec<u8>> = {\n");
    code.push_str("        let mut map = HashMap::new();\n");
    for path in &paths {
        let rel_path = if cfg!(windows) {
            path.to_str().unwrap().replace("\\", "\\\\")
        } else {
            path.to_str().unwrap().to_owned()
        };

        let key = path.strip_prefix(&assets_path).unwrap().to_str().unwrap().replace("\\", "/");
        code.push_str(&format!("        map.insert(\"{}\", include_bytes!(\"{}\").to_vec());\n", key, rel_path));
    }
    code.push_str("        map\n");
    code.push_str("    };\n");
    code.push_str("}\n");

    code
}



