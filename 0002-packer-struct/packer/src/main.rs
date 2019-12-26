
use byteorder::{LittleEndian, WriteBytesExt};

extern crate clap;
use clap::{Arg, App};

use crc::{crc32, Hasher32};

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::process;
use std::string::String;

#[derive(Debug)]
struct Entry {
	basepath:String,
	filename:String,
	crc:u32,
	size:u32,
}

impl Entry {
	fn create(basepath:&String, filename:&String) -> Entry {

		let fullfilename = format!( "{}/{}", basepath, filename );

		 // :TODO: better error handling
		let size = match fs::metadata( fullfilename ) {
			Ok( metadata ) => metadata.len() as u32,
			Err( _ ) => 0,
		};

		// :TODO: calculate actual CRC name
		let downcase_name = filename.to_lowercase();
		// Ruby: .gsub( /\W\./, ' ' ) // should be 'a-zA-Z0-9_', but actual code behaves differently
		let clean_name: String = downcase_name.chars().map(|c| match c {
			'0'..='9' => c,
			'a'..='z' => c,
	//			'A'..='Z' => c,	// already downcase
			'!'..='@' => c,
			'['..='`' => c,
			'{'..='~' => c,
	//		0x7f => c,			// ignore DEL
			_ => ' '
		}).collect();
		let crc = crc32::checksum_ieee(clean_name.as_bytes());
		println!("CRC: {:?} -> {:?} crc: {:?}\n", filename, clean_name, crc );
//	      puts "CRC: " + filename + " -> " + name + " crc: " + @crc.to_s


		Entry {
			basepath: basepath.to_string(),
			filename: filename.to_string(),
			crc: crc,
			size: size,
		}
	}

	fn display(&self) {
		println!("Displaying Entry for filename {:?}", self.filename );
		print!("{:?}\n", self);
	}
}

#[derive(Debug)]
struct Archive {
	basepath: String,
	entries: Vec<Entry>
}

impl Archive {
	fn create(basepath:&String) -> Archive {
		Archive {
			basepath: basepath.clone(),
			entries: Vec::new(),
		}
	}
	fn add_entry(&mut self, filename:&String) -> bool {
		let entry = Entry::create(
			&self.basepath,
			&filename,
		);

		self.entries.push(entry);
		true
	}

	fn save(&self, output: &String) -> Result<u32,&'static str> {
		// write output
		let output_file = File::create(output);
		// :TODO: rethink error handling
		let mut output_file = match output_file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error writing file"),
		};

		// :TODO: add error handling
		let write_names = false;
		let mut flags: u8 = 0;
		if write_names {
			flags |= 1
		}
		let number_of_files: u32 = self.entries.len() as u32;

		output_file.write_all(&[
			0x4f, 0x4d, 0x41, 0x52, 	// magic header
			2,							// version
			flags,						// flags
			0, 0,						// reserved
		]);
		output_file.write_u32::<LittleEndian>( number_of_files ).unwrap();

		// write the directory
		let mut pos = 0;
		for entry in &self.entries {
			// crc, pos, size all as LittleEndian u32
			output_file.write_u32::<LittleEndian>( entry.crc ).unwrap();
			output_file.write_u32::<LittleEndian>( pos ).unwrap();
			output_file.write_u32::<LittleEndian>( entry.size ).unwrap();

			pos += entry.size;
		}

		// write data to output

		for entry in &self.entries {

			let filename = format!( "{}/{}", self.basepath, entry.filename );
	//		println!("{:?}", filename );
			let data_file = File::open(filename);
			// :TODO: rethink error handling
			let mut data_file = match data_file {
				Ok( p ) => p,
				Err( _e ) => return Err("Error reading data file"),
			};
			let mut buffer = Vec::<u8>::new();
			data_file.read_to_end(&mut buffer);
			output_file.write_all( &buffer );
		}

		Ok(number_of_files)
	} 
}

struct Helper {

}

impl Helper {
	fn filenames_in_file(filename: &String) -> Result<Vec<String>, &'static str> {
		let file = File::open(filename);

		let file = match file {
			Ok( p ) => p,
			Err( _e ) => return Err("Error reading file"),
		};

		let bufreader = BufReader::new(file);

		let mut files: Vec<String> = Vec::new();
		for line in bufreader.lines() {
			let filename = line.unwrap();
			files.push( filename );
		}

		Ok(files)
	}
}

fn packer(
		basepath:&String,
		paklist:&String,
		output:&String,
) -> Result<u32,&'static str> {
	let mut archive = Archive::create(basepath);

	for filename in Helper::filenames_in_file(paklist).unwrap_or( Vec::new() ) {	// :TODO: add better error handling
		println!("{:?}", filename );
		archive.add_entry( &filename );		
	}

	archive.save( output )
}

fn main() {
	let matches = App::new("omt-packer")
					.version("0.2")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Packs data into archive")
					.arg(Arg::with_name("basepath")
						.long("basepath")
						.value_name("BASEPATH")
						.help("Set the base path (for relative names)")
						.takes_value(true)
					)
					.arg(Arg::with_name("output")
						.long("output")
						.value_name("OUTPUT")
						.help("Set the output filename")
						.takes_value(true)
					)
					.arg(Arg::with_name("paklist")
						.long("paklist")
						.value_name("PAKLIST")
						.help("Set the pakelist name")
						.takes_value(true)
					)
					.get_matches();

//	println!("{:?}", matches);

	let basepath = matches.value_of("basepath").unwrap_or(".").to_string();
	let output = matches.value_of("output").unwrap_or("out.omar").to_string();
	let paklist = matches.value_of("paklist").unwrap_or("").to_string();


	println!("basepath: {:?}", basepath );
	println!("output  : {:?}", output );
	println!("paklist : {:?}", paklist );

	match packer( &basepath, &paklist, &output ) {
		Ok( number_of_files ) => {
				println!("{:?} files added to archive", number_of_files );
				process::exit( 0 );
			},
		Err( e ) => {
			println!("Error {:?}", e );
			process::exit( -1 );
		},
	}
}
