/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.9.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/

import { CosmWasmClient, ExecuteResult, SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
export type Addr = string;
export type Uint128 = string;
export type Timestamp = Uint64;
export type Uint64 = string;
export type SlotType = "Block" | "Cron";
export type AgentStatus = "Active" | "Pending" | "Nominated";
export type CosmosMsgForEmpty = {
  bank: BankMsg;
} | {
  custom: Empty;
} | {
  staking: StakingMsg;
} | {
  distribution: DistributionMsg;
} | {
  stargate: {
    type_url: string;
    value: Binary;
    [k: string]: unknown;
  };
} | {
  ibc: IbcMsg;
} | {
  wasm: WasmMsg;
} | {
  gov: GovMsg;
};
export type BankMsg = {
  send: {
    amount: Coin[];
    to_address: string;
    [k: string]: unknown;
  };
} | {
  burn: {
    amount: Coin[];
    [k: string]: unknown;
  };
};
export type StakingMsg = {
  delegate: {
    amount: Coin;
    validator: string;
    [k: string]: unknown;
  };
} | {
  undelegate: {
    amount: Coin;
    validator: string;
    [k: string]: unknown;
  };
} | {
  redelegate: {
    amount: Coin;
    dst_validator: string;
    src_validator: string;
    [k: string]: unknown;
  };
};
export type DistributionMsg = {
  set_withdraw_address: {
    address: string;
    [k: string]: unknown;
  };
} | {
  withdraw_delegator_reward: {
    validator: string;
    [k: string]: unknown;
  };
};
export type Binary = string;
export type IbcMsg = {
  transfer: {
    amount: Coin;
    channel_id: string;
    timeout: IbcTimeout;
    to_address: string;
    [k: string]: unknown;
  };
} | {
  send_packet: {
    channel_id: string;
    data: Binary;
    timeout: IbcTimeout;
    [k: string]: unknown;
  };
} | {
  close_channel: {
    channel_id: string;
    [k: string]: unknown;
  };
};
export type WasmMsg = {
  execute: {
    contract_addr: string;
    funds: Coin[];
    msg: Binary;
    [k: string]: unknown;
  };
} | {
  instantiate: {
    admin?: string | null;
    code_id: number;
    funds: Coin[];
    label: string;
    msg: Binary;
    [k: string]: unknown;
  };
} | {
  migrate: {
    contract_addr: string;
    msg: Binary;
    new_code_id: number;
    [k: string]: unknown;
  };
} | {
  update_admin: {
    admin: string;
    contract_addr: string;
    [k: string]: unknown;
  };
} | {
  clear_admin: {
    contract_addr: string;
    [k: string]: unknown;
  };
};
export type GovMsg = {
  vote: {
    proposal_id: number;
    vote: VoteOption;
    [k: string]: unknown;
  };
};
export type VoteOption = "yes" | "no" | "abstain" | "no_with_veto";
export type Boundary = {
  Height: {
    end?: Uint64 | null;
    start?: Uint64 | null;
    [k: string]: unknown;
  };
} | {
  Time: {
    end?: Timestamp | null;
    start?: Timestamp | null;
    [k: string]: unknown;
  };
};
export type Interval = ("Once" | "Immediate") | {
  Block: number;
} | {
  Cron: string;
};
export interface Croncat {
  Agent?: Agent | null;
  BalanceResponse?: GetBalancesResponse | null;
  ConfigResponse?: GetConfigResponse | null;
  GetAgentIdsResponse?: GetAgentIdsResponse | null;
  GetAgentResponse?: (AgentResponse | null) | null;
  GetAgentTasksResponse?: AgentTaskResponse | null;
  GetSlotHashesResponse?: GetSlotHashesResponse | null;
  GetSlotIdsResponse?: GetSlotIdsResponse | null;
  GetTaskHashResponse?: string | null;
  GetTaskResponse?: (TaskResponse | null) | null;
  GetTasksByOwnerResponse?: TaskResponse[] | null;
  GetTasksResponse?: TaskResponse[] | null;
  GetWalletBalancesResponse?: GetWalletBalancesResponse | null;
  Task?: Task | null;
  TaskRequest?: TaskRequest | null;
  TaskResponse?: TaskResponse | null;
  ValidateIntervalResponse?: boolean | null;
  [k: string]: unknown;
}
export interface Agent {
  balance: GenericBalance;
  last_missed_slot: number;
  payable_account_id: Addr;
  register_start: Timestamp;
  total_tasks_executed: number;
  [k: string]: unknown;
}
export interface GenericBalance {
  cw20: Cw20CoinVerified[];
  native: Coin[];
  [k: string]: unknown;
}
export interface Cw20CoinVerified {
  address: Addr;
  amount: Uint128;
  [k: string]: unknown;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface GetBalancesResponse {
  available_balance: GenericBalance;
  cw20_whitelist: Addr[];
  native_denom: string;
  staked_balance: GenericBalance;
  [k: string]: unknown;
}
export interface GetConfigResponse {
  agent_active_indices: [SlotType, number, number][];
  agent_fee: Coin;
  agents_eject_threshold: number;
  gas_price: number;
  min_tasks_per_agent: number;
  native_denom: string;
  owner_id: Addr;
  paused: boolean;
  proxy_callback_gas: number;
  slot_granularity: number;
  [k: string]: unknown;
}
export interface GetAgentIdsResponse {
  active: Addr[];
  pending: Addr[];
  [k: string]: unknown;
}
export interface AgentResponse {
  balance: GenericBalance;
  last_missed_slot: number;
  payable_account_id: Addr;
  register_start: Timestamp;
  status: AgentStatus;
  total_tasks_executed: number;
  [k: string]: unknown;
}
export interface AgentTaskResponse {
  num_block_tasks: Uint64;
  num_block_tasks_extra: Uint64;
  num_cron_tasks: Uint64;
  num_cron_tasks_extra: Uint64;
  [k: string]: unknown;
}
export interface GetSlotHashesResponse {
  block_id: number;
  block_task_hash: string[];
  time_id: number;
  time_task_hash: string[];
  [k: string]: unknown;
}
export interface GetSlotIdsResponse {
  block_ids: number[];
  time_ids: number[];
  [k: string]: unknown;
}
export interface TaskResponse {
  actions: ActionForEmpty[];
  boundary?: Boundary | null;
  interval: Interval;
  owner_id: Addr;
  rules?: Rule[] | null;
  stop_on_fail: boolean;
  task_hash: string;
  total_cw20_deposit: Cw20CoinVerified[];
  total_deposit: Coin[];
  [k: string]: unknown;
}
export interface ActionForEmpty {
  gas_limit?: number | null;
  msg: CosmosMsgForEmpty;
  [k: string]: unknown;
}
export interface Empty {
  [k: string]: unknown;
}
export interface IbcTimeout {
  block?: IbcTimeoutBlock | null;
  timestamp?: Timestamp | null;
  [k: string]: unknown;
}
export interface IbcTimeoutBlock {
  height: number;
  revision: number;
  [k: string]: unknown;
}
export interface Rule {
  contract_addr: string;
  msg: Binary;
  [k: string]: unknown;
}
export interface GetWalletBalancesResponse {
  cw20_balances: Cw20CoinVerified[];
  [k: string]: unknown;
}
export interface Task {
  actions: ActionForEmpty[];
  amount_for_one_task: GenericBalance;
  boundary: BoundaryValidated;
  funds_withdrawn_recurring: Uint128;
  interval: Interval;
  owner_id: Addr;
  rules?: Rule[] | null;
  stop_on_fail: boolean;
  total_deposit: GenericBalance;
  [k: string]: unknown;
}
export interface BoundaryValidated {
  end?: number | null;
  start?: number | null;
  [k: string]: unknown;
}
export interface TaskRequest {
  actions: ActionForEmpty[];
  boundary?: Boundary | null;
  cw20_coins: Cw20Coin[];
  interval: Interval;
  rules?: Rule[] | null;
  stop_on_fail: boolean;
  [k: string]: unknown;
}
export interface Cw20Coin {
  address: string;
  amount: Uint128;
  [k: string]: unknown;
}
export type ExecuteMsg = {
  update_settings: {
    agent_fee?: Coin | null;
    agents_eject_threshold?: number | null;
    gas_price?: number | null;
    min_tasks_per_agent?: number | null;
    owner_id?: string | null;
    paused?: boolean | null;
    proxy_callback_gas?: number | null;
    slot_granularity?: number | null;
    [k: string]: unknown;
  };
} | {
  move_balances: {
    account_id: string;
    balances: Balance[];
    [k: string]: unknown;
  };
} | {
  register_agent: {
    payable_account_id?: string | null;
    [k: string]: unknown;
  };
} | {
  update_agent: {
    payable_account_id: string;
    [k: string]: unknown;
  };
} | {
  check_in_agent: {
    [k: string]: unknown;
  };
} | {
  unregister_agent: {
    [k: string]: unknown;
  };
} | {
  withdraw_reward: {
    [k: string]: unknown;
  };
} | {
  create_task: {
    task: TaskRequest;
    [k: string]: unknown;
  };
} | {
  remove_task: {
    task_hash: string;
    [k: string]: unknown;
  };
} | {
  refill_task_balance: {
    task_hash: string;
    [k: string]: unknown;
  };
} | {
  refill_task_cw20_balance: {
    cw20_coins: Cw20Coin[];
    task_hash: string;
    [k: string]: unknown;
  };
} | {
  proxy_call: {
    task_hash?: string | null;
    [k: string]: unknown;
  };
} | {
  receive: Cw20ReceiveMsg;
} | {
  withdraw_wallet_balance: {
    cw20_amounts: Cw20Coin[];
    [k: string]: unknown;
  };
};
export type Balance = {
  native: NativeBalance;
} | {
  cw20: Cw20CoinVerified;
};
export type NativeBalance = Coin[];
export interface Cw20ReceiveMsg {
  amount: Uint128;
  msg: Binary;
  sender: string;
  [k: string]: unknown;
}
export type GetAgentResponse = AgentResponse | null;
export type GetAgentTasksResponse = TaskResponse | null;
export type GetTaskHashResponse = string;
export type GetTaskResponse = TaskResponse | null;
export type GetTasksByOwnerResponse = TaskResponse[];
export type GetTasksResponse = TaskResponse[];
export interface InstantiateMsg {
  agent_nomination_duration?: number | null;
  denom: string;
  gas_base_fee?: Uint64 | null;
  owner_id?: Addr | null;
  [k: string]: unknown;
}
export type QueryMsg = {
  get_config: {
    [k: string]: unknown;
  };
} | {
  get_balances: {
    [k: string]: unknown;
  };
} | {
  get_agent: {
    account_id: string;
    [k: string]: unknown;
  };
} | {
  get_agent_ids: {
    [k: string]: unknown;
  };
} | {
  get_agent_tasks: {
    account_id: string;
    [k: string]: unknown;
  };
} | {
  get_tasks: {
    from_index?: number | null;
    limit?: number | null;
    [k: string]: unknown;
  };
} | {
  get_tasks_with_rules: {
    from_index?: number | null;
    limit?: number | null;
    [k: string]: unknown;
  };
} | {
  get_tasks_by_owner: {
    owner_id: string;
    [k: string]: unknown;
  };
} | {
  get_task: {
    task_hash: string;
    [k: string]: unknown;
  };
} | {
  get_task_hash: {
    task: Task;
    [k: string]: unknown;
  };
} | {
  validate_interval: {
    interval: Interval;
    [k: string]: unknown;
  };
} | {
  get_slot_hashes: {
    slot?: number | null;
    [k: string]: unknown;
  };
} | {
  get_slot_ids: {
    [k: string]: unknown;
  };
} | {
  get_wallet_balances: {
    wallet: string;
    [k: string]: unknown;
  };
};
export type ValidateIntervalResponse = boolean;
export interface CwCroncatReadOnlyInterface {
  contractAddress: string;
  getConfig: () => Promise<GetConfigResponse>;
  getBalances: () => Promise<GetBalancesResponse>;
  getAgent: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<GetAgentResponse>;
  getAgentIds: () => Promise<GetAgentIdsResponse>;
  getAgentTasks: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<GetAgentTasksResponse>;
  getTasks: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetTasksResponse>;
  getTasksWithRules: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetTasksWithRulesResponse>;
  getTasksByOwner: ({
    ownerId
  }: {
    ownerId: string;
  }) => Promise<GetTasksByOwnerResponse>;
  getTask: ({
    taskHash
  }: {
    taskHash: string;
  }) => Promise<GetTaskResponse>;
  getTaskHash: ({
    task
  }: {
    task: Task;
  }) => Promise<GetTaskHashResponse>;
  validateInterval: ({
    interval
  }: {
    interval: string | object;
  }) => Promise<ValidateIntervalResponse>;
  getSlotHashes: ({
    slot
  }: {
    slot?: number;
  }) => Promise<GetSlotHashesResponse>;
  getSlotIds: () => Promise<GetSlotIdsResponse>;
  getWalletBalances: ({
    wallet
  }: {
    wallet: string;
  }) => Promise<GetWalletBalancesResponse>;
}
export class CwCroncatQueryClient implements CwCroncatReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getConfig = this.getConfig.bind(this);
    this.getBalances = this.getBalances.bind(this);
    this.getAgent = this.getAgent.bind(this);
    this.getAgentIds = this.getAgentIds.bind(this);
    this.getAgentTasks = this.getAgentTasks.bind(this);
    this.getTasks = this.getTasks.bind(this);
    this.getTasksWithRules = this.getTasksWithRules.bind(this);
    this.getTasksByOwner = this.getTasksByOwner.bind(this);
    this.getTask = this.getTask.bind(this);
    this.getTaskHash = this.getTaskHash.bind(this);
    this.validateInterval = this.validateInterval.bind(this);
    this.getSlotHashes = this.getSlotHashes.bind(this);
    this.getSlotIds = this.getSlotIds.bind(this);
    this.getWalletBalances = this.getWalletBalances.bind(this);
  }

  getConfig = async (): Promise<GetConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_config: {}
    });
  };
  getBalances = async (): Promise<GetBalancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_balances: {}
    });
  };
  getAgent = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<GetAgentResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent: {
        account_id: accountId
      }
    });
  };
  getAgentIds = async (): Promise<GetAgentIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_ids: {}
    });
  };
  getAgentTasks = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<GetAgentTasksResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_tasks: {
        account_id: accountId
      }
    });
  };
  getTasks = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetTasksResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getTasksWithRules = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetTasksWithRulesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks_with_rules: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getTasksByOwner = async ({
    ownerId
  }: {
    ownerId: string;
  }): Promise<GetTasksByOwnerResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_tasks_by_owner: {
        owner_id: ownerId
      }
    });
  };
  getTask = async ({
    taskHash
  }: {
    taskHash: string;
  }): Promise<GetTaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_task: {
        task_hash: taskHash
      }
    });
  };
  getTaskHash = async ({
    task
  }: {
    task: Task;
  }): Promise<GetTaskHashResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_task_hash: {
        task
      }
    });
  };
  validateInterval = async ({
    interval
  }: {
    interval: string | object;
  }): Promise<ValidateIntervalResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      validate_interval: {
        interval
      }
    });
  };
  getSlotHashes = async ({
    slot
  }: {
    slot?: number;
  }): Promise<GetSlotHashesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_slot_hashes: {
        slot
      }
    });
  };
  getSlotIds = async (): Promise<GetSlotIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_slot_ids: {}
    });
  };
  getWalletBalances = async ({
    wallet
  }: {
    wallet: string;
  }): Promise<GetWalletBalancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_wallet_balances: {
        wallet
      }
    });
  };
}
export interface CwCroncatInterface extends CwCroncatReadOnlyInterface {
  contractAddress: string;
  sender: string;
  updateSettings: ({
    agentFee,
    agentsEjectThreshold,
    gasPrice,
    minTasksPerAgent,
    ownerId,
    paused,
    proxyCallbackGas,
    slotGranularity
  }: {
    agentFee?: Coin;
    agentsEjectThreshold?: number;
    gasPrice?: number;
    minTasksPerAgent?: number;
    ownerId?: string;
    paused?: boolean;
    proxyCallbackGas?: number;
    slotGranularity?: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  moveBalances: ({
    accountId,
    balances
  }: {
    accountId: string;
    balances: Balance[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  registerAgent: ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  updateAgent: ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  checkInAgent: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  unregisterAgent: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  withdrawReward: (fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  createTask: ({
    task
  }: {
    task: TaskRequest;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  removeTask: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  refillTaskBalance: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  refillTaskCw20Balance: ({
    cw20Coins,
    taskHash
  }: {
    cw20Coins: Cw20Coin[];
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  proxyCall: ({
    taskHash
  }: {
    taskHash?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  receive: ({
    amount,
    msg,
    sender
  }: {
    amount: string;
    msg: string;
    sender: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
  withdrawWalletBalance: ({
    cw20Amounts
  }: {
    cw20Amounts: Cw20Coin[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: readonly Coin[]) => Promise<ExecuteResult>;
}
export class CwCroncatClient extends CwCroncatQueryClient implements CwCroncatInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateSettings = this.updateSettings.bind(this);
    this.moveBalances = this.moveBalances.bind(this);
    this.registerAgent = this.registerAgent.bind(this);
    this.updateAgent = this.updateAgent.bind(this);
    this.checkInAgent = this.checkInAgent.bind(this);
    this.unregisterAgent = this.unregisterAgent.bind(this);
    this.withdrawReward = this.withdrawReward.bind(this);
    this.createTask = this.createTask.bind(this);
    this.removeTask = this.removeTask.bind(this);
    this.refillTaskBalance = this.refillTaskBalance.bind(this);
    this.refillTaskCw20Balance = this.refillTaskCw20Balance.bind(this);
    this.proxyCall = this.proxyCall.bind(this);
    this.receive = this.receive.bind(this);
    this.withdrawWalletBalance = this.withdrawWalletBalance.bind(this);
  }

  updateSettings = async ({
    agentFee,
    agentsEjectThreshold,
    gasPrice,
    minTasksPerAgent,
    ownerId,
    paused,
    proxyCallbackGas,
    slotGranularity
  }: {
    agentFee?: Coin;
    agentsEjectThreshold?: number;
    gasPrice?: number;
    minTasksPerAgent?: number;
    ownerId?: string;
    paused?: boolean;
    proxyCallbackGas?: number;
    slotGranularity?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_settings: {
        agent_fee: agentFee,
        agents_eject_threshold: agentsEjectThreshold,
        gas_price: gasPrice,
        min_tasks_per_agent: minTasksPerAgent,
        owner_id: ownerId,
        paused,
        proxy_callback_gas: proxyCallbackGas,
        slot_granularity: slotGranularity
      }
    }, fee, memo, funds);
  };
  moveBalances = async ({
    accountId,
    balances
  }: {
    accountId: string;
    balances: Balance[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      move_balances: {
        account_id: accountId,
        balances
      }
    }, fee, memo, funds);
  };
  registerAgent = async ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  updateAgent = async ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  checkInAgent = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      check_in_agent: {}
    }, fee, memo, funds);
  };
  unregisterAgent = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      unregister_agent: {}
    }, fee, memo, funds);
  };
  withdrawReward = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_reward: {}
    }, fee, memo, funds);
  };
  createTask = async ({
    task
  }: {
    task: TaskRequest;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_task: {
        task
      }
    }, fee, memo, funds);
  };
  removeTask = async ({
    taskHash
  }: {
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  refillTaskBalance = async ({
    taskHash
  }: {
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      refill_task_balance: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  refillTaskCw20Balance = async ({
    cw20Coins,
    taskHash
  }: {
    cw20Coins: Cw20Coin[];
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      refill_task_cw20_balance: {
        cw20_coins: cw20Coins,
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  proxyCall = async ({
    taskHash
  }: {
    taskHash?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      proxy_call: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  receive = async ({
    amount,
    msg,
    sender
  }: {
    amount: string;
    msg: string;
    sender: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      receive: {
        amount,
        msg,
        sender
      }
    }, fee, memo, funds);
  };
  withdrawWalletBalance = async ({
    cw20Amounts
  }: {
    cw20Amounts: Cw20Coin[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: readonly Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_wallet_balance: {
        cw20_amounts: cw20Amounts
      }
    }, fee, memo, funds);
  };
}