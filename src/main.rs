// TV Series checker by Ivan Temchenko (C) 2016

extern crate hyper;
extern crate notify_rust;
extern crate clap;
use std::io;
use std::io::prelude::*;
use std::fs::{ File, metadata, create_dir, OpenOptions };
use hyper::Client;
use hyper::header::Connection;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use notify_rust::Notification;
use clap::{ App, Arg, ArgMatches };

const CUT: &'static str = "/";

fn main() {
	//set and check
	let homem = homedir();
	let home = &homem.to_str().unwrap();
	if !test(&home.to_string()) { match create_dir(&home.to_string()) {
				Err(why) => println!("! {:?}", why.kind()),
				Ok(_) => {},
			}}
	let mut listm = homedir();
	listm.push(".tvcheck");
	listm.push("list");
	let list = &listm.to_str().unwrap();

	//arg parsing
	let matches = parse_args();
	//if add
	let a = matches.value_of("add").unwrap_or("");
		if a != "" { add_series(a.to_string()); }
	//if new
	let n = matches.value_of("new").unwrap_or("");
		if n != "" { new_series(n.to_string()); }
    //if remove
    if matches.is_present("remove"){ remove(); }

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
	for path in vlist { //path - each series episode list file
	 if &path != "" {
		let file = path.trim_left_matches("http://fs.to/flist/").trim_left_matches("http://brb.to/flist/").to_string();
		let mut filem = homedir();
		filem.push(".tvcheck");
		filem.push(&file);
		let target = &filem.to_str().unwrap();

		//check watched series
		//if not watched before - adding list of server episodes to local file
		if !test(&target.to_string()) {
			add(&target.to_string(), &path);
		}

		//if watched - checking series list
		if test(&target.to_string()) {
			let season = get(&path);
			let local_series = read(&target);
			println!("{} episodes on server. {} watched episodes.", &season.len(), &local_series.len());
			let mut cnt = &season.len() - &local_series.len();

			//if list from server is empty
			if &season.len() < &local_series.len() { println!("0 series from server. Possibly, need to redownload link!"); }

			//if episodes on server more then local
			else if cnt > 0 {
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
						.arg(&store)
						.status()
						.unwrap_or_else(|i| {panic!("Failed to run aria2c: {}", i)});

                    let success: i32 = 0; //download status checker
                    if status.code().unwrap() != success { println!("Download failed with code: {}", status.code().unwrap()); }
                    else{
                            let new = &filem.to_str().unwrap();
                            //let name = add(&new.to_string(), &path); //WTF???? why add?
							append(&new.to_string(), &episode);
                            if matches.is_present("silent") != true {
                                notify(&episode);//filem.to_str().unwrap());
                            }
                        }
				}
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
	else {panic!("Server returned some crap! Stopping to prevent files damage! Try again later.");}

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
fn add(filem: &String, path: &String) -> String {
	let mut file = match File::create(filem){
		Ok(file) => file,
		Err(_) => panic!("Unable to create file!"),
	};
	let season = get(&path);
		for e in &season {
			match file.write_all(e.as_bytes()){
				Ok(file) => file,
				Err(_) => panic!("Unable to write to file!"),
			};
			match file.write_all(b"\n"){
				Ok(file) => file,
				Err(_) => panic!("failed to add new line"),
 			};
		}
		//Open function?
		let ep = &season[&season.len() - 1];
		let mut ep = &ep[..];
		while ep.contains(CUT) {
			ep = &ep[1..];
		}

		println!("Added new Episode: {}", &ep);
		ep.to_string()
}
//open existing file and append one line to it
fn append(line: &String, path_given: &String){

	//getting path to list
	let mut path = homedir();
	path.push(".tvcheck");
	if path_given != ""{ path.push(path_given); }
	else { path.push("list"); }
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

//play downloaded series?
fn playit(ep: String){
	let status = Command::new("vlc")
		.arg(ep)
		.status()
		.unwrap_or_else(|i| {panic!("Failed to run aria2c: {}", i)});
	if status != status {}; //to ignore return of process
}

//show system notification if download finished
fn notify(s: &str) {
	//let s = &ep[..];
	let mut episode = homedir();
	episode.push("Downloads");
	episode.push(&s);
	let episode = episode.to_str().unwrap();
	Notification::new()
		.summary("New episode downloaded by tvcheck!")
		.body(&s)
		.action("default", "default")
		.action("play", "Play")
		.show()
		.unwrap()
		.wait_for_action(|action|
			match action {
				"default" => {},
				"play" => { playit(episode.to_string()) },
				"__closed" => {},
				_ => ()
			});
}

//add wtached series
fn add_series(txt: String){
	println!("Adding: {}", &txt);
	//adding new line to list
	append(&txt, &String::from(""));
}

//adding new series
fn new_series(txt: String){

	println!("Adding new series: {}", &txt);

	//making path on new file
	let mut filem = homedir();
	filem.push(".tvcheck");
	filem.push(&txt.trim_left_matches("http://fs.to/flist/").trim_left_matches("http://brb.to/flist/").to_string());

	//converting to str and than ri
	let target = &filem.to_str().unwrap();

	append(&target.to_string(), &String::from(""));
}

//remove ended season
fn remove() {
    let mut home = homedir();
    home.push(".tvcheck/list");
    let home = home.to_str().unwrap();
    let mut list = read(&home);
    let mut count = 0;
    for series in &list{
        let file = &series.trim_left_matches("http://fs.to/flist/").trim_left_matches("http://brb.to/flist/").to_string();
        let mut lf = homedir();
        lf.push(".tvcheck");
        lf.push(file);
        let lf = lf.to_str().unwrap();
        let names = read(&lf);

        //if at leest 1 episode is listed

        if names.len() > 0 {
            let name = names[0].trim_left_matches("http://fs.to/").trim_left_matches("http://brb.to/");
            let size = name.len();
            println!("{}: {}", &count, &name[66..size].replace("."," "));
            count += 1;
        }
    }

    //reading the selection

    let mut choice = String::new();
    println!("Chose a seriec to remove (by number):");
    io::stdin().read_line(&mut choice).ok().expect("Failed to read choice to remove.");
    let choice: usize = choice.trim().parse().ok().expect("Not a number");

    //removing selection

    let removed = list.remove(choice);
    println!("Removing {}", &removed);
    let new = list.as_slice().join("\n");
    write(new, &home);

    //exit after done
    std::process::exit(0);
}

//argumet parser function
fn parse_args<'a>() -> ArgMatches<'a> {
	let matches = App::new("TV Episode Check")
		.version("0.5.0 build 04092016.1300")
		.author("Ivan Temchenko <35359595i@gmail.com>")
		.about("
||===============|Welcome to tvcheck 0.5.0|=================||
||===========|Author: Ivan Temchenko (C) (@ 2016)|==============||

Options:

tvcheck			: Run without parameters to check new episodes of added series;

If you whant some specifiv episode - manualy edit the file of it in ~/.tvcheck/, but remember that series must remain in line and there is NO support to download series from the middle (2-4 of 1-11 etc.)
||==========================================================||
")
		.arg(Arg::with_name("add")
			.short("a")
			.long("add")
			.help("Adds series to list with all watched episodes from a filelist link.")
			.value_name("LINK")
			.takes_value(true))
		.arg(Arg::with_name("new")
			.short("n")
			.long("new")
			.help("Adds series to list and downloads all episodes from a filelist link.")
			.value_name("LINK")
			.takes_value(true))
        .arg(Arg::with_name("remove")
            .short("r")
            .long("remove")
            .help("Remove ended series from list (watched episodes list will remain unremoved)")
            .takes_value(false))
        .arg(Arg::with_name("silent")
            .short("s")
            .long("silent")
            .help("Dont show any notifications. Silent/quiet mode.")
            .takes_value(false))
		.get_matches();
	matches
}
