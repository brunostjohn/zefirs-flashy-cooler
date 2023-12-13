use super::*;

#[test]
fn test_computer() {
    let mut computer = Computer::new();
    let params = computer.get_params();
    println!("{:#?}", params);
    panic!();
}
