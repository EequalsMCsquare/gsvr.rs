import { AgentMgr, PfAgent, GateAgent } from "./agent";
import type { AgentMgrData, PfAgentData, GateAgentData } from "./agent"
import { invoke } from '@tauri-apps/api/tauri';

let _agentMgr: AgentMgr;

function useAgentMgr(): Promise<AgentMgr> {
	return new Promise<AgentMgr>((resolve, reject) => {
		if (_agentMgr === undefined) {
			console.log("AgentMgr is undefined");
			invoke<AgentMgrData>("agent_mgr_cache").then((res) => {
				const pfmap = new Map();
				const gatemap = new Map();
				res.pfs.forEach(data => {
					pfmap.set(data.username, new PfAgent(data.username, data.players));
				});

				res.gates.forEach(data => {
					gatemap.set(data.pid, new GateAgent(data.pid));
				})
				_agentMgr = new AgentMgr(pfmap, gatemap);
				console.log(_agentMgr);
				resolve(_agentMgr);
			}).catch(err => {
				console.error(err);
				reject(err)
			})
		} else {
			console.log("AgentMgr exist");
			resolve(_agentMgr)
		}
	})
}

export {
	useAgentMgr
}