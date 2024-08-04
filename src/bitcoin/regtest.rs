// Starts Regtest Node providing a temp Config
// Every Level Setup Includes running a Clean Regtest Node

use anyhow::{Ok, Result};
use bitcoind::{BitcoinD, Conf};

// might need state later!!
pub struct CtfFramework {
    pub bitcoind: BitcoinD,
}

impl CtfFramework {
    pub fn new() -> Result<Self> {
        let mut conf = Conf::default();
        conf.staticdir = Some("bin/bitcoin/static".into());

        let key = "BITCOIND_EXE";
        let curr_dir_path = std::env::current_dir().unwrap();
        let bitcoind_path = curr_dir_path.join("bin").join("bitcoind");
        std::env::set_var(key, bitcoind_path);
        let exe_path = bitcoind::exe_path().unwrap();
        let bitcoind = BitcoinD::with_conf(exe_path, &conf).unwrap();
        Ok(CtfFramework { bitcoind })
    }

    pub fn clean() -> Result<()> {
        // deletes bin/bitcoin/static directory
        if !std::path::Path::new("bin/bitcoin/static").exists() {
            return Ok(());
        }
        Ok(std::fs::remove_dir_all("bin/bitcoin/static")?)
    }
}
