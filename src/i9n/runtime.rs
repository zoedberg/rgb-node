// RGB standard library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::thread;

use lnpbp::lnp::transport::zmqsocket::ZmqType;
use lnpbp::lnp::{
    session, transport, CreateUnmarshaller, PlainTranscoder, Unmarshaller,
};

use super::Config;
use crate::api::Reply;
use crate::error::BootstrapError;
use crate::rgbd::{self, ContractName};

pub struct Runtime {
    pub(super) config: Config,
    pub(super) session_rpc:
        session::Raw<PlainTranscoder, transport::zmqsocket::Connection>,
    pub(super) unmarshaller: Unmarshaller<Reply>,
}

impl Runtime {
    pub fn init(config: Config) -> Result<Self, BootstrapError> {
        // Start rgbd on a separate thread
        if config.threaded {
            let rgbd_opts = rgbd::Opts {
                verbose: 5,
                bin_dir: String::new(),
                data_dir: config.data_dir.clone(),
                contracts: config
                    .contract_endpoints
                    .iter()
                    .map(|(k, _)| k.clone())
                    .collect(),
                network: config.network.clone(),
                threaded: true,
            };

            thread::spawn(move || {
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    rgbd::main_with_config(rgbd_opts.into()).await.unwrap();
                });
            });
        }

        let session_rpc = session::Raw::with_zmq_unencrypted(
            ZmqType::Req,
            config
                .contract_endpoints
                .get(&ContractName::Fungible)
                .expect(
                    "Fungible engine is not connected in the configuration",
                ),
            None,
            None,
        )?;
        Ok(Self {
            config,
            session_rpc,
            unmarshaller: Reply::create_unmarshaller(),
        })
    }
}
