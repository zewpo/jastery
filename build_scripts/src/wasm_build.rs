// build_scripts/src/wasm_build.rs

use regex::{Regex, escape};

use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
// use std::path::PathBuf;
use std::process::Command;

#[allow(dead_code)]
fn main() -> io::Result<()> {
    wasm_build()
}

pub fn wasm_build() -> io::Result<()> {

    let mut root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let script_file_name = Path::new("wasm_build.script");

    let mut script_path = root_dir.join(script_file_name.clone());

    // If the script is not found in the current directory, try relative to the parent directory.
    if !script_path.exists() {
        root_dir = root_dir.parent().unwrap();
        script_path = root_dir.join(script_file_name);
    }
    if !script_path.exists() {
        panic!("The wasm build script does not exist at the expected path: {:?}", script_path);
    }
    run_shell_script(script_path, root_dir)?;

    let wasm_dir = root_dir.join("wasm");

    let wasm_target_dir = wasm_dir.join("target");
    let wasm_file_path = wasm_target_dir.join("wasm_jastery_bg.wasm");

    if !wasm_file_path.exists() {
        panic!("The wasm file does not exist at the expected path: {:?}", wasm_file_path);
    }

    let mut wasm_file = File::open(&wasm_file_path).expect("Failed to open wasm file");
    let mut wasm_data = Vec::new();
    wasm_file.read_to_end(&mut wasm_data).expect("Failed to read wasm file");

    let wasm_md5 = md5::compute(wasm_data);
    let wasm_md5_string = format!("{:x}", wasm_md5);

    let js_file = "wasm_jastery.js";
    let js_file_path = wasm_target_dir.join(js_file);

    let new_js_file = &format!("wasm_jastery.{}.js", wasm_md5_string);
    let new_js_file_path = js_file_path.with_file_name(new_js_file);

    let keep_files = vec![js_file, new_js_file];
    delete_js_files_in_dir(&wasm_target_dir, &keep_files)?;

    if !new_js_file_path.exists() {
        fs::rename(&js_file_path, &new_js_file_path).expect(&format!("Failed to rename input file: [{}] ", js_file_path.display()));
    }

    let index_html_path = wasm_dir.join("index.html");
    let mut index_html_content =
        fs::read_to_string(&index_html_path).expect("Failed to read index.html file");

    let wasm_jastery_pattern = Regex::new(r#"wasm_jastery(\.[a-f0-9]{32})?\.js"#).unwrap();

    index_html_content = wasm_jastery_pattern
        .replace(&index_html_content, new_js_file)
        .to_string();

    fs::write(&index_html_path, index_html_content).expect("Failed to write updated index.html file");

    println!("Cache-busted index.html; {} -> {}", js_file, new_js_file);

    Ok(())
}


fn run_shell_script(script_path: impl AsRef<Path>, working_directory: impl AsRef<Path>) -> io::Result<()> {
    let script_path = script_path.as_ref();
    let working_directory = working_directory.as_ref();

    if !script_path.exists() {
        eprintln!("Error: script not found: {}", script_path.display());
        std::process::exit(1);
    }

    let file = File::open(&script_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let shell_cmd = if cfg!(windows) { "cmd" } else { "sh" };
        let shell_flag = if cfg!(windows) { "/C" } else { "-c" };
        let status = Command::new(shell_cmd)
            .arg(shell_flag)
            .arg(&line)
            .current_dir(&working_directory)
            .status()?;

        if !status.success() {
            eprintln!("Command failed: {}", line);
            std::process::exit(1);
        }
    }

    Ok(())
}

// fn run_shell_script(script_path: PathBuf) -> io::Result<()> {

//     if !script_path.exists() {
//         eprintln!("Error: script not found: {}", script_path.display());
//         std::process::exit(1);
//     }

//     let file = File::open(&script_path)?;
//     let reader = io::BufReader::new(file);

//     for line in reader.lines() {
//         let line = line?;
//         let shell_cmd = if cfg!(windows) { "cmd" } else { "sh" };
//         let shell_flag = if cfg!(windows) { "/C" } else { "-c" };
//         let status = Command::new(shell_cmd)
//             .arg(shell_flag)
//             .arg(&line)
//             .status()?;

//         if !status.success() {
//             eprintln!("Command failed: {}", line);
//             std::process::exit(1);
//         }
//     }

//     Ok(())
// }



fn delete_js_files_in_dir(dir_path: &Path, keep_files: &[&str]) -> io::Result<()> {
    let paths = fs::read_dir(dir_path)?;
    let exclude_pattern = format!(
        r#"^({})$"#,
        keep_files.iter().map(|f| escape(f)).collect::<Vec<_>>().join("|")
    );
    let pattern = Regex::new(&exclude_pattern).unwrap();
    for path in paths {
        let entry = path?;
        let file_name = entry.file_name();
        if let Some(name) = file_name.to_str() {
            if name.ends_with(".js") {
                if !pattern.is_match(name) {
                    fs::remove_file(entry.path())?;
                    println!("Deleting file: {}", entry.path().display());
                } else {
                    println!("Keeping file: {}", entry.path().display());
                }
                
            }
        }
    }
    Ok(())
}
