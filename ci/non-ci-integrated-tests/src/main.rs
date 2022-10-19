mod helpers;
mod test_cases;
mod types;

use anyhow::Result;
use cosm_orc::{config::cfg::Config, orchestrator::cosm_orc::CosmOrc};
use helpers::{create_task, execute_proxy, init_contracts, key_addr_from_account, register_agent};
use types::Account;

const RULES_NAME: &str = "cw_rules";
const CRONCAT_NAME: &str = "cw_croncat";
const ADDR_1: &str = "juno1njf5qv8ryfl07qgu5hqy8ywcvzwyrt4kzqp07d";
const ADDR_2: &str = "juno1pd43m659naajmn2chkt6tna0uud2ywyp5dm4h3";
const ADDR_3: &str = "juno15w7hw4klzl9j2hk4vq7r3vuhz53h3mlzug9q6s";

fn main() -> Result<()> {
    env_logger::init();

    let cfg = Config::from_yaml("ci/local_config.yaml")?;
    let denom = cfg.chain_cfg.denom.clone();
    let mut orc = CosmOrc::new(cfg, true)?;
    let accounts: Vec<Account> = serde_json::from_slice(&std::fs::read("ci/test_accounts.json")?)?;
    let admin_account = accounts[0].clone();
    let agent_account = accounts[1].clone();
    let user_account = accounts[2].clone();

    let (admin_key, admin_addr) = key_addr_from_account(admin_account);
    let (agent_key, agent_addr) = key_addr_from_account(agent_account);
    let (user_key, _user_addr) = key_addr_from_account(user_account);

    init_contracts(&mut orc, &admin_key, &admin_addr, &denom)?;
    register_agent(&mut orc, &agent_key)?;

    let task_request = test_cases::three_send_actions(&denom);
    create_task(
        &mut orc,
        task_request,
        &user_key,
        vec![cosm_orc::config::cfg::Coin {
            denom: denom.clone(),
            amount: 300_000,
        }],
    )?;
    // make sure balance is updated
    orc.poll_for_n_blocks(1, std::time::Duration::from_millis(20_000), false)?;
    let agent_balance_before_proxy = helpers::query_balance(&mut orc, &agent_addr, &denom)?;

    // wait for the task to be ready
    orc.poll_for_n_blocks(3, std::time::Duration::from_millis(20_000), false)?;
    execute_proxy(&mut orc, &agent_key)?;
    let agent_balance_after_1_proxy = helpers::query_balance(&mut orc, &agent_addr, &denom)?;
    println!(
        "diff_first_proxy = {}",
        agent_balance_before_proxy - agent_balance_after_1_proxy
    );

    // wait for the task to be ready
    orc.poll_for_n_blocks(3, std::time::Duration::from_millis(20_000), false)?;
    execute_proxy(&mut orc, &agent_key)?;
    let agent_balance_after_2_proxy = helpers::query_balance(&mut orc, &agent_addr, &denom)?;
    println!(
        "diff_second_proxy = {}",
        agent_balance_after_1_proxy - agent_balance_after_2_proxy
    );

    Ok(())
}
