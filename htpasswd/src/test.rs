use super::*;

const TEST_FILE: &str = include_str!("htpasswd.txt");

#[test]
fn parse_file_test() {
    let res = parse(TEST_FILE).unwrap();
    assert_eq!(
        res["bcryptuser"],
        HashedPassword::Bcrypt(
            "$2y$05$4hlGFwyiqrMxB4XS9.0nLeaKvU40nNmyv73UkrQmW8sUn9hdoa99.".into()
        )
    );
    assert_eq!(res["md5user"], HashedPassword::Md5("$apr1$Ca0XKxvm$IAmDo2yNnJysSXp8jcw7I0".into()));
    assert_eq!(res["shauser"], HashedPassword::Sha("{SHA}qUqP5cyxm6YcTAhz05Hph5gvu9M=".into()));
    assert_eq!(res["wtfisthis"], HashedPassword::Unknown("jsajhgoryg".into()));
}

#[test]
fn verify_bcrypt_test() {
    let res = parse(TEST_FILE).unwrap();
    let bc_hash = &res["bcryptuser"];
    let suc = bc_hash.verify("test");
    assert!(suc.is_ok());
    let fail = bc_hash.verify("test2");
    assert!(fail.is_err());
}
