use photos::Repository;
use photos::Scanner;
use std::path::PathBuf;
use tempfile;

fn picture_dir() -> PathBuf {
    let mut test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data_dir.push("resources/test");
    test_data_dir
}

#[test]
fn test_scan_and_persist() {
    let mut db_path = tempfile::tempdir().unwrap().into_path();
    db_path.push("test.sqlite");

    let repo = Repository::build(&db_path).unwrap();

    let pic_dir = picture_dir();
    let scanner = Scanner::build(&pic_dir).unwrap();

    scanner.visit_all(|pic| {
        repo.add(&pic).ok();
    });
}
