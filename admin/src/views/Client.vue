<template>
	<n-tabs :value="currAgent" type="card" addable tab-style="min-width: 80px;" @add="showAddClient = true" @update:value="handleSwitch"
		@close="handleClose">
		<n-tab-pane v-for="agent in agents" closable :key="agent.key" :tab="agent.name" :name="agent.key">
			<gate-agent-vue v-if="agent.typ === 'fast'" :agent="agentMgr.gateAgent(agent.name as number)!" />
			<pf-agent-vue v-else-if="agent.typ === 'normal'" />
		</n-tab-pane>
	</n-tabs>
	<n-modal style="width:500px" size="huge" :bordered="true" closable v-model:show="showAddClient">
		<add-agent @on-click-add="handleAddClient" />
	</n-modal>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { NTabs, NTabPane, NModal } from 'naive-ui'
import { useAgentMgr } from 'agentmgr-api';
import { PfAgent, GateAgent, AgentInfo } from 'agentmgr-api';

import AddAgent from '../components/AddAgent.vue';
import GateAgentVue from '../components/GateAgent.vue';
import PfAgentVue from '../components/PfAgent.vue';


let currAgent = ref("");
const showAddClient = ref(false)

const agentMgr = await useAgentMgr();
console.log(agentMgr.info);
const agents: AgentInfo[] = reactive(agentMgr.info !== undefined ? agentMgr.info : []);

function handleClose(value: string) {
	const index = agents.findIndex((v) => value === v.key)
	const info = agents[index];
	if (info.typ === 'fast') {
		agentMgr.removeGateAgent(info.name as number).then(_ => {
			agents.splice(index, 1)
			if (currAgent.value === value) {
				currAgent.value = agents[index].key
			}
		})
	} else {
		agentMgr.removePfAgent(info.name as string).then(_ => {
			agents.splice(index, 1)
			if (currAgent.value! === value) {
				currAgent.value = agents[index].key
			}
		})
	}
}

function handleSwitch(tabname: string) {
	currAgent.value = tabname;
}

function handleAddClient(kind: "normal" | "fast" | "reg", playerid?: number, username?: string, password?: string, email?: string, phone?: string) {
	if (kind === 'normal') {
		agentMgr.addPfAgent(username!, password!).then((res: PfAgent) => {
			const key = `norm-${res.username}`;
			agents.push({ name: res.username, typ: 'normal', key } as AgentInfo);
			currAgent.value = key;
		})
	} else if (kind === "fast") {
		agentMgr.addGateAgent(playerid!).then((res: GateAgent) => {
			const key = `fast-${res.id}`;
			agents.push({ name: res.pid, typ: 'fast', key } as AgentInfo);
			currAgent.value = key;
		}).catch(err => {
			console.error(err);
		})
	} else {

	}
	showAddClient.value = false
}
</script>