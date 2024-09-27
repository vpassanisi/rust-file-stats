use serde_json::from_str;
use std::{collections::HashMap, fs::{self}, path::Path};

#[derive(serde::Deserialize)]
struct Config {
    ignore_folders: Vec<String>,
    exts_to_count: Vec<String>
}

struct Data {
    file_count: HashMap<String, i32>,
}

// fn read_js_file(path: &Path, data: &mut Data) {
//     let input = File::open(path).unwrap();
//     let buffered = BufReader::new(input);
//     let line_count: i32 = buffered.lines().count().try_into().unwrap();

//     data.num_of_js_files += 1;
//     data.total_js_lines += line_count;
// }

fn check_dir(path: &Path, data: &mut Data, config: &Config) {
    let paths = fs::read_dir(path).unwrap();

    for dir_entry in paths {
        let entry =  dir_entry.unwrap();
        let file_path = entry.path();
        let file_type = entry.file_type().unwrap();
        let ext = file_path.as_path().extension();
        let name_as_string = entry.file_name().into_string().unwrap();

        if file_type.is_dir() && !config.ignore_folders.contains(&name_as_string) {
            check_dir(file_path.as_path(), data, config);
        } else if !ext.is_none() {
            let ext_as_str = ext.unwrap().to_str().unwrap().to_string();
            let should_count = config.exts_to_count.contains(&ext_as_str);

            if should_count {
                if let Some(val) = data.file_count.get_mut(&ext_as_str) {
                    *val += 1;
                } else {
                    data.file_count.insert(ext_as_str, 1);
                }
            }
        };
    }
}

fn main() {
    let str = fs::read_to_string("./config.json").expect("no config file");
    let config: Config = from_str(&str).unwrap();
    let mut data = Data {
        file_count: HashMap::new()
    };

    for key in &config.exts_to_count {
        data.file_count.insert(key.clone(), 0);
    }

    let cur_path = Path::new("../../Game 2");

    check_dir(cur_path, &mut data, &config);

    for (key, val) in data.file_count {
        println!("{} files: {}", key, val)
    }
}
