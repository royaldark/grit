use helpers;
use std::fs;

fn build_objects_directory() {
    fs::create_dir(&*helpers::OBJECTS_DIR).expect("Failed to create objects dir");
    fs::create_dir(&format!("{}/info", *helpers::OBJECTS_DIR)).unwrap();
    fs::create_dir(&format!("{}/pack", *helpers::OBJECTS_DIR)).unwrap();
}

fn build_refs_directory() {
    fs::create_dir(&*helpers::REFS_DIR).expect("Failed to create refs dir");
    fs::create_dir(&format!("{}/heads", *helpers::REFS_DIR)).unwrap();
    fs::create_dir(&format!("{}/tags", *helpers::REFS_DIR)).unwrap();
}

fn initialize_head() {
    fs::write(
        &format!("{}/HEAD", *helpers::GIT_DIR),
        b"ref: refs/heads/master\n",
    ).expect("Failed to init HEAD");
}

pub fn initialize_repo() {
    if helpers::in_repo() {
        println!(".git already exists");
        ::std::process::exit(1);
    }

    fs::create_dir(&*helpers::GIT_DIR).expect("Failed to create .git");

    build_objects_directory();
    build_refs_directory();
    initialize_head();
}
