<template>
	<n-list hoverable clickable>
		<template #header>
			{{ title }}
		</template>

		<n-list-item v-for="(his, i) in historyList" :key="i">
			<n-thing :title="makeTitle(i, his.name)" content-style="margin-top: 10px;">
				<template #description>
					<n-space size="small" style="margin-top: 4px">
						<n-tag :bordered="false" type="info" size="small">
							{{ his.msgid }}
						</n-tag>
					</n-space>
				</template>
				{{ his.payload }}
			</n-thing>
		</n-list-item>

	</n-list>
</template>

<script lang="ts" setup>
import { NTag, NSpace, NList, NListItem, NThing } from 'naive-ui'
import { HistoryData } from 'agentmgr-api';

function makeTitle(msgid: number, msgname: string): string {
	return `[${msgid}] ${msgname}`
}

defineProps<{
	title?: string,
	historyList: HistoryData[],
}>();

</script>