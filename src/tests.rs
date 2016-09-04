use std::io::prelude::*;
use std::fs::OpenOptions;
use std::env;
use std::path::PathBuf;


fn homedir() -> PathBuf {
	let homedir: PathBuf = match env::home_dir() {
		Some(ref p) => p.to_owned(),
		None => PathBuf::from("./"),
	};
	homedir
}

fn append(line: &String, path_given: &String){

	//getting path to list
	let mut path = homedir();
	path.push(".tvcheck");
	if path_given != ""{ path.push(path_given); }
	else { path.push("test"); }
	let path = path.to_str().unwrap();

	//adding new line to text
    let mut text = String::new();
    text.push_str(line);

	//opening file for writing and append
	let target = OpenOptions::new()
        .read(true)
		.write(true)
		.create(true)
		.append(true)
		.open(path)
		.unwrap();

	//Writing string to file
	match writeln!(&target, "{}", text) {
		Err(_) => panic!("Unable to write line to file!"),
		Ok(_) => {},
	};
}


fn main() {
    let line: String = String::from("Fuck!");
    append(&line, &String::from(""))
}
