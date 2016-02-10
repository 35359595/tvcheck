// TV Series checker by Ivan Temchenko 2016
// ivan.temchenko@yandex.ua

extern crate hyper;
use std::io;
use std::io::prelude::*;
use std::fs::{File, metadata, create_dir};
use hyper::Client;
use hyper::header::Connection;
use std::env;

fn main() {
	//Some predifined variables here
	static home: String = match env::home_dir() {
		Some(ref p) => p.to_str().unwrap().to_owned(),
		None => "./".to_string(),
		};

	if !test(&home) { match create_dir(&home) {
				Err(why) => println!("! {:?}", why.kind()),
				Ok(_) => {},
			} }
	let list = home.join("/list");
	if !test(&list) {
		println!("List is empty, provide link to episodes list file:");
		let mut txt = String::new();
		io::stdin().read_line(&mut txt).ok().expect("Failed to read line");
		write(txt, &list);
	}
	
	let vlist = read(&list);
	for path in vlist {
	if &path != "" {
		let file = 	&path.trim_left_matches("http://fs.to/flist/").to_string();
		let mut file = match File::create(file){
			Ok(file) => file,
			Err(_) => panic!("Unable to create file!"),
		};
		println!("List entry: {}", &path);
		let season = get(&path);
		for e in season {
		match file.write_all(&e.as_bytes()){
			Ok(file) => file,
			Err(_) => panic!("Unable to write to file!"),
		};
		match file.write_all(b"\n"){
			Ok(file) => file,
			Err(_) => panic!("fucking new line!"),
		}
		println!("Episode: {}", e);
		}
	}}
}

fn get(list: &str) -> Vec<String>{
	let client = Client::new();
	let mut responce = client.get(list).header(Connection::close()).send().unwrap();
	let mut body = String::new();	
	responce.read_to_string(&mut body).unwrap();
	write(body, "episodes.txt");
	read("episodes.txt")
}

fn read(name: &str) -> Vec<String>{
	let mut open = match File::open(name){
		Ok(file) => file,
		Err(_) => panic!("Unable to open file!")
		};
	let mut eps = String::new();
	match open.read_to_string(&mut eps){
		Ok(file) => file,
		Err(_) => panic!("Unable to read from file!"),
		};
	let ret = eps.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
	ret
}

fn write(inpt: String, name: &str){
	 let mut file = match File::create(name){
                Ok(file) => file,
                Err(_) => panic!("Unable to create file!"),
                };
        match file.write_all(inpt.as_bytes()){
                Ok(file) => file,
                Err(_) => panic!("Unable to write to file!"),
        };
}

fn test(path: &String) -> bool {
	match metadata(path) {
		Ok(_) => true,
		Err(_) => false,
	}
}
