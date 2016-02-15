// TV Series checker by Ivan Temchenko 2016
// ivan.temchenko@yandex.ua

extern crate hyper;
extern crate notify_rust;
use std::io;
use std::io::prelude::*;
use std::fs::{File, metadata, create_dir, OpenOptions};
use hyper::Client;
use hyper::header::Connection;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use notify_rust::Notification;
use std::process;

fn main() {
	//set and check 
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

	//checking program arguments
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
	let arg1 = &args[1][..];
	match arg1 {
		//show version info
		"-v" => {
			println!("version 0.3.4 build 021516.2155");
			process::exit(0);
		}
		//show help
		"-h" => {
print!("
||===============|Welcome to tvcheck 0.3.4|=================||
||===========|Author: Ivan Temchenko (C) (@ 2016)|==============||

Options:

tvcheck			: Run without parameters to check new episodes of added series;
	-add URL	: add series with all watched episodes;
	-new URL	: add new series you did not watched yet;
	-v		: version info;
	-h		: show this help message;

If you whant some specifiv episode - manualy edit the file of it in ~/.tvcheck/, but remember that series must remain in line and there is NO support to download series from the middle (2-4 of 1-11 etc.)
||==========================================================||
");
			process::exit(0);
		}
		//add wtached series
		 "-add" => {
                        println!("Adding: {}", args[args.len() - 1]);
			let txt = &args[args.len() -1];
			//adding new line to list
			append(txt);
		}
		//adding new series
		"-new" => {
			println!("Adding new series");
			//making path on new file
			let txt = &args[&args.len() - 1][..];
			let file = &txt.trim_left_matches("http://fs.to/flist/").to_string();
			let mut filem = homedir();
			filem.push(".tvcheck");
			filem.push(file);
			let link = String::new();
			let link = link + txt + "&quality=webdl";
			let target = &filem.to_str().unwrap();
			//add series to list
			append(&link);
			//creating empty new file
			match File::create(target) {
				Ok(file) => file,
				Err(_) => panic!("Unable to create new file!"),
			};
		}
		_ => {}
	}}

	//check if any series added
	if !test(&list.to_string()) {
		//if no list file - ask for link and create list
		println!("List is empty, provide link to episodes list file:");
		let mut txt = String::new();
		io::stdin().read_line(&mut txt).ok().expect("Failed to read line");
		write(txt, &list.to_string());
	}
	//if list exists - process each series
	let vlist = read(&list.to_string());
	for path in vlist {
	 if &path != "" {
		let file = &path.trim_left_matches("http://fs.to/flist/").to_string();
		let mut filem = homedir();
		filem.push(".tvcheck");
		filem.push(file);
		let target = &filem.to_str().unwrap();

		//check watched series
		//if not watched before - adding list of server episodes to local file
		if !test(&target.to_string()) {
			add(&target.to_string(), &path);
		}
	
		//if watched - checking series list
		if test(&target.to_string()) {
			println!("Getting list from {}", &path);
			let season = get(&path);
			println!("Got {} episodes", &season.len());
			println!("Getting watched list from {}", &target);
			let local_series = read(&target);
			println!("Got {} watched episodes", &local_series.len());
			let mut cnt = &season.len() - &local_series.len();
			//if episodes on server more then local
			if cnt > 0 {
				while cnt > 0 {
					let episode = &season[&season.len() - &cnt];
					println!("New episode! {}", &episode);
					cnt -= 1;
					println!("Downloading...");
					let mut dwnl = homedir();
					dwnl.push("Downloads");
					let store = dwnl.to_str().unwrap();
					//starting download manager for this episode
					let status = Command::new("aria2c")
						.arg("-x 5")
						.arg(&episode)
						.arg("-d")
						.arg(store)
						.status()
						.unwrap_or_else(|i| {panic!("Failed to run process: {}", i)});
					println!("Done. Status: {}", status);
					notify();//&episode);
				}
				//add all episodes after downloading to list file
				let new = &filem.to_str().unwrap();
				add(&new.to_string(), &path);
			}
			else { println!("No new episodes found for this series."); }
		}
		//some sht happend
		else {
			println!("Wow, thats unexpected....");
		     }
	} }
}
//loading list from web
fn get(list: &str) -> Vec<String>{
	let client = Client::new();
	let mut responce = client.get(list).header(Connection::close()).send().unwrap();
	let mut body = String::new();	
	responce.read_to_string(&mut body).unwrap();
	let result = body.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
	//check if we did get episodes or some crap
	if &result.len() < &(30) { result }
	else {panic!("Server returned some crap! Stopping to prevent files damage! Try later.");}

}
//read list from local list file
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
//write list to local file
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
//check if file exists in path specified
fn test(path: &String) -> bool {
	match metadata(path) {
		Ok(_) => true,
		Err(_) => false,
	}
}
//get ~ or %HOME% or %UserProfile% path from environement
fn homedir() -> PathBuf {
	let homedir: PathBuf = match env::home_dir() {
		Some(ref p) => p.to_owned(),
		None => PathBuf::from("./"),
	};
	homedir
}
//create file and add each episode from new line
fn add(filem: &String, path: &String) {
	let mut file = match File::create(filem){
		Ok(file) => file,
		Err(_) => panic!("Unable to create file!"),
	};
	let season = get(&path);
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
//open existing file and append one line to it
fn append(line: &String){
	//getting path to list
	let mut path = homedir();
	path.push(".tvcheck");
	path.push("list");
	let path = path.to_str().unwrap();	
	//opening file for writing and append
	let mut target = OpenOptions::new()
		.write(true)
		.append(true)
		.open(path)
		.unwrap();
	//Writing string to file
	match writeln!(target, "{}", line) {
		Err(_) => panic!("Unable to write line to file!"),
		Ok(_) => {},
	};
}
//show system notification if download finished
fn notify() {
	Notification::new()
		.summary("New episode downloaded by tvcheck!")
		.show()
		.unwrap();
}
