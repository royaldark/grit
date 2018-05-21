use std::env;

lazy_static! {
    pub static ref GIT_DIR: String = String::from(".git");
    pub static ref OBJECTS_DIR: String = format!("{}/objects", *GIT_DIR);
    pub static ref REFS_DIR: String = format!("{}/refs", *GIT_DIR);
}

pub fn in_repo() -> bool {
    let mut cur_dir = env::current_dir().unwrap();

    loop {
        let mut path = cur_dir.clone();
        path.push(".git");

        if path.exists() {
            return true;
        } else {
            match cur_dir.clone().parent() {
                None => return false,
                Some(parent) => cur_dir = parent.to_path_buf(),
            }
        }
    }
}

pub fn assert_in_repo() {
    if !in_repo() {
        println!("not in git directory");
        ::std::process::exit(128);
    }
}
