mod beacon_block_body_merkle_tree;
mod beacon_rpc_client;
mod config_for_tests;
mod errors;
mod execution_block_proof;
mod light_client_snapshot_with_proof;
mod utils;

use crate::beacon_rpc_client::BeaconRPCClient;
use crate::config_for_tests::ConfigForTests;

pub fn main() {
    fn get_test_config() -> ConfigForTests {
        ConfigForTests::load_from_toml("config_for_tests.toml".try_into().unwrap())
    }

    let config = get_test_config();

    let beacon_rpc_client = BeaconRPCClient::new(
        &config.beacon_endpoint,
        config.eth_requests_timeout_seconds.unwrap_or(10),
        config.eth_requests_timeout_seconds.unwrap_or(10),
        Some(config.beacon_rpc_version.clone()),
    );

    let light_client_update_with_next_sync_committee = beacon_rpc_client
        .get_light_client_update_for_last_period()
        .expect("Error on fetching finality light client update with sync committee update");
    let finality_light_client_update = beacon_rpc_client
        .get_finality_light_client_update()
        .expect("Error on fetching finality light client update");
}
