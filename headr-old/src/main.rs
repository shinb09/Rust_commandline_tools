fn main() {
    if let Err(e) = headr_old::get_args().and_then(headr_old::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
