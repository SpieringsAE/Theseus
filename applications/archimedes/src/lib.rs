#![no_std]

#[macro_use] extern crate app_io;

#[macro_use] extern crate alloc;

use alloc::{
	vec::Vec,
	string::String, borrow::ToOwned,
};
use path::Path;
use fs_node::FileOrDir;

pub fn main(args: Vec<String>) -> isize {
    if !args.is_empty() {
		//create directories/files as the command argument specifies
		if args[0].starts_with("/")	{
			todo!("search root");
		} else {
			let Ok(cwd) = task::with_current_task(|t| t.get_env().lock().working_dir.clone()) else {
				println!("failed to get current task");
				return -1;
			};
			let path: &Path = args[0].as_ref();
			match path.get(&cwd) {
				Some(file_dir_enum) => {
					match file_dir_enum {
						FileOrDir::File(File) => {
							todo!("load into buffer");
						},
						FileOrDir::Dir(dir) => {
							todo!("at some point open file selector for now error?");
						}
					}
				},
				None => {
					todo!("create new empty buffer, maybe validate if its a valid path, create file at save");
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