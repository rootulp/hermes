use std::thread;

use ibc_test_framework::{prelude::*, util::random::random_u128_range, chain::exec::simple_exec};

#[test]
fn test_client_default_refresh() -> Result<(), Error> {
    run_binary_channel_test(&ICS31Test)
}

struct ICS31Test;

// Override the clients `trusting_period` such that the refresh_window is 40 seconds.
impl TestOverrides for ICS31Test {
    fn modify_genesis_file(&self, genesis: &mut serde_json::Value) -> Result<(), Error> {

        warn!("genesis: {:?}", genesis);


        match genesis
            .get_mut("app_state")
            .and_then(|app_state| app_state.get_mut("epochs"))
            .and_then(|epochs| epochs.get_mut("epochs"))
            .and_then(|epochs_list| epochs_list.as_array_mut()) {
                Some(epochs_list) => {

                    for v in epochs_list {
                        warn!("v: {:?}", v);
                        let identifier = v.get("identifier")
                        //.and_then(|identifier| identifier.as_object())
                        .ok_or_else(|| eyre!("failed to find identifier"))?;
                        warn!("identifier: {:?}", identifier);
            
            
                        if identifier.as_str() == Some("stride_epoch") {
                            warn!("Will update duration");
                            let duration = v.get_mut("duration")
                            .ok_or_else(|| eyre!("failed to get duration"))?;

                            warn!("before duration: {:?}", duration);
            
                            *duration = serde_json::Value::String("3s".to_owned());
                            warn!("after duration: {:?}", duration);
                        } else {
                            warn!("Wrong identifier: {:?}", identifier.as_str());
                        }
                    }

                }
                None => {
                    warn!("No epochs list");
                }
            }
        Ok(())
    }
}

impl BinaryChannelTest for ICS31Test {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        _relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channel: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        let denom_a = chains.node_a.denom();
        let a_to_b_amount = random_u128_range(1000, 5000);
        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
            denom_a
        );

        info!("wallet_a: {:?}", wallet_a);
        info!("wallet_b: {:?}", wallet_b);

        chains.node_a.chain_driver().ibc_transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_a.as_ref(),
            &wallet_b.address(),
            &denom_a.with_amount(a_to_b_amount).as_ref(),
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            &denom_b.with_amount(a_to_b_amount).as_ref(),
        )?;

    info!("Registering host-zone");

    info!("chain id: {}", chains.chain_id_b().0.as_str());
    simple_exec(
        "stride",
        "strided",
        &[
            "--home",
            chains.node_b.0.chain_driver.home_path.as_str(),
            "--node",
            chains.node_b.0.chain_driver.rpc_listen_address().as_str(),
            "--keyring-backend",
            "test",
            "tx",
            "stakeibc",
            "register-host-zone",
            "connection-0",
            denom_a.0.as_str(),
            "cosmos",
            denom_b.0.as_str(),
            channel.channel_id_a.0.as_str(),
            "1",
            "--from",
            "admin",
            "--chain-id",
            chains.chain_id_b().0.as_str(),
            "--gas",
            "auto",
            "-b",
            "block",
        ],
    )?;
        thread::sleep(Duration::from_secs(60));
        Ok(())
    }
}
