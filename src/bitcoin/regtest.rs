// Starts Regtest Node providing a temp Config 
// Every Level Setup Includes running a Clean Regtest Node

use anyhow::{Ok, Result};
use bitcoind::{bitcoincore_rpc::{ Auth, Client}, BitcoinD, Conf};

// might need state later!!
pub struct CtfFramework {
    client : Client
}

impl<'a> CtfFramework {
    pub fn new() -> Result<Self> {
        let mut conf = Conf::default();
        conf.staticdir = Some("src/bitcoin/static".into());

        let key = "BITCOIND_EXE";
        let curr_dir_path = std::env::current_dir().unwrap();
        let bitcoind_path = curr_dir_path.join("bin").join("bitcoind");
        std::env::set_var(key, bitcoind_path);
        let exe_path = bitcoind::exe_path().unwrap();
        let bitcoind = BitcoinD::with_conf(exe_path, &conf).unwrap();

        Ok(CtfFramework {
            client: Client::new(bitcoind.rpc_url().as_str(), Auth::UserPass("regtestrpcuser".to_string(), "regtestrpcpass".to_string())).unwrap()
        })
    }
}