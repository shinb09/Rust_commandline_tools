fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
