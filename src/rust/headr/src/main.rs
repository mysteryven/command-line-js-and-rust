use std;

fn main() {
    if let Err(e) = headr::get_args() {
        eprint!(e);
        std::process::exit(1)
    }
}
