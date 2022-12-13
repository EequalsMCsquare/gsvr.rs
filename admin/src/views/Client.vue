<template>
	<n-tabs v-model:value="agent!.name" type="card" addable tab-style="min-width: 80px;" @add="showAddClient = true"
		@close="handleClose">
		<n-tab-pane v-for="panel in panels" closable :key="panel.name" :tab="panel.toString()" :name="panel.name">
			{{ panel }}
		</n-tab-pane>
	</n-tabs>
	<n-modal style="width:500px" size="huge" :bordered="true" closable v-model:show="showAddClient">
		<add-client @on-click-add="handleAddClient" />
	</n-modal>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import type { Ref } from 'vue';
import { useMessage, NTabs, NTabPane, NModal } from 'naive-ui'
import AddClient from '../components/AddClient.vue';
import { useAgentMgr} from '../hooks';
import { PfAgent, GateAgent } from '../hooks/agent';

type AgentInfo = {
	name: string | number, typ: 'fast' | 'normal'
};

let agent: Ref<AgentInfo> = ref({ name: "", typ: "fast" } as AgentInfo)
const message = useMessage()
const panels: AgentInfo[] = reactive([])
const showAddClient = ref(false)
const agentMgr = await useAgentMgr();

function handleClose(value: string) {
	message.info('关掉 ' + value)
	const index = panels.findIndex((v) => value === v.name)
	const info = panels[index];
	if (info.typ === 'fast') {
		agentMgr.removeGateAgent(info.name as number).then(_ => {
			panels.splice(index, 1)
			if (agent.value?.name === value) {
				agent.value = panels[index]
			}
		})
	} else {
		agentMgr.removePfAgent(info.name as string).then(_ => {
			panels.splice(index, 1)
			if (agent.value?.name === value) {
				agent.value = panels[index]
			}
		})
	}
}

function handleAddClient(kind: "normal" | "fast", playerid?: number, username?: string, password?: string) {
	console.log(kind, playerid, username, password);
	console.log(agentMgr);
	if (kind == 'normal') {
		agentMgr.addPfAgent(username!, password!).then((res: PfAgent) => {
			agent.value = { name: res.username, typ: 'normal' } as AgentInfo;
			panels.push(agent.value);
		})
	} else {
		agentMgr.addGateAgent(playerid!).then((res: GateAgent) => {
			agent.value = { name: res.pid, typ: 'fast' } as AgentInfo;
			panels.push(agent.value);
		})
	}
	showAddClient.value = false
}
</script>