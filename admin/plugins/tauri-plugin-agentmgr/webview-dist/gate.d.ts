import type { HistoryData } from './history';
type GateAgentData = {
    pid: number;
};
declare class GateAgent {
    private _pid;
    private reqHis;
    private ackHis;
    private _unlistenFn;
    constructor(data: GateAgentData);
    get id(): string;
    get pid(): number;
    get reqs(): HistoryData[];
    get acks(): HistoryData[];
    refreshHistory(): Promise<void>;
    useHistory(reqs: HistoryData[], acks: HistoryData[]): Promise<void>;
    listen(): Promise<void>;
    get unlisten(): () => Promise<void>;
    send(msg: string): Promise<HistoryData>;
    recv(): Promise<HistoryData>;
}
export { GateAgent };
export type { GateAgentData };
