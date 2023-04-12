use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {

    
    // let out_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
    // let wasm_path = Path::new(&out_dir).join("wasm32-unknown-unknown/release/jastery.wasm");

    let wasm_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("wasm");
    let wasm_file_path = Path::new(&wasm_dir).join("target").join("wasm_jastery_bg.wasm");

    if !wasm_file_path.exists() {
        panic!("The wasm file does not exist at the expected path: {:?}", wasm_file_path);
    }

    let mut wasm_file = fs::File::open(&wasm_file_path).expect("Failed to open wasm file");
    let mut wasm_data = Vec::new();
    wasm_file.read_to_end(&mut wasm_data).expect("Failed to read wasm file");

    let wasm_md5 = md5::compute(wasm_data);
    let wasm_md5_string = format!("{:x}", wasm_md5);

    // let new_wasm_path = wasm_path.with_file_name(format!("jastery-{}.wasm", wasm_md5_string));

    // if new_wasm_path.exists() {
    //     println!("The wasm file with the correct hash already exists: {:?}", new_wasm_path);
    // } else {
    //     fs::rename(&wasm_path, &new_wasm_path)
    //         .expect("Failed to rename wasm file");
    //     println!("Renamed wasm file to {:?}", new_wasm_path);
    // }


    // let input_dir = "wasm/target";
    let js_file = "wasm_jastery.js";
    let js_file_path = Path::new(&wasm_dir).join("target").join(js_file);

    let new_js_file = format!("wasm_jastery.{}.js", wasm_md5_string );
    let new_js_file_path = js_file_path.with_file_name(new_js_file.clone());

    if !new_js_file_path.exists() {
        fs::rename(&js_file_path, &new_js_file_path).expect(&format!("Failed to rename input file: [{}] ", js_file_path.display() ) );
    }

    // let mut content = Vec::new();
    // let mut file = fs::File::open(&input_path).expect(&format!("Failed to open input file [{}]",input_path.display()));
    // file.read_to_end(&mut content).expect(&format!("Failed to read input file [{}]", input_path.display()));

    // let hash = md5::compute(&content);

    // let output_file = format!("wasm_jastery.{}.js", format!("{:x}", hash));
    // let output_path = Path::new(input_dir).join(&output_file);

    // fs::rename(&input_path, &output_path).expect("Failed to rename input file");

    let index_html_path = Path::new(&wasm_dir).join("index.html");
    let mut index_html_content =
        fs::read_to_string(&index_html_path).expect("Failed to read index.html file");
    index_html_content = index_html_content.replace(js_file, &new_js_file);

    fs::write(&index_html_path, index_html_content).expect("Failed to write updated index.html file");

    println!("Cache-busted {} -> {}", js_file, new_js_file);
}
