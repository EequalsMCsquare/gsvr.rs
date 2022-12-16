import { AgentMgr, AgentMgrData, AgentInfo } from "./agentmgr"
import { AccountPlayer, PfAgent } from "./pf"
import { GateAgent, GateAgentData } from "./gate";
import { HistoryData } from "./history";
import { invoke } from "@tauri-apps/api";

let _agentMgr: AgentMgr;

function useAgentMgr(): Promise<AgentMgr> {
  return new Promise<AgentMgr>((resolve, reject) => {
    if (_agentMgr === undefined) {
      invoke<AgentMgrData>("plugin:agentmgr|agent_mgr_cache").then(data => {
        _agentMgr = new AgentMgr(data);
        resolve(_agentMgr);
      }).catch((err: any) => {
        console.error(err);
        reject(err)
      })
    } else {
      resolve(_agentMgr)
    }
  })
}

export {
  useAgentMgr,
}

export type {
  HistoryData,
  GateAgentData,
  AgentMgr,
  GateAgent,
  AccountPlayer,
  PfAgent,
  AgentInfo,
}