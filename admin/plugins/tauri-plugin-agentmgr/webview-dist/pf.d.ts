import { GateAgent } from "./gate";
type AccountPlayer = {
    pid: number;
    name: string;
};
type PfAgentData = {
    username: string;
    players: AccountPlayer[];
};
declare class PfAgent {
    private _username;
    private _players;
    constructor(username: string, players: AccountPlayer[]);
    get id(): string;
    get username(): string;
    get players(): AccountPlayer[];
    createPlayer(name: string): Promise<void>;
    refreshPlayers(): Promise<void>;
    usePlayers(players: AccountPlayer[]): Promise<void>;
    usePlayer(pid: number): Promise<GateAgent>;
}
export { PfAgent };
export type { PfAgentData, AccountPlayer };
