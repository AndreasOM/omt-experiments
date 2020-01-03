use crate::OmError;
use image::{ DynamicImage, ImageBuffer, ImageFormat, GenericImage, GenericImageView };
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone)]
struct Entry{
	filename:	String,
	image:		Option<DynamicImage>,
	x:			u32,
	y:			u32,
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
			filename: 	filename.to_string(),
			image: 		None,
			x:			0,
			y:			0,
			width: 		width,
			height: 	height,
		}
	}

	fn set_image( &mut self, image: DynamicImage ) {
		self.width  = image.dimensions().0;
		self.height = image.dimensions().1;
		self.image = Some( image );
	}

	fn set_position( &mut self, x: u32, y: u32 ) {
		self.x = x;
		self.y = y;
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
#[derive(Debug)]
struct Row {
	y: u32,			// start of row
	width: u32,
	height: u32,
	end_x: u32, 	// current end of row
}

impl Row {
	fn new( y: u32, width: u32, height: u32 ) -> Row {
		Row {
			y: y,
			width: width,
			height: height,
			end_x: 0
		}
	}

	fn would_fit( &self, w: u32, h: u32 ) -> bool {
		if self.height >= h {
			let available_space = self.width - self.end_x;
			if available_space >= w {
				true
			} else {
				// not enough space
				println!("Row {:?} not enough space for {:?}", self, w );
				false
			}

		} else {
			// not high enough
			println!("Row {:?} not high enough for {:?}", self, h );
			false
		}
	}
}


#[derive()]
pub struct Atlas {
	size: u32,
	border: u32,
	entries: Vec<Entry>,
	image: Option<DynamicImage>,
	rows: Vec<Row>,
	used_height: u32,
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
			rows: Vec::new(),
			used_height: 0,
		}
	}
	/* would prefer pass through with ownership transfer and return, but need more rust knowledge
	fn add_entry( &mut self, entry: &Entry ) -> Result<usize, Entry> {
		Err( *entry )
	*/

	fn blit( dest: &mut image::DynamicImage, source: &image::DynamicImage, start_x: u32, start_y: u32 ) {
		let w = source.dimensions().0;
		let h = source.dimensions().1;

		for y in 0..h {
			for x in 0..w {
				let dx = start_x + x;
				let dy = start_y + y;

				let pixel = image::Rgba( [255, x as u8, y as u8, 255] );
				let pixel = source.get_pixel( x, y );
				dest.put_pixel( dx, dy, pixel );
			}
		}
	}
	fn add_row( &mut self, height: u32 ) -> Option<usize> {
		if height <= ( self.size - self.used_height ) {
			let row = Row::new( self.used_height, self.size, height );
			self.used_height += height;
			let row_index = self.rows.len();
			println!("Created row #{:?} at {:?}. {:?} used now.", row_index, row.y, self.used_height );
			self.rows.push( row );
			Some( row_index )
		} else {
			println!("Can not create row with {:?} height, {:?} used of {:?}", height, self.used_height, self.size );
			None
		}
	}
	fn add_entry_to_row_with_index( &mut self, entry: &Entry, row_index: usize ) -> bool {
		match self.rows.get_mut( row_index ) {
			None => false,	// give up, should never happen
			Some( row ) => {
				println!("Got row {:?}", row );
				if row.would_fit( entry.width, entry.height  ) {
					// add it
					let mut e = entry.clone();
					// blitting
					let x = row.end_x;
					let y = row.y;
					match &mut self.image {
						None => {},
						Some( di ) => {
							Atlas::blit( di, &e.image.unwrap(), x, y );
						},
					}
					row.end_x += e.width;
					e.image = None;	// cleanup, data not needed anymore
					e.set_position( x, y );
					self.entries.push(
						e
					);
					true
				} else {
					println!("Row {:?} would not fit {:?}", row, entry );
					false
				}
			}
		}
	}
	fn add_entry( &mut self, entry: &Entry ) -> bool {
		let h = entry.height;

		if( self.size < entry.width || self.size < entry.height ) {
			false
		} else {
			// find row
			let mut candidates = Vec::new();

			for ri in 0..self.rows.len() {
				let r = &self.rows[ ri ];
				if r.would_fit( entry.width, entry.height ) {
					println!("Row {:?} would fit {:?}", r, entry );
					if r.height < 2*entry.height {	// do not waste too much space, "2" is purely guessed
						candidates.push( ri );
					}
				}
			}

			if candidates.len() > 0 {
				// find best candidate
				let mut best_candidate_index = 0;	// :TODO: actually find best candidate
				/*
				for ci in 0..candidates.len() {
					//
				}
				*/
				println!("Got candidate rows. Using best one {:?}", candidates[ best_candidate_index ] );
				self.add_entry_to_row_with_index( entry, candidates[ best_candidate_index ] )
			} else {
				// or create new row
				println!("No candidate row found creating new one. {:?}", self);
				match self.add_row( h ) {
					None				=> false,													// give up
					Some( row_index )	=> self.add_entry_to_row_with_index( entry, row_index ),
				}
			}
		} 
	}

	fn save_png( &self, filename: &str ) -> Result< u32, OmError > {
//		Err( OmError::NotImplemented("Atlas::save_png".to_string()))
		match self.image.as_ref().unwrap().save_with_format(filename, ImageFormat::PNG) {
			_ => Ok( 0 )
		}
	}

	fn save_atlas( &self, filename: &str ) -> Result< u32, OmError > {
		let f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};

		Ok( 0 )
	}

	fn save_map( &self, filename: &str ) -> Result< u32, OmError > {
		let mut f = match File::create(filename) {
			Ok( f ) => f,
			Err( _ ) => return Err(OmError::Generic("io".to_string())),
		};

//		println!("{:?}", self );

		for e in &self.entries {
//			println!("{:?}", e );
			// overlay-00-title-square.png:0,0-2048,1536
			let basename = Path::new(&e.filename).file_name().unwrap().to_str().unwrap();
			let l = format!("{}:{},{}-{},{}\n", basename, e.x, e.y, e.x+e.width, e.y+e.height);
//			println!("{}", l);
			write!( f, "{}", l );
		}
		Ok( 0 )
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
					println!("‼️ Image doesn't fit into empty atlas {:?}", e );
					return Err(OmError::Generic("‼️ Image doesn't fit into empty atlas".to_string()));
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
			let atlasname = format!("{}.atlas", outname );
			let mapname = format!("{}.map", outname );
			match a.save_png( &pngname ) {
				Ok( bytes_written ) => {
					println!("{:?} bytes written to image {}", bytes_written, pngname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			match a.save_atlas( &atlasname ) {
				Ok( bytes_written ) => {
					println!("{:?} bytes written to atlas {}", bytes_written, atlasname );
				},
				Err( e ) => {
					return Err( e );
				}
			}
			match a.save_map( &mapname ) {
				Ok( bytes_written ) => {
					println!("{:?} bytes written to map {}", bytes_written, atlasname );
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
