import { invoke } from '@tauri-apps/api/tauri'
import { AccountPlayer, PfAgent, PfAgentData } from './pf'
import { GateAgent, GateAgentData } from './gate'

type AgentInfo = {
  name: string | number,
  typ: 'fast' | 'normal',
  key: string,
};

type AgentMgrData = {
  pfs: PfAgentData[],
  gates: GateAgentData[],
}

type AddPfAgentAck = {
  username: string,
  players: AccountPlayer[],
}

class AgentMgr {
  private _pfs: Map<string, PfAgent>
  private _gates: Map<number, GateAgent>
  private _info: AgentInfo[]

  constructor(data: AgentMgrData) {
    this._pfs = new Map();
    this._gates = new Map();
    data.pfs.forEach(d => {
      this._pfs.set(d.username, new PfAgent(d.username, d.players));
    });
    data.gates.forEach(d => {
      this._gates.set(d.pid, new GateAgent(d));
    });
    this._info = new Array(this._pfs.size + this._gates.size);
    this._pfs.forEach(v => {
      this._info.push({ name: v.id, typ: 'normal', key: `norm-${v.id}` });
    });
    this._gates.forEach(v => {
      this._info.push({ name: v.id, typ: 'fast', key: `fast-${v.id}` });
    })
  }

  public get info(): AgentInfo[] {
    return this._info
  }

  public async addPfAgent(username: string, password: string): Promise<PfAgent> {
    // if local has the pf agent, just return it
    if (this._pfs.has(username)) {
      console.log(`${username} found in local AgentMgr`)
      return this._pfs.get(username)!
    }
    console.log(`${username} not found in local AgentMgr`)
    const res = await invoke<AddPfAgentAck>("plugin:agentmgr|add_pf_agent", { username, password })
    const agent = new PfAgent(res.username, res.players);
    this._pfs.set(res.username, agent);
    return agent;
  }

  public addGateAgent(pid: number): Promise<GateAgent> {
    return new Promise<GateAgent>((resolve, reject) => {
      if (this._gates.has(pid)) {
        resolve(this._gates.get(pid)!);
      } else {
        invoke("plugin:agentmgr|add_gate_agent", { pid }).then(_ => {
          const agent = new GateAgent({ pid });
          this._gates.set(pid, agent);
          resolve(agent);
        }).catch(err => {
          reject(err)
        })
      }
    })
  }

  public async removeGateAgent(pid: number) {

  }

  public async removePfAgent(username: string) {

  }

  public gateAgent(pid: number): GateAgent | undefined {
    return this._gates.get(pid);
  }

  public pfAgent(username: string): PfAgent | undefined {
    return this._pfs.get(username);
  }
}

export {
  AgentMgr
}

export type {
  AgentMgrData, AgentInfo
}