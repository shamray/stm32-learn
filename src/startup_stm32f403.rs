#[unsafe(no_mangle)]
fn reset_handler() {
    crate::main();
}
