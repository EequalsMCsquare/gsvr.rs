import { PfAgent, PfAgentData } from './pf';
import { GateAgent, GateAgentData } from './gate';
type AgentInfo = {
    name: string | number;
    typ: 'fast' | 'normal';
    key: string;
};
type AgentMgrData = {
    pfs: PfAgentData[];
    gates: GateAgentData[];
};
declare class AgentMgr {
    private _pfs;
    private _gates;
    private _info;
    constructor(data: AgentMgrData);
    get info(): AgentInfo[];
    addPfAgent(username: string, password: string): Promise<PfAgent>;
    addGateAgent(pid: number): Promise<GateAgent>;
    removeGateAgent(pid: number): Promise<void>;
    removePfAgent(username: string): Promise<void>;
    gateAgent(pid: number): GateAgent | undefined;
    pfAgent(username: string): PfAgent | undefined;
}
export { AgentMgr };
export type { AgentMgrData, AgentInfo };
