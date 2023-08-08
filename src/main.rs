/*
MIT License

Copyright (c) 2023 Stoian Ivanov

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/


use std::io;
use std::env;
use std::io::BufRead;
use std::path::Path;
use std::fs::File;
use git2::{Repository,  ObjectType, Oid};
fn main() {

	let mut args=env::args_os();


	let arg0_osstr=args.nth(0).unwrap();
	let arg0fno_osstr=Path::new(&arg0_osstr).file_name().unwrap();
	let mut lockfile=String::from( arg0fno_osstr.to_str().unwrap());
	lockfile.push_str(".lock");
	let lockfile=lockfile;//freeze
	//println!("The lock file must be  {}",lockfile);

	let tgt_ref=match args.nth(0) {
		Some(val) => val.into_string().unwrap(),
		None =>{
			println!("we expect 3 params target_ref from_commit to_commit");
			std::process::exit(1);	
		}
	};

	let from_commit=match args.nth(0) {
		Some(val) => val.into_string().unwrap(),
		None =>{
			println!("we expect 3 params target_ref from_commit to_commit");
			std::process::exit(1);
		}
	};

	let from_commit_oid =match Oid::from_str(&from_commit) {
		Ok(oid) => oid,
		Err(e)=>{
			println!("from_comit id {} does not seem like a obj id! err:{}",from_commit,e);
			std::process::exit(1);
		}
	};
	let to_commit=match args.nth(0) {
		Some(val) => val.into_string().unwrap(),
		None =>{
			println!("we expect 3 params target_ref from_commit to_commit");
			std::process::exit(1);
		}
	};
	let to_commit_oid =match Oid::from_str(&to_commit) {
		Ok(oid) => oid,
		Err(e)=>{
			println!("to_commit id {} does not seem like a obj id! err:{}",to_commit,e);
			std::process::exit(1);
		}
	};

	let file = match File::open(&lockfile) {
		Ok(val) => val,
		Err(e) =>{
			println!("Lock file {} can't be opened! err:{}",&lockfile,e);
			std::process::exit(1);			
		}
	};
	let mut reader=io::BufReader::new(file);
	let mut locked_file=String::new();
	match reader.read_line(&mut locked_file){
		Ok(_)=>(),
		Err(e)=>{
			println!("Lock file {} can't provide a line?! err:{}",&lockfile,e);
			std::process::exit(1);
		}
	};
	let locked_file=locked_file.trim();
	println!("Target ref {} Commits: from {} to {} The locked file is {} via {}",tgt_ref, &from_commit, &to_commit, &locked_file,&lockfile);

	let repo=match Repository::open(".") {
		Ok(val) => val,
		Err(e) =>{
			println!("no repo at . Err {}",e);
			std::process::exit(1);
		}
	};


	//search from commit for locked file version

	let commit=match repo.find_commit(from_commit_oid) {
		Ok(val) => val,
		Err(e)=>{
			println!("from commit {} get err: {}", &from_commit,e);
			std::process::exit(1);
		}
	};

	let from_tree=match commit.tree() {
		Ok(val) => val,
		Err(e)=>{
			println!("from commit {} get tree err: {}", &from_commit,e);
			std::process::exit(1);
		}
	};

	let mut old_oid=Oid::zero();
	let mut found=false;
	for obj in from_tree.iter() {
		let typ: ObjectType=obj.kind().unwrap_or(ObjectType::Any);
		if typ!=ObjectType::Blob {
			continue;
		}
		let nm=match obj.name() {
			Some(val) =>val,
			None => continue
		};

		if nm!=locked_file {
			continue;
		}

		old_oid=obj.id();
		found=true; 
		break;
	}
	let old_oid=old_oid; //freeze
	let found =found; //freeze

	if !found {
		println!("Current state:   file {} not found!",locked_file);

	} else {
		println!("Current state:   file {} at version {} ",locked_file,old_oid);
	}


	//now we check the to commit

	let commit=match repo.find_commit(to_commit_oid) {
		Ok(val) => val,
		Err(e)=>{
			println!("commit {} get err: {}", &to_commit,e);
			std::process::exit(1);
		}
	};

	let from_tree=match commit.tree() {
		Ok(val) => val,
		Err(e)=>{
			println!("to commit {} get tree err: {}", &to_commit,e);
			std::process::exit(1);
		}
	};

	for obj in from_tree.iter() {
		let typ: ObjectType=obj.kind().unwrap_or(ObjectType::Any);
		if typ!=ObjectType::Blob {
			continue;
		};

		let nm=match obj.name() {
			Some(val) =>val,
			None =>continue
		};
		if nm!=locked_file {
			continue;
		};

		if found==false {
			println!("locked file {} was not found in from comit but is present in to comit! We don't allow this!",locked_file);
			std::process::exit(1);
		};

		
		let id=obj.id();
		if id==old_oid {
			println!("locked file does not change! Good!");
			std::process::exit(0);
		};
		println!("Incomming state: file {} at version {} We don't allow this!",locked_file,id);
		std::process::exit(1);
	}
	if found==false {
		println!("Incomming state: file {} still not found! Good!",locked_file);
		std::process::exit(0);
	}
	println!("Incomming state: file {} not found! We don't allow this!",locked_file);
	std::process::exit(1);

}
