import { invoke } from '@tauri-apps/api';
import { listen, Event } from "@tauri-apps/api/event";
import type { HistoryData } from './history';


type GateAgentData = {
    pid: number
}

class GateAgent {
    private _pid: number
    private reqHis: HistoryData[]
    private ackHis: HistoryData[]
    private _unlistenFn: () => Promise<void>

    constructor(data: GateAgentData) {
        this._pid = data.pid;
        this.reqHis = new Array();
        this.ackHis = new Array();
        this._unlistenFn = async () => { }
    }

    public get id(): string {
        return `p-${this._pid}`
    }

    public get pid(): number {
        return this._pid
    }

    public get reqs(): HistoryData[] {
        return this.reqHis
    }

    public get acks(): HistoryData[] {
        return this.ackHis
    }

    public async refreshHistory() {
        this.reqHis.splice(0, this.reqHis.length);
        this.reqHis.push(...JSON.parse(await invoke<string>("plugin:agentmgr|history_send", { pid: this.pid, limit: 64, reverse: true })));
        this.ackHis.splice(0, this.ackHis.length);
        this.ackHis.push(...JSON.parse(await invoke<string>("plugin:agentmgr|history_recv", { pid: this.pid, limit: 64, reverse: true })));
    }

    public async useHistory(reqs: HistoryData[], acks: HistoryData[]) {
        this.reqHis = reqs;
        this.ackHis = acks;
        await this.refreshHistory();
    }

    public async listen(): Promise<void> {
        await invoke("plugin:agentmgr|gate_listen_recv", { pid: this.pid });
        const unlisten = await listen(`psc-${this.pid}`, (event: Event<string>) => {
            let his: HistoryData = JSON.parse(event.payload);
            this.ackHis.push(his);
        });
        this._unlistenFn = (): Promise<void> => {
            return new Promise<void>((resolve, reject) => {
                invoke("plugin:agentmgr|gate_unlisten_recv", { pid: this.pid }).then(_ => {
                    console.log(`unlisten psc.${this.pid}`)
                    unlisten();
                    resolve();
                }).catch(err => {
                    reject(err)
                });
            });
        }
    }

    public get unlisten(): () => Promise<void> {
        return this._unlistenFn
    }

    public send(msg: string): Promise<HistoryData> {
        return new Promise((resolve, reject) => {
            invoke<string>("plugin:agentmgr|gate_send", { pid: this._pid, msg }).then(res => {
                const his: HistoryData = JSON.parse(res);
                this.reqHis.push(his);
                resolve(his);
            }).catch(err => {
                reject(err);
            });
        });
    }

    public recv(): Promise<HistoryData> {
        return new Promise((resolve, reject) => {
            invoke<string>("plugin:agentmgr|gate_recv", { pid: this._pid }).then(res => {
                const his: HistoryData = JSON.parse(res);
                this.ackHis.push(his);
                console.log(this.ackHis);
                resolve(his);
            }).catch(err => {
                reject(err)
            });
        })
    }
}

export {
    GateAgent
}

export type {
    GateAgentData
}