/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, CosmosMsgForEmpty, BankMsg, Uint128, StakingMsg, DistributionMsg, Binary, IbcMsg, Timestamp, Uint64, WasmMsg, GovMsg, VoteOption, Boundary, Interval, ValueIndex, PathToValue, UpdateConfigMsg, TaskRequest, ActionForEmpty, Coin, Empty, IbcTimeout, IbcTimeoutBlock, Cw20Coin, CroncatQuery, Transform, TasksRemoveTaskByManager, TasksRescheduleTask, QueryMsg, Addr, Task, AmountForOneTask, Cw20CoinVerified, BoundaryValidated, Config, TaskResponse, TaskInfo, CurrentTaskInfoResponse, SlotHashesResponse, SlotIdsResponse, SlotTasksTotalResponse, String, ArrayOfTaskInfo, ArrayOfTaskResponse } from "./CroncatTasks.types";
export interface CroncatTasksReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  tasksTotal: () => Promise<Uint64>;
  currentTaskInfo: () => Promise<CurrentTaskInfoResponse>;
  tasksWithQueriesTotal: () => Promise<Uint64>;
  tasks: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfTaskInfo>;
  tasksWithQueries: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfTaskResponse>;
  tasksByOwner: ({
    fromIndex,
    limit,
    ownerAddr
  }: {
    fromIndex?: number;
    limit?: number;
    ownerAddr: string;
  }) => Promise<ArrayOfTaskResponse>;
  task: ({
    taskHash
  }: {
    taskHash: string;
  }) => Promise<TaskResponse>;
  taskHash: ({
    task
  }: {
    task: Task;
  }) => Promise<String>;
  slotHashes: ({
    slot
  }: {
    slot?: number;
  }) => Promise<SlotHashesResponse>;
  slotIds: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<SlotIdsResponse>;
  slotTasksTotal: ({
    offset
  }: {
    offset?: number;
  }) => Promise<SlotTasksTotalResponse>;
  currentTask: () => Promise<TaskResponse>;
  currentTaskWithQueries: ({
    taskHash
  }: {
    taskHash: string;
  }) => Promise<TaskResponse>;
}
export class CroncatTasksQueryClient implements CroncatTasksReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.tasksTotal = this.tasksTotal.bind(this);
    this.currentTaskInfo = this.currentTaskInfo.bind(this);
    this.tasksWithQueriesTotal = this.tasksWithQueriesTotal.bind(this);
    this.tasks = this.tasks.bind(this);
    this.tasksWithQueries = this.tasksWithQueries.bind(this);
    this.tasksByOwner = this.tasksByOwner.bind(this);
    this.task = this.task.bind(this);
    this.taskHash = this.taskHash.bind(this);
    this.slotHashes = this.slotHashes.bind(this);
    this.slotIds = this.slotIds.bind(this);
    this.slotTasksTotal = this.slotTasksTotal.bind(this);
    this.currentTask = this.currentTask.bind(this);
    this.currentTaskWithQueries = this.currentTaskWithQueries.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  tasksTotal = async (): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_total: {}
    });
  };
  currentTaskInfo = async (): Promise<CurrentTaskInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_task_info: {}
    });
  };
  tasksWithQueriesTotal = async (): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_with_queries_total: {}
    });
  };
  tasks = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfTaskInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks: {
        from_index: fromIndex,
        limit
      }
    });
  };
  tasksWithQueries = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfTaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_with_queries: {
        from_index: fromIndex,
        limit
      }
    });
  };
  tasksByOwner = async ({
    fromIndex,
    limit,
    ownerAddr
  }: {
    fromIndex?: number;
    limit?: number;
    ownerAddr: string;
  }): Promise<ArrayOfTaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_by_owner: {
        from_index: fromIndex,
        limit,
        owner_addr: ownerAddr
      }
    });
  };
  task = async ({
    taskHash
  }: {
    taskHash: string;
  }): Promise<TaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      task: {
        task_hash: taskHash
      }
    });
  };
  taskHash = async ({
    task
  }: {
    task: Task;
  }): Promise<String> => {
    return this.client.queryContractSmart(this.contractAddress, {
      task_hash: {
        task
      }
    });
  };
  slotHashes = async ({
    slot
  }: {
    slot?: number;
  }): Promise<SlotHashesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_hashes: {
        slot
      }
    });
  };
  slotIds = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<SlotIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_ids: {
        from_index: fromIndex,
        limit
      }
    });
  };
  slotTasksTotal = async ({
    offset
  }: {
    offset?: number;
  }): Promise<SlotTasksTotalResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_tasks_total: {
        offset
      }
    });
  };
  currentTask = async (): Promise<TaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_task: {}
    });
  };
  currentTaskWithQueries = async ({
    taskHash
  }: {
    taskHash: string;
  }): Promise<TaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_task_with_queries: {
        task_hash: taskHash
      }
    });
  };
}
export interface CroncatTasksInterface extends CroncatTasksReadOnlyInterface {
  contractAddress: string;
  sender: string;
  updateConfig: ({
    croncatAgentsKey,
    croncatFactoryAddr,
    croncatManagerKey,
    gasActionFee,
    gasBaseFee,
    gasLimit,
    gasQueryFee,
    ownerAddr,
    paused,
    slotGranularityTime
  }: {
    croncatAgentsKey?: string[][];
    croncatFactoryAddr?: string;
    croncatManagerKey?: string[][];
    gasActionFee?: number;
    gasBaseFee?: number;
    gasLimit?: number;
    gasQueryFee?: number;
    ownerAddr?: string;
    paused?: boolean;
    slotGranularityTime?: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  createTask: ({
    task
  }: {
    task: TaskRequest;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeTask: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeTaskByManager: ({
    taskHash
  }: {
    taskHash: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  rescheduleTask: ({
    taskHash
  }: {
    taskHash: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CroncatTasksClient extends CroncatTasksQueryClient implements CroncatTasksInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateConfig = this.updateConfig.bind(this);
    this.createTask = this.createTask.bind(this);
    this.removeTask = this.removeTask.bind(this);
    this.removeTaskByManager = this.removeTaskByManager.bind(this);
    this.rescheduleTask = this.rescheduleTask.bind(this);
  }

  updateConfig = async ({
    croncatAgentsKey,
    croncatFactoryAddr,
    croncatManagerKey,
    gasActionFee,
    gasBaseFee,
    gasLimit,
    gasQueryFee,
    ownerAddr,
    paused,
    slotGranularityTime
  }: {
    croncatAgentsKey?: string[][];
    croncatFactoryAddr?: string;
    croncatManagerKey?: string[][];
    gasActionFee?: number;
    gasBaseFee?: number;
    gasLimit?: number;
    gasQueryFee?: number;
    ownerAddr?: string;
    paused?: boolean;
    slotGranularityTime?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        croncat_agents_key: croncatAgentsKey,
        croncat_factory_addr: croncatFactoryAddr,
        croncat_manager_key: croncatManagerKey,
        gas_action_fee: gasActionFee,
        gas_base_fee: gasBaseFee,
        gas_limit: gasLimit,
        gas_query_fee: gasQueryFee,
        owner_addr: ownerAddr,
        paused,
        slot_granularity_time: slotGranularityTime
      }
    }, fee, memo, funds);
  };
  createTask = async ({
    task
  }: {
    task: TaskRequest;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
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
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  removeTaskByManager = async ({
    taskHash
  }: {
    taskHash: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task_by_manager: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  rescheduleTask = async ({
    taskHash
  }: {
    taskHash: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      reschedule_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
}