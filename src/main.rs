use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn rcat(file_name : &str) -> Result<(), io::Error> {
	// We can just assume that the file exists and have try! take care of the rest.

	// We cannot use the try! macro as it will return Errs automatically but main does not
	// return anything, which will just produce a (misleading) error message. Therefore this
	// code is contained in a separate function.
	//
	// See also http://stackoverflow.com/questions/30555477/try-does-not-compile
	let f = try!(File::open(file_name));
	let reader = BufReader::new(f);

	for line in reader.lines() {
		println!("{}", line.unwrap());
	}
	Ok(())
}

fn main() {
	// This is not used anymore and just here for documentatory purposes - see more details in
	// the if branch below.

	//let args : Args = env::args();
	let num_args = env::args().count();

	if num_args != 2 {
		// rebind as mutable otherwise the next line would not work (once again - just here for
		// documentatory purposes).
		//let mut args = args;

		//Note how we call env::args every time we need it as other methods
		// (such as #count()) consume the iterator provided by env::args and
		// we'd get a non-obvious error on the second invocation:
		//
		// `args` moved here because it has type `std::env::Args`, which is non-copyable

		// args[0] is always present
		let executable_name = env::args().nth(0).unwrap();
		println!("Usage: {}, input file name", executable_name);
		// EX_USAGE (see sysexits.h)
		std::process::exit(64);
	} else {
		// as we have previously checked the number of arguments this is safe to unwrap
		let file_name = env::args().nth(1).unwrap();
		match rcat(&file_name) {
			Ok(_) => (),
			Err(e) => {
				println!("Could not display '{}'. {}", file_name, e);
				// EX_IOERR (see sysexits.h)
				std::process::exit(74);
			}
		}
    }
}
