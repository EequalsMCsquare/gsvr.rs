import { AgentMgr, AgentInfo } from "./agentmgr";
import { AccountPlayer, PfAgent } from "./pf";
import { GateAgent, GateAgentData, PingInfo } from "./gate";
import { HistoryData } from "./history";
declare function useAgentMgr(): Promise<AgentMgr>;
export { useAgentMgr, };
export type { HistoryData, GateAgentData, PingInfo, AgentMgr, GateAgent, AccountPlayer, PfAgent, AgentInfo, };
