// Starts Regtest Node providing a temp Config
// Every Level Setup Includes running a Clean Regtest Node

use std::thread::sleep;

use anyhow::{Ok, Result};
use bitcoind::{
    bitcoincore_rpc::{Auth, Client, RpcApi},
    BitcoinD, Conf,
};

// might need state later!!
pub struct CtfFramework {
    pub client: Client,
}

impl<'a> CtfFramework {
    pub fn new() -> Result<Self> {
        let mut conf = Conf::default();
        conf.staticdir = Some("bin/bitcoin/static".into());

        let key = "BITCOIND_EXE";
        let curr_dir_path = std::env::current_dir().unwrap();
        let bitcoind_path = curr_dir_path.join("bin").join("bitcoind");
        std::env::set_var(key, bitcoind_path);
        let exe_path = bitcoind::exe_path().unwrap();
        let bitcoind = BitcoinD::with_conf(exe_path, &conf).unwrap();

        let mining_address = bitcoind
            .client
            .get_new_address(None, None)
            .unwrap()
            .require_network(bitcoind::bitcoincore_rpc::bitcoin::Network::Regtest)
            .unwrap();

        println!("dir {:#?}" , curr_dir_path.join("bin/bitcoin/static/regtest.cookie"));
        sleep(std::time::Duration::from_secs(50));
        Ok(CtfFramework {
            client: Client::new(
                bitcoind.rpc_url().as_str(),
                Auth::CookieFile(curr_dir_path.join("bin/bitcoin/static/regtest.cookie")),
            )
            .unwrap(),
        })
    }
}
