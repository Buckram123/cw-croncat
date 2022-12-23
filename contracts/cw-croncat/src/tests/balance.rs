use crate::balancer::{Balancer, BalancerMode, RoundRobinBalancer};
use crate::state::TaskInfo;
use crate::tests::helpers::{default_task, AGENT0, AGENT1, AGENT2, AGENT3, AGENT4};
use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env};
use cosmwasm_std::{coins, Addr};
use cw_croncat_core::types::SlotType;

use crate::CwCroncat;

use super::helpers::NATIVE_DENOM;

#[test]
fn test_agent_has_valid_task_count_ao_mode() {
    let store = CwCroncat::default();
    let mut deps = mock_dependencies_with_balance(&coins(200, NATIVE_DENOM));
    let env = mock_env();
    let mut balancer = RoundRobinBalancer::default();
    store
        .agent_active_indices
        .save(&mut deps.storage, &vec![])
        .unwrap();

    let mut active_agents: Vec<Addr> = store
        .agent_active_queue
        .may_load(&deps.storage)
        .unwrap()
        .unwrap_or_default();
    active_agents.extend(vec![
        Addr::unchecked(AGENT0),
        Addr::unchecked(AGENT1),
        Addr::unchecked(AGENT2),
        Addr::unchecked(AGENT3),
        Addr::unchecked(AGENT4),
    ]);

    store
        .agent_active_queue
        .save(&mut deps.storage, &active_agents)
        .unwrap();
    let slot: (Option<u64>, Option<u64>) = (Some(1), Some(2));
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
            slot,
        )
        .unwrap()
        .unwrap();
    assert_eq!(result.num_block_tasks.u64(), 1);
    assert_eq!(result.num_cron_tasks.u64(), 1);

    //Verify earch gents valid amount
    let slot: (Option<u64>, Option<u64>) = (Some(100), Some(50));
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
            slot,
        )
        .unwrap()
        .unwrap();
    assert!(result.num_block_tasks.u64() == 20);
    assert!(result.num_cron_tasks.u64() == 10);

    //Verify agents gets zero
    let slot: (Option<u64>, Option<u64>) = (Some(0), Some(0));
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
            slot,
        )
        .unwrap()
        .unwrap();
    assert!(result.num_block_tasks.u64() == 0);
    assert!(result.num_cron_tasks.u64() == 0);
}

#[test]
fn test_check_valid_agents_get_extra_tasks_ao_mode() {
    let store = CwCroncat::default();
    let mut deps = mock_dependencies_with_balance(&coins(200, NATIVE_DENOM));
    let env = mock_env();
    let mut balancer = RoundRobinBalancer::default();
    store
        .agent_active_indices
        .save(&mut deps.storage, &vec![])
        .unwrap();

    let mut active_agents: Vec<Addr> = store
        .agent_active_queue
        .may_load(&deps.storage)
        .unwrap()
        .unwrap_or_default();
    active_agents.extend(vec![
        Addr::unchecked(AGENT0),
        Addr::unchecked(AGENT1),
        Addr::unchecked(AGENT2),
        Addr::unchecked(AGENT3),
        Addr::unchecked(AGENT4),
    ]);

    store
        .agent_active_queue
        .save(&mut deps.storage, &active_agents)
        .unwrap();

    //Verify agent0 gets extra
    let slot: (Option<u64>, Option<u64>) = (Some(7), Some(7));
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 2);
    assert_eq!(result.num_cron_tasks.u64(), 2);
    assert_eq!(result.num_block_tasks_extra.u64(), 1);
    assert_eq!(result.num_cron_tasks_extra.u64(), 1);

    //Verify agent1 gets extra
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT1),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 2);
    assert_eq!(result.num_cron_tasks.u64(), 2);
    assert_eq!(result.num_block_tasks_extra.u64(), 1);
    assert_eq!(result.num_cron_tasks_extra.u64(), 1);

    //Verify agent3 not getting extra
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT3),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 1);
    assert_eq!(result.num_cron_tasks.u64(), 1);
    assert_eq!(result.num_block_tasks_extra.u64(), 0);
    assert_eq!(result.num_cron_tasks_extra.u64(), 0);
}
#[test]
fn test_check_valid_agents_get_extra_tasks_eq_mode() {
    let store = CwCroncat::default();
    let mut deps = mock_dependencies_with_balance(&coins(200, NATIVE_DENOM));
    let env = mock_env();
    let mut balancer = RoundRobinBalancer::new(BalancerMode::Equalizer);
    store
        .agent_active_indices
        .save(&mut deps.storage, &vec![])
        .unwrap();

    let mut active_agents: Vec<Addr> = store
        .agent_active_queue
        .may_load(&deps.storage)
        .unwrap()
        .unwrap_or_default();
    active_agents.extend(vec![
        Addr::unchecked(AGENT0),
        Addr::unchecked(AGENT1),
        Addr::unchecked(AGENT2),
        Addr::unchecked(AGENT3),
        Addr::unchecked(AGENT4),
    ]);

    store
        .agent_active_queue
        .save(&mut deps.storage, &active_agents)
        .unwrap();

    let task_info = TaskInfo {
        task: default_task(),
        task_hash: "".as_bytes().to_vec(),
        task_is_extra: Some(true),
        agent_id: Addr::unchecked(AGENT0),
        slot_kind: SlotType::Block,
    };

    //Notify agent got 1 task
    balancer
        .on_task_completed(
            &mut deps.storage,
            &env,
            &store.agent_active_indices,
            &store.agent_active_queue,
            &task_info,
        )
        .unwrap();

    //Verify agent0 gets extra
    let slot: (Option<u64>, Option<u64>) = (Some(7), Some(7));
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
            slot,
        )
        .unwrap()
        .unwrap();

    //In equalizer mode, agent0 get 1 task and 0 extra
    assert_eq!(result.num_block_tasks.u64(), 1);
    assert_eq!(result.num_cron_tasks.u64(), 1);
    assert_eq!(result.num_block_tasks_extra.u64(), 0);
    assert_eq!(result.num_cron_tasks_extra.u64(), 0);

    //Verify agent1 gets extra
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT1),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 2);
    assert_eq!(result.num_cron_tasks.u64(), 2);
    assert_eq!(result.num_block_tasks_extra.u64(), 1);
    assert_eq!(result.num_cron_tasks_extra.u64(), 1);

    //Verify agent2 gets extra
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT2),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 2);
    assert_eq!(result.num_cron_tasks.u64(), 2);
    assert_eq!(result.num_block_tasks_extra.u64(), 1);
    assert_eq!(result.num_cron_tasks_extra.u64(), 1);

    //Verify agent3 not getting extra
    let result = balancer
        .get_agent_tasks(
            &deps.as_ref(),
            &env.clone(),
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT3),
            slot,
        )
        .unwrap()
        .unwrap();

    assert_eq!(result.num_block_tasks.u64(), 1);
    assert_eq!(result.num_cron_tasks.u64(), 1);
    assert_eq!(result.num_block_tasks_extra.u64(), 0);
    assert_eq!(result.num_cron_tasks_extra.u64(), 0);
}
#[test]
fn test_on_task_completed() {
    let store = CwCroncat::default();
    let mut deps = mock_dependencies_with_balance(&coins(200, NATIVE_DENOM));
    let env = mock_env();
    let balancer = RoundRobinBalancer::default();
    store
        .agent_active_indices
        .save(&mut deps.storage, &vec![])
        .unwrap();

    let mut active_agents: Vec<Addr> = store
        .agent_active_queue
        .may_load(&deps.storage)
        .unwrap()
        .unwrap_or_default();
    active_agents.extend(vec![
        Addr::unchecked(AGENT0),
        Addr::unchecked(AGENT1),
        Addr::unchecked(AGENT2),
        Addr::unchecked(AGENT3),
        Addr::unchecked(AGENT4),
    ]);

    store
        .agent_active_queue
        .save(&mut deps.storage, &active_agents)
        .unwrap();

    let task_info = TaskInfo {
        task: default_task(),
        task_hash: "".as_bytes().to_vec(),
        task_is_extra: Some(true),
        agent_id: Addr::unchecked(AGENT0),
        slot_kind: SlotType::Block,
    };
    let mut agent_active_indices = store.agent_active_indices.load(&mut deps.storage).unwrap();

    balancer.update_or_append(&mut agent_active_indices, (SlotType::Block, 0, 10));
    store
        .agent_active_indices
        .save(&mut deps.storage, &agent_active_indices)
        .unwrap();
    balancer
        .on_task_completed(
            &mut deps.storage,
            &env,
            &store.agent_active_indices,
            &store.agent_active_queue,
            &task_info,
        )
        .unwrap();

    agent_active_indices = store.agent_active_indices.load(&mut deps.storage).unwrap();
    assert_eq!(agent_active_indices, vec![(SlotType::Block, 0, 11)])
}

#[test]
fn test_on_agent_unregister() {
    let store = CwCroncat::default();
    let mut deps = mock_dependencies_with_balance(&coins(200, NATIVE_DENOM));
    let balancer = RoundRobinBalancer::default();
    store
        .agent_active_indices
        .save(&mut deps.storage, &vec![])
        .unwrap();

    let mut active_agents: Vec<Addr> = store
        .agent_active_queue
        .may_load(&deps.storage)
        .unwrap()
        .unwrap_or_default();
    active_agents.extend(vec![
        Addr::unchecked(AGENT0),
        Addr::unchecked(AGENT1),
        Addr::unchecked(AGENT2),
        Addr::unchecked(AGENT3),
        Addr::unchecked(AGENT4),
    ]);

    store
        .agent_active_queue
        .save(&mut deps.storage, &active_agents)
        .unwrap();

    let mut agent_active_indices = store.agent_active_indices.load(&mut deps.storage).unwrap();
    balancer.update_or_append(&mut agent_active_indices, (SlotType::Block, 0, 1));
    balancer.update_or_append(&mut agent_active_indices, (SlotType::Cron, 0, 1));
    store
        .agent_active_indices
        .save(&mut deps.storage, &agent_active_indices)
        .unwrap();
    balancer
        .on_agent_unregister(
            &mut deps.storage,
            &store.agent_active_indices,
            &store.agent_active_queue,
            Addr::unchecked(AGENT0),
        )
        .unwrap();

    agent_active_indices = store.agent_active_indices.load(&mut deps.storage).unwrap();
    assert_eq!(agent_active_indices, vec![])
}
