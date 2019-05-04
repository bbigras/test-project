fn something() -> String {
    "test".to_string()
}

#[test]
fn test_something() {
    assert_eq!(something(), "test".to_string());
}

fn main() {
    let _r = something();
}
