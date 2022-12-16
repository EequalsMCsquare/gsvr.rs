import { invoke } from "@tauri-apps/api"
import { GateAgent } from "./gate"

type AccountPlayer = {
    pid: number,
    name: string
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

    public get id(): string {
        return `pf-${this.username}`
    }

    public get username(): string {
        return this._username
    }

    public get players(): AccountPlayer[] {
        return this._players;
    }

    public async createPlayer(name: string) {
        let player = await invoke<AccountPlayer>("plugin:agentmgr|pf_create_player", { username: this._username, name });
        this._players.push(player);
    }

    public async refreshPlayers() {
        this._players.splice(0, this._players.length);
        this._players.push(...await invoke<AccountPlayer[]>("plugin:agentmgr|pf_refresh_players", { username: this._username }));
    }

    public async usePlayers(players: AccountPlayer[]) {
        this._players = players;
        await this.refreshPlayers()
    }

    public async usePlayer(pid: number): Promise<GateAgent> {
        await invoke("plugin:agentmgr|pf_use_player", { username: this._username, pid });
        return new GateAgent({ pid });
    }
}

export {
    PfAgent
}
export type {
    PfAgentData, AccountPlayer
}