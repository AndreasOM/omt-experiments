
use clap::{Arg, App, SubCommand};
use glob::glob;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
enum ParameterValue {
	NoValue,
	IntegerValue(i64),
	StringValue(String),
}

impl fmt::Display for ParameterValue {
	fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
		match self {
			ParameterValue::NoValue => write!(f, "NOVALUE"),
			ParameterValue::IntegerValue( i ) => write!(f, "{}", i),
			ParameterValue::StringValue( s ) => write!(f, "\"{}\"", s ),

		}
//		write!(f, "FUU")
	}
}

struct ToolRun {
	tool: String,
	command: String,
	output: String,
	input: Vec<String>,
	parameters: HashMap<String,ParameterValue>,
	cmd_line: String,
}

impl ToolRun {
	fn new(
		tool: &str,
		command: &str,
		output: &str,
		input: &Vec<String>,
		parameters: &HashMap<String,ParameterValue>,
		cmd_line: &str,
	) -> ToolRun
	{
		ToolRun {
			tool: tool.to_string(),
			command: command.to_string(),
			output: output.to_string(),
			input: input.clone(),
			parameters: parameters.clone(),
			cmd_line: cmd_line.to_string(),
		}
	}
}

struct Asset{
}

impl Asset{

	fn tool_asset(
		tool_run: &ToolRun,
	)
	-> Result<u32,&'static str> {
		match tool_run.command.as_ref() {
			"" => {
				println!("NO command for asset tool" );
				Ok(0)	// :TODO: return error
			},
			"dump" => {
				println!("command.  : {:?}", tool_run.command );
				println!("output    : {:?}", tool_run.output );
				println!("input     : {:?}", tool_run.input );
				println!("parameters: {:?}", tool_run.parameters );
				Ok(0)
			},
			cmd => {
				println!("Unhandled asset tool command: {:?}", cmd );
				Ok(0)	// :TODO: return error
			},
		}
	}

	fn tool_call_external(
		tool_run: &ToolRun,
	)
	-> Result<u32,&'static str> {
		let cmd_line = tool_run.cmd_line.clone();
		let re = Regex::new(r"\$\{(.*?)\}").unwrap();

		let cmd_line = re.replace_all(
			&cmd_line,
			|c: &regex::Captures| {
				let placeholder = c.get(1).map_or( "", |m| m.as_str() );
				println!("Found {:?}", placeholder );
				match placeholder {
					"" => "".to_string(),
					"tool" => tool_run.tool.clone(),
					"command" => tool_run.command.clone(),
					"output" => format!("\"{}\"", tool_run.output).clone(),
					"input" => {
						tool_run.input.iter().map( |s|
							format!("\t\"{}\"", s).clone()
						).collect::<Vec<_>>().join(" ").to_string()
					},
					param => {
						println!("{:?}", tool_run.parameters.get( param ) );
						tool_run.parameters.get( param ).unwrap_or( &ParameterValue::NoValue ).to_string()
					},
				}
			}
		);
		println!("Calling\n{}", cmd_line );
//		let output = Command::new("/bin/sh").args(&["-c", "echo", ""]).output();
//		let output = Command::new("/bin/sh").args(&["-c", "date", ""]).output();
		let output = Command::new("/bin/sh").args(&["-c", &cmd_line]).output();
		println!("Output: {:?}", output );
		Ok(0)
	}


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
//			println!("===\n{:?}", config_file );
			let mut file = File::open( config_file ).expect( "Failed opening file" );
			let mut config = String::new();
			file.read_to_string(&mut config).expect( "Failed reading file" );
			let yaml = YamlLoader::load_from_str(&config).unwrap();

			// parse yaml
//			println!("YAML: {:?}", yaml );
			for doc in yaml {
//				println!("---");
				let tool = doc["tool"].as_str().unwrap_or("");
				let command = doc["command"].as_str().unwrap_or("");
				let output = doc["output"].as_str().unwrap_or("");
				let cmd_line = doc["cmd_line"].as_str().unwrap_or("");
				let mut input = Vec::new();

				if doc["input"].is_array() {
					match doc["input"].as_vec() {
						None => {},
						Some(i) => {
//							println!("i: {:?}", i );
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

				let mut parameters = HashMap::new();

				match doc["parameters"].as_hash() {
					None => {},
					Some(params) => {
						for (name, value) in params {
//							println!("name: {:?} -> {:?}", name, value );
							let value = match value {
								Yaml::Integer( v ) => ParameterValue::IntegerValue( *v ),
								Yaml::String( v ) => ParameterValue::StringValue( v.clone() ),
								x => {
									println!("Unhandled parameter value {:?}", x );
									ParameterValue::NoValue
								}
							};
							let name = match name.as_str() {
								Some( s ) => s.to_string(),
								x => { println!("Unhandled name type {:?}", x ); "".to_string() },
							};
							parameters.insert( name, value );
						}
					}
				};
/*
				println!("tool      : {:?}", tool );
				println!("command.  : {:?}", command );
				println!("output    : {:?}", output );
				println!("input     : {:?}", input );
				println!("parameters: {:?}", parameters );
*/
				let tool_run = ToolRun::new( &tool, &command, &output, &input, &parameters, &cmd_line );
				// call tool
				match tool {
					""			=> continue,
					"noop"		=> println!("NOOP -> Do nothing"),
					"$asset"	=> {
						println!("$asset command found");
						Asset::tool_asset( &tool_run );
					}
					"echo"		=> {
						Asset::tool_call_external( &tool_run );
					}
					tool		=> {
						Asset::tool_call_external( &tool_run );						
//					println!("Command {:?} not implemented", tool ),
					},
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
