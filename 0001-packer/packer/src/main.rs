
extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("omt-packer")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com>")
					.about("Packs data into archive")
					.arg(Arg::with_name("basedir")
						.long("basedir")
						.value_name("BASEDIR")
						.help("Set the basedirectory (for relative names)")
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

	let basedir = matches.value_of("basedir").unwrap_or(".");
	let output = matches.value_of("output").unwrap_or("out.omar");
	let paklist = matches.value_of("paklist").unwrap_or("");


	println!("basedir: {:?}", basedir );
	println!("output : {:?}", output );
	println!("paklist: {:?}", paklist );
}
