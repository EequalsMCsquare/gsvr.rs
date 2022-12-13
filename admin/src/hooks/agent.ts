import { invoke } from '@tauri-apps/api/tauri'

type AccountPlayer = {
	pid: number,
	name: string
}

type AddPfClientAck = {
	username: string,
	players: AccountPlayer[],
}

type GateAgentData = {
	pid: number
}

class GateAgent {
	private _pid: number

	constructor(pid: number) {
		this._pid = pid;
	}

	public get pid(): number {
		return this._pid
	}

	public async send(msg: JSON) {
		await invoke("gate_send", { pid: this._pid, msg })
	}

	public recv(): Promise<JSON> {
		return invoke<JSON>("gate_recv", { pid: this._pid })
	}
}

type PfAgentData = {
	username: string,
	players: AccountPlayer[]
}

class PfAgent {
	private _username: string
	private _players: AccountPlayer[]

	constructor(username: string, players: AccountPlayer[]) {
		this._username = username;
		this._players = players;
	}

	public get username(): string {
		return this._username
	}

	public list_players(): AccountPlayer[] {
		return this._players;
	}

	public async create_player(name: string) {
		let player = await invoke<AccountPlayer>("pf_create_player", { username: this._username, name });
		this._players.push(player);
	}

	public async refresh_players() {
		this._players = await invoke<AccountPlayer[]>("pf_refresh_players", { username: this._username });
	}

	public async use_player(pid: number): Promise<GateAgent> {
		await invoke("pf_use_player", { username: this._username, pid });
		return new GateAgent(pid);
	}
}

type AgentMgrData = {
	pfs: PfAgentData[],
	gates: GateAgentData[],
}


class AgentMgr {
	private _pfs: Map<string, PfAgent>
	private _gates: Map<number, GateAgent>

	constructor(pfs: Map<string, PfAgent>, gates: Map<number, GateAgent>) {
		this._pfs = pfs;
		this._gates = gates;
	}

	public async addPfAgent(username: string, password: string): Promise<PfAgent> {
		// if local has the pf agent, just return it
		if (this._pfs.has(username)) {
			console.log(`${username} found in local AgentMgr`)
			return new Promise<PfAgent>((resolve) => {
				resolve(this._pfs.get(username)!);
			});
		}
		console.log(`${username} not found in local AgentMgr`)
		const res = await invoke<AddPfClientAck>("add_pf_agent", { username, password })
		return new PfAgent(res.username, res.players)
	}

	public async addGateAgent(pid: number): Promise<GateAgent> {
		if (this._gates.has(pid)) {
			return new Promise(resolve => {
				resolve(this._gates.get(pid)!);
			});
		}
		await invoke("add_gate_agent", { pid });
		return new GateAgent(pid)
	}

	public async removeGateAgent(pid: number) {

	}

	public async removePfAgent(username: string) {

	}
}

export {
	GateAgent, PfAgent, AgentMgr
}
export type { GateAgentData, PfAgentData, AgentMgrData }
