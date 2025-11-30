#![allow(dead_code)]

mod emv;
mod hex;
mod pcsc;
mod scripts;
mod smartcard;
mod tlv;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    scripts::send_ppse::run()?;
    // scripts::read_emv_card::run();

    Ok(())
}
