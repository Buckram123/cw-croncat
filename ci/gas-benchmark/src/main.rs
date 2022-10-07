mod helpers;
mod report;
mod test_cases;
mod types;

use anyhow::Result;
use cosm_orc::{config::cfg::Config, orchestrator::cosm_orc::CosmOrc};
use types::Account;

use crate::{
    helpers::{
        average_gas_for_one_native_ujunox, complete_tasks_for_three_times, init_contracts,
        key_addr_from_account, register_agent,
    },
    report::cost_approxes,
    test_cases::{
        complete_simple_task, delegate_to_bob_and_alice_recurring, delegate_to_bob_recurring,
        send_to_bob_and_alice_recurring, send_to_bob_recurring,
    },
};

const RULES_NAME: &str = "cw_rules";
const CRONCAT_NAME: &str = "cw_croncat";
const BOB_ADDR: &str = "juno14vhcdsyf83ngsrrqc92kmw8q9xakqjm0ff2dpn";
const ALICE_ADDR: &str = "juno1l8hl8e0ut8jdaecxwazs9m32ak02ez4rssq4wl";

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
    // TEST IT WORKS
    let _ = complete_simple_task(&mut orc, (&agent_key, &agent_addr), &user_key, &denom)?;

    let tasks = vec![
        // Send tasks
        (send_to_bob_recurring(&denom), 100),
        (send_to_bob_and_alice_recurring(&denom), 100),
        // Stake tasks
        (delegate_to_bob_recurring(&denom), 100),
        (delegate_to_bob_and_alice_recurring(&denom), 100),
    ];
    let gas_fees_usage = complete_tasks_for_three_times(
        &mut orc,
        (&agent_key, &agent_addr),
        &user_key,
        &denom,
        tasks,
    )?;
    let cost_per_send = cost_approxes(&gas_fees_usage[0], &gas_fees_usage[1]);
    println!("bank send reports:");
    println!("approx_base_gas: {}", cost_per_send.approx_base_gas());
    println!(
        "approx_gas_per_action: {}\n",
        cost_per_send.approx_gas_per_action()
    );

    let cost_per_delegate = cost_approxes(&gas_fees_usage[2], &gas_fees_usage[3]);
    println!("delegate reports:");
    println!("approx_base_gas: {}", cost_per_delegate.approx_base_gas());
    println!(
        "approx_gas_per_action: {}\n",
        cost_per_delegate.approx_gas_per_action()
    );

    let all_tasks_info = gas_fees_usage.into_iter().flatten().collect();
    println!(
        "avg_gas: {}",
        average_gas_for_one_native_ujunox(all_tasks_info)
    );

    let gas_report_dir = std::env::var("GAS_OUT_DIR").unwrap_or_else(|_| "gas_reports".to_string());
    save_gas_report(&orc, &gas_report_dir);
    Ok(())
}

fn save_gas_report(orc: &CosmOrc, gas_report_dir: &str) {
    let report = orc
        .gas_profiler_report()
        .expect("error fetching profile reports");

    let s = serde_json::to_string(report).unwrap();

    let p = std::path::Path::new(gas_report_dir);
    if !p.exists() {
        std::fs::create_dir(p).unwrap();
    }

    let file_name = "gas_report.json";
    std::fs::write(p.join(file_name), s).unwrap();
}