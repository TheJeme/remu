use std::env;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;

// args = --uppercase | -u, --lowercase | -l, --first-letter-uppercase | -U, --first-letter-lowercase | -L
// brf [dir_path] [file_name] [args]

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid number of arguments. Usage: `brf [dir_path] [file_name] [args]`. Example: `brf ./files img`");
        std::process::exit(0);
    }
    let config = Config::init(&args);
    run(config);
    println!("Done!");
    Ok(())
}
enum Action {
    Rename,
    Uppercase,
    Lowercase,
    FirstLetterUppercase,
    FirstLetterLowercase,
}

struct Config {
    dir_path: String,
    file_name: String,
    text_format: Action,
}

impl Config {
    pub fn init(args: &[String]) -> Config {
        let dir_path = args[1].to_string();
        let mut file_name = String::new();
        let mut text_format = Action::Rename;

        match args[2].as_str() {
            "-uppercase" | "-u" => text_format = Action::Uppercase,
            "-lowercase" | "-l" => text_format = Action::Lowercase,
            "-first-letter-uppercase" | "-U" => text_format = Action::FirstLetterUppercase,
            "-first-letter-lowercase" | "-L" => text_format = Action::FirstLetterLowercase,
            _ => file_name = args[2].to_string(),
        }

        Config {
            dir_path,
            file_name,
            text_format,
        }
    }
}

fn run(config: Config) {
    let paths;
    match fs::read_dir(&config.dir_path) {
        Ok(dir) => paths = dir,
        Err(_) => {
            println!("Path is not a directory!");
            std::process::exit(0)
        }
    }

    let mut number = 0;

    match config.text_format {
        Action::Rename => println!("Renaming files to: `{}_xx`", config.file_name),
        Action::Uppercase => println!("Renaming files to uppercase"),
        Action::Lowercase => println!("Renaming files to lowercase"),
        Action::FirstLetterUppercase => {
            println!("Renaming files first letter to uppercase")
        }
        Action::FirstLetterLowercase => {
            println!("Renaming files first letter to lowercase")
        }
    }

    for path in paths
        .filter_map(Result::ok)
        .filter(|e| e.metadata().unwrap().is_file())
    {
        let file_extension = get_file_extension_from_path(&path);
        let mut file_name = get_file_name_from_path(&path);

        match config.text_format {
            Action::Rename => file_name = format!("{}_{}", config.file_name, number),
            Action::Uppercase => file_name = to_uppper_case(&file_name),
            Action::Lowercase => file_name = to_lower_case(&file_name),
            Action::FirstLetterUppercase => file_name = to_first_letter_upper_case(&file_name),
            Action::FirstLetterLowercase => file_name = to_first_letter_lower_case(&file_name),
        }

        let mut new_file = format!("{}/{}{}", config.dir_path, file_name, file_extension);

        // If file already exists increment number
        while matches!(config.text_format, Action::Rename) && Path::new(&new_file).is_file() {
            number += 1;
            file_name = format!("{}_{}", config.file_name, number);
            new_file = format!("{}/{}{}", config.dir_path, file_name, file_extension);
        }

        fs::rename(path.path(), new_file).unwrap_or_else(|_| println!("Renaming file failed!"));
        number += 1;
    }
}

fn get_file_extension_from_path(path: &DirEntry) -> String {
    path.path()
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| format!(".{}", extension))
        .unwrap_or_else(|| String::from("")) // No extension
}

fn get_file_name_from_path(path: &DirEntry) -> String {
    let file_name = path.file_name().to_str().unwrap().to_string();
    let file_extension = get_file_extension_from_path(path);
    file_name.trim_end_matches(&file_extension).to_string()
}

fn to_uppper_case(s: &str) -> String {
    s.to_uppercase()
}

fn to_lower_case(s: &str) -> String {
    s.to_lowercase()
}

fn to_first_letter_upper_case(s: &str) -> String {
    let mut chars = s.chars();
    let first_char = chars.next().unwrap_or(' ');
    if !first_char.is_alphabetic() {
        return s.to_string();
    }
    let mut result = first_char.to_uppercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}

fn to_first_letter_lower_case(s: &str) -> String {
    let mut chars = s.chars();
    let first_char = chars.next().unwrap_or(' ');
    if !first_char.is_alphabetic() {
        return s.to_string();
    }
    let mut result = first_char.to_lowercase().collect::<String>();
    result.push_str(chars.as_str());
    result
}
