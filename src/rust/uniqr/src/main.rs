
fn main() {
    if let Err(e) = uniqr::get_args().and_then(uniqr::run) {
        eprint!("{}", e);
        std::process::exit(1)
    }
}
