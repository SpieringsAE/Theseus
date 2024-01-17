#![no_std]

#[macro_use] extern crate app_io;

#[macro_use] extern crate alloc;

use alloc::{
	vec::Vec,
	string::String,
};
use path::Path;

pub fn main(args: Vec<String>) -> isize {
	if !args.is_empty() {
		let path: &Path = args[0].as_ref();
		//create directories/files as the command argument specifies
		if path.is_absolute() {
			todo!("search root");
		} else {
			let Ok(cwd) = task::with_current_task(|t| t.get_env().lock().working_dir.lock().get_absolute_path().clone()) else {
				println!("failed to get current task");
				return -1;
			};
			match path.relative(cwd.as_str()) {
				Some(path) => {
					if path.is_file() {
						todo!("load into buffer");
					} else if path .is_dir() {
						todo!("at some point open file selector for now error?");
					}
				},
				None => {
					println!("something is wrong with the given path");
				}
			}
		}
	} else {
		todo!("emtpy buffer.");
	}
	return 0;
}

pub enum EditorMode {
	Normal,
	Visual,
	Insert,
}

pub struct Editor {
	mode: EditorMode,
	cursor: (usize, usize),
	buffer: Vec<Vec<char>>, 
}