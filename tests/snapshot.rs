#[test]
fn snapshot() {
    use abilists::AbiList;

    insta::assert_debug_snapshot!(AbiList::from_bytes(include_bytes!(
        "../deps/glibc-abi-tool/abilists"
    )));
}
