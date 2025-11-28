mod smartcard;
mod scripts;

fn main() {
    env_logger::init();
    scripts::send_ppse::run().unwrap();
}
