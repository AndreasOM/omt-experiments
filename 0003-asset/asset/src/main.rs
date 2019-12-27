
use clap::{Arg, App, SubCommand};
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use yaml_rust::YamlLoader;

struct Asset{

}

impl Asset{
	fn build (
		content_directory: &String,
		data_directory: &String,
		temp_directory: &String,
		archive: &String,
		paklist: &String
	)
	-> Result<u32,&'static str> {

		// find all asset_config.yaml
		let mut config_files = Vec::new();
		let config_glob = format!( "{}/**/*.asset_config.yaml", content_directory );
		for config_file in glob( &config_glob ).expect("Failed glob pattern") {
			match config_file {
				Err(e) => return Err( "Error finding config" ),
				Ok(config_file) => {
//					println!("Config file: {:?}", config_file );
					config_files.push( config_file );
				},
			}
		}
		println!("Found {:?} config files", config_files.len() );

		for config_file in config_files {
			// read yaml
			println!("===\n{:?}", config_file );
			let mut file = File::open( config_file ).expect( "Failed opening file" );
			let mut config = String::new();
			file.read_to_string(&mut config).expect( "Failed reading file" );
			let yaml = YamlLoader::load_from_str(&config).unwrap();

			// parse yaml
//			println!("YAML: {:?}", yaml );
			for doc in yaml {
				println!("---");
				let tool = doc["tool"].as_str();
				let command = doc["command"].as_str();
				let output = doc["output"].as_str();
				let mut input = Vec::new();

				if( doc["input"].is_array()){
					match doc["input"].as_vec() {
						None => {},
						Some(i) => {
							println!("i: {:?}", i );
							for i in i {
								match i.as_str() {
									None => {},
									Some(s) => input.push( s.to_string() )
								}
							}
						},
					}
				} else {
					let i = doc["input"].as_str();
					match i {
						Some(i) => input.push( i.to_string() ),
						None => {},
					};
				}
//				let input = doc["input"].as_str();

				println!("tool   : {:?}", tool );
				println!("command: {:?}", command );
				println!("output : {:?}", output );
				println!("input  : {:?}", input );

				// call tool
				match tool {
					Some("noop") => println!("NOOP -> Do nothing"),
					Some( tool ) => println!("Command {:?} not implemented", tool ),
					None => continue,
				}
			}
		}

		Ok(0)
	}
}

fn main() {
// omt-asset build --content-directory Content --temp-directory Temp --data-directory Data --archive App/data/base.omar --paklist Data/data.paklist

	let matches = App::new("omt-asset")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Handles assets")
					.subcommand(SubCommand::with_name("build")
						.arg(Arg::with_name("content-directory")
							.long("content-directory")
							.value_name("CONTENT-DIRECTORY")
							.help("Set the content directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("data-directory")
							.long("data-directory")
							.value_name("DATA-DIRECTORY")
							.help("Set the data directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("temp-directory")
							.long("temp-directory")
							.value_name("TEMP-DIRECTORY")
							.help("Set the temp directory")
							.takes_value(true)
						)
						.arg(Arg::with_name("archive")
							.long("archive")
							.value_name("archive")
							.help("Set the archive filename")
							.takes_value(true)
						)
						.arg(Arg::with_name("paklist")
							.long("paklist")
							.value_name("PAKLIST")
							.help("Set the pakelist name")
							.takes_value(true)
						)
					)
					.get_matches();

//	println!("{:?}", matches);
//	println!("{:?}", matches.subcommand());

	if let ("build", Some( sub_matches ) ) = matches.subcommand() {
		let content_directory = sub_matches.value_of("content-directory").unwrap_or(".").to_string();
		let data_directory = sub_matches.value_of("data-directory").unwrap_or(".").to_string();
		let temp_directory = sub_matches.value_of("temp-directory").unwrap_or(".").to_string();
		let archive = sub_matches.value_of("archive").unwrap_or("out.omar").to_string();
		let paklist = sub_matches.value_of("paklist").unwrap_or("").to_string();

		println!("content_directory: {:?}", content_directory );
		println!("data_directory   : {:?}", data_directory );
		println!("temp_directory   : {:?}", temp_directory );
		println!("archive          : {:?}", archive );
		println!("paklist          : {:?}", paklist );

		match Asset::build(
			&content_directory,
			&data_directory,
			&temp_directory,
			&archive,
			&paklist
		) {
			Ok( number_of_files ) => {
					println!("{:?} assets build", number_of_files );
					process::exit( 0 );
				},
			Err( e ) => {
				println!("Error {:?}", e );
				process::exit( -1 );
			},
		}
	}
}
