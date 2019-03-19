use spooky::config_file;
use std::io::ErrorKind;

#[test]
fn find_config_file() {
    let base_dir = std::env::current_dir()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join("tests")
        .join("find_config_file");

    dbg!(&base_dir);

    let test_with_dir = |sub_dirs: &[&str]| {
        let mut dir = base_dir.to_path_buf();

        for sub_dir in sub_dirs {
            dir.push(sub_dir);
        }

        std::env::set_current_dir(&dir).unwrap();
        let actual_config_dir = config_file::find_config_dir(&dir).unwrap();
        assert_eq!(actual_config_dir, base_dir);
    };

    test_with_dir(&["."]);
    test_with_dir(&["sub1"]);
    test_with_dir(&["sub1", "sub1_1"]);
    test_with_dir(&["sub1", "sub1_1", "sub1_1_1"]);
    test_with_dir(&["sub2"]);
    test_with_dir(&["sub2", "sub2_1"]);
    test_with_dir(&["sub2", "sub2_2"]);
}

#[test]
fn find_config_file_fail() {
    match config_file::find_config_dir(&std::env::current_dir().unwrap()) {
        Err(ref e) => {
            if e.kind() != ErrorKind::NotFound {
                panic!("Unexpected error: {}", e)
            }
        }
        Ok(path) => panic!("Unexpectedly found config dir: {}", path.display()),
    }
}
