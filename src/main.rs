// TV Series checker by Ivan Temchenko 2016
// ivan.temchenko@yandex.ua

extern crate hyper;
use std::io;
use std::io::prelude::*;
use std::fs::{File, metadata, create_dir};
use hyper::Client;
use hyper::header::Connection;
use std::env;
use std::path::PathBuf;
use std::collections::HashSet;

fn main() {
	let homem = homedir();
	let home = &homem.to_str().unwrap();	

	if !test(&home.to_string()) { match create_dir(&home.to_string()) {
				Err(why) => println!("! {:?}", why.kind()),
				Ok(_) => {},
			} }
	let mut listm = homedir();
	listm.push(".tvcheck");
	listm.push("list");

	let list = &listm.to_str().unwrap();
	if !test(&list.to_string()) {
		println!("List is empty, provide link to episodes list file:");
		let mut txt = String::new();
		io::stdin().read_line(&mut txt).ok().expect("Failed to read line");
		write(txt, &list.to_string());
	}
	
	let vlist = read(&list.to_string());
	for path in vlist {
	 if &path != "" {
		let file = &path.trim_left_matches("http://fs.to/flist/").to_string();
		let mut filem = homedir();
		filem.push(".tvcheck");
		filem.push(file);
		let target = &filem.to_str().unwrap();

		//check watched series
		//if not watched before
		if !test(&target.to_string()) {
			let mut file = match File::create(&filem){
				Ok(file) => file,
				Err(_) => panic!("Unable to create file!"),
			};
			println!("List entry: {}", &path);
			let season = get(&path);
			
			//Writing new series to file
	
			for e in season {
				match file.write_all(&e.as_bytes()){
					Ok(file) => file,
					Err(_) => panic!("Unable to write to file!"),
				};
				match file.write_all(b"\n"){
					Ok(file) => file,
					Err(_) => panic!("fucking new line!"),
				};
			
				println!("Added new Episode: {}", e);
			}
		}
		
		//if watched - checking series list
		if test(&target.to_string()) {
			let season = get(&path);
			let local_series = read(target);
			let remote: HashSet<_> = season.iter().cloned().collect();
			let local: HashSet<_> = local_series.iter().cloned().collect();
			for d in local.difference(&remote) {
				println!("Difference!: {}", d);
			}
		}
		
		else {
			println!("Series allready in list!");
		     }
	} }
}

fn get(list: &str) -> Vec<String>{
	let client = Client::new();
	let mut responce = client.get(list).header(Connection::close()).send().unwrap();
	let mut body = String::new();	
	responce.read_to_string(&mut body).unwrap();
	body.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
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
	eps.lines().map(|s| s.to_owned()).collect::<Vec<_>>()
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

fn homedir() -> PathBuf {
	let homedir: PathBuf = match env::home_dir() {
		Some(ref p) => p.to_owned(),
		None => PathBuf::from("./"),
	};
	homedir
}
