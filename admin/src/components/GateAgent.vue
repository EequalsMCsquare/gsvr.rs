<template>
  <n-grid :x-gap="12" :y-gap="8" :cols="4">
    <n-grid-item span="2">
      <HistoryVue title="SC History" :history-list="scHistory" :style="{ height: '66.5vh', overflow: 'scroll' }" />
    </n-grid-item>

    <n-grid-item span="2">
      <HistoryVue title="CS History" :history-list="csHistory" :style="{ height: '66.5vh', overflow: 'scroll' }" />
    </n-grid-item>

    <n-grid-item span="1">
      <n-select filterable :options="options" :value="pbName" @update:value="selectPbName" />
    </n-grid-item>
    <n-grid-item span="3">
      <n-input-group style="height: 26vh;">
        <n-input type="textarea" v-model:value="csInput" />
        <n-button-group vertical>
          <n-button type="primary" ghost style="height: 50%; min-width: 90px;" @click="csSend">
            Send
          </n-button>
          <n-button type="warning" ghost style="height: 50%;" disabled>
            {{ping.ping}}ms
          </n-button>
        </n-button-group>
      </n-input-group>
    </n-grid-item>
  </n-grid>
</template>


<script lang="ts" setup>
import { NGrid, NGridItem, NSelect, NInput, NInputGroup, NButton, NButtonGroup, NThing } from 'naive-ui';
import HistoryVue from './History.vue';
import { useHint } from 'hint-api';
import { GateAgent, HistoryData, PingInfo } from 'agentmgr-api'
import { ref, onBeforeUnmount, reactive } from 'vue';

const props = defineProps<{
  agent: GateAgent
}>();

const csHistory: HistoryData[] = reactive(new Array);
const scHistory: HistoryData[] = reactive(new Array);
const ping: PingInfo = reactive({ping: 0})

await props.agent.useHistory(csHistory, scHistory);
await props.agent.usePing(ping);
const hint = await useHint();
const pbName = ref("");
const csInput = ref("");
const options = hint.names.map((v: string) => {
  return { value: v, label: v }
});

await props.agent.listen();
onBeforeUnmount(async () => {
  await props.agent.unlisten()
})

function selectPbName(name: string) {
  pbName.value = name;
  const payload = hint.get_payload(name)!;
  csInput.value = payload;
}

async function csSend() {
  let payload = `{ "${pbName.value}": ${csInput.value} }`;
  await props.agent.send(payload);
  pbName.value = "";
  csInput.value = "";
}



</script>