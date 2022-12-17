import { invoke } from '@tauri-apps/api';
import { listen, Event } from "@tauri-apps/api/event";
import type { HistoryData } from './history';


type GateAgentData = {
    pid: number
}

type PingInfo = {
    // start: number,
    // end: number,
    ping: number,
}

class GateAgent {
    private _pid: number
    private reqHis: HistoryData[]
    private ackHis: HistoryData[]
    private _unlistenFn: () => Promise<void>

    private _ping: PingInfo
    private _start: number
    private _wait_seq: number
    private _timeout: number

    constructor(data: GateAgentData) {
        this._pid = data.pid;
        this.reqHis = new Array();
        this.ackHis = new Array();
        this._unlistenFn = async () => { };
        this._wait_seq = 0;
        this._ping = {
            ping: 0
        };
        this._start = 0;
        this._timeout = 0;
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


    public async usePing(ping: PingInfo) {
        this._ping = ping;
        this._wait_seq++;
        const msg = `{"CsPing":{"seq": ${this._wait_seq}}}`;
        await invoke<string>("plugin:agentmgr|gate_send", { pid: this._pid, msg });
        this._start = Date.now();
    }

    public async listen(): Promise<void> {
        await invoke("plugin:agentmgr|gate_listen_recv", { pid: this.pid });
        const unlisten = await listen(`psc-${this.pid}`, (event: Event<string>) => {
            let his: HistoryData = JSON.parse(event.payload);
            // ping 
            if (this._wait_seq !== 0 && his.msgid === 2) {
                const seq: number = his.payload.ScPing.seq;
                if (this._wait_seq === seq) {
                    const end = Date.now();
                    this._ping.ping = end - this._start;
                    this._timeout = window.setTimeout(async () => {
                        this._wait_seq++;
                        const msg = `{"CsPing":{"seq": ${this._wait_seq}}}`;
                        await invoke<string>("plugin:agentmgr|gate_send", { pid: this._pid, msg });
                        this._start = Date.now();
                    }, 2500);
                }
            } else {
                this.ackHis.push(his);
            }
        });
        this._unlistenFn = (): Promise<void> => {
            return new Promise<void>((resolve, reject) => {
                invoke("plugin:agentmgr|gate_unlisten_recv", { pid: this.pid }).then(_ => {
                    console.log(`unlisten psc.${this.pid}`)
                    window.clearTimeout(this._timeout);
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
    GateAgentData, PingInfo
}