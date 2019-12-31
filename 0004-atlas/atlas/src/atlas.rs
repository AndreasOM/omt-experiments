use crate::OmError;
use image::{ DynamicImage, ImageBuffer, ImageFormat, GenericImage, GenericImageView };
use regex::Regex;

#[derive(Clone)]
struct Entry{
	filename:	String,
	image:		Option<DynamicImage>,
	width:		u32,
	height:		u32,
}

impl std::fmt::Debug for Entry {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
		f.debug_struct("Entry")
			.field("filename", &self.filename)
//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}
impl Entry {
	fn new( filename: &str, width: u32, height: u32 ) -> Entry {
		Entry{
			filename: filename.to_string(),
			image: None,
			width: width,
			height: height,
		}
	}

	fn set_image( &mut self, image: DynamicImage ) {
		self.width  = image.dimensions().0;
		self.height = image.dimensions().1;
		self.image = Some( image );
	}
}

fn simple_format_u32( f: &str, n: u32 ) -> String {
	let s = f.clone();
	let re = Regex::new(r"(%d)").unwrap();

//	println!("simple_format_u32 {:?} with {:?}", s, re );
	let s = re.replace_all(
		&s,
		|c: &regex::Captures| {
			let placeholder = c.get(1).map_or( "", |m| m.as_str() );
//			println!("Found {:?}", placeholder );
			match placeholder {
				"" => "".to_string(),
				"%d" => n.to_string(),
				x => {
					println!("simple_format_u32 got {:?}", x);
					x.to_string()
				},
			}
		}
	);

	s.to_string()
}

#[derive()]
pub struct Atlas {
	size: u32,
	border: u32,
	entries: Vec<Entry>,
	image: Option<DynamicImage>,
}

impl std::fmt::Debug for Atlas {
	fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::result::Result<(), std::fmt::Error > {
		f.debug_struct("Atlas")
			.field("size", &self.size)
			.field("border", &self.border)
			.field("entries", &self.entries)
//			.field("image", if self.image.is_some() {"YES"} else {"NO"} )
			.finish()
	}
}

impl Atlas {
	fn new( size: u32, border: u32 ) -> Atlas {
		Atlas {
			size: size,
			border: border,
			entries: Vec::new(),
			image: Some( image::DynamicImage::new_rgba8(size, size) ),
		}
	}
	/* would prefer pass through with ownership transfer and return, but need more rust knowledge
	fn add_entry( &mut self, entry: &Entry ) -> Result<usize, Entry> {
		Err( *entry )
	*/

	fn add_entry( &mut self, entry: &Entry ) -> bool {
		// check if it fits
		// else return false
		if self.entries.len() > 0 {
			false 
		} else if( self.size < entry.width || self.size < entry.height ) {
			false
		} else {
			// add it
			let mut e = entry.clone();
			// blitting
			let w = e.width;
			let h = e.height;
			let start_x = 0;
			let start_y = 0;
			let si = e.image.unwrap();

			for y in 0..h {
				for x in 0..w {
					let dx = start_x + x;
					let dy = start_y + y;

					let pixel = image::Rgba( [255, x as u8, y as u8, 255] );
					let pixel = si.get_pixel( x, y );
					// :TODO: move unwrapping outside of loop
					match &mut self.image {
						None => {},
						Some( di ) => {
							di.put_pixel( dx, dy, pixel );
						}
					};
				}
			}
			e.image = None;	// cleanup, data not needed anymore
			self.entries.push(
				e
			);
			true
		}
	}

	fn save_png( &self, filename: &str ) -> Result< u32, OmError > {
//		Err( OmError::NotImplemented("Atlas::save_png".to_string()))
		match self.image.as_ref().unwrap().save_with_format(filename, ImageFormat::PNG) {
			_ => Ok( 0 )
		}
	}

	pub fn hello() {
		println!("Atlas::hello()");
	}
	pub fn combine(
		output: &str, size: u32, border: u32, input: &Vec<&str> 
	) -> Result<u32, OmError>{
		println!("Atlas::combine()");
		let mut entries = Vec::new();
		// collect inputs
		for i in input {
			println!("Analysing {:?}", i );
			let img = image::open(i).unwrap();
			println!("dimensions {:?}", img.dimensions());

			let mut e = Entry::new( i, 0, 0 );
			e.set_image( img );
			entries.push(e);
		}

		// sort entries by size

		let mut atlases: Vec<Atlas> = Vec::new();

		// combine outputs
		for e in entries.drain(..) {
			let mut did_fit = false;
			for a in &mut atlases {
				if a.add_entry( &e) {
					did_fit = true;
					break;
				}
			}
			if !did_fit {
				let mut a = Atlas::new( size, border );
				if !a.add_entry( &e ) {
					println!("Image doesn't fit into empty atlas {:?}", e );
					return Err(OmError::Generic("Image doesn't fit into empty atlas".to_string()));
				}
				atlases.push( a );				
			}
		}

		// write outputs
		let mut n = 0;
		for a in atlases {
			println!("Atlas #{} {:?}", n, a );
			let outname = simple_format_u32( output, n ); //format!(output, n);
			let pngname = format!("{}.png", outname );
			println!("{:?}", pngname);
			match a.save_png( &pngname ) {
				Ok( bytes_written ) => {
					println!("{:?} bytes written to {}", bytes_written, pngname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			n += 1;
		}

		Err(OmError::NotImplemented("".to_string()))
//		Ok(0)
	}
}
