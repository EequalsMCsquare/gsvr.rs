<template>
	<n-card title="Add Client">
		<n-tabs animated :on-update:value="(val) => kind = val" :value="kind" size="large"
			justify-content="space-evenly">
			<n-tab-pane name="fast" tab="Fast Login">
				<n-form>
					<n-form-item-row label="Player ID">
						<n-input v-model:value="playerid" :allow-input="onlyNumber" />
					</n-form-item-row>
				</n-form>
				<n-button type="primary" block secondary strong
					:on-click="() => emits('on-click-add', kind, parseInt(playerid), username, password)">
					Add
				</n-button>
			</n-tab-pane>

			<n-tab-pane name="normal" tab="Login">
				<n-form>
					<n-form-item-row label="Username">
						<n-input v-model:value="username" />
					</n-form-item-row>
					<n-form-item-row label="Password">
						<n-input v-model:value="password" type="password" />
					</n-form-item-row>
				</n-form>
				<n-button type="primary" block secondary strong
					:on-click="() => emits('on-click-add', kind, parseInt(playerid), username, password)">
					Add
				</n-button>
			</n-tab-pane>
		</n-tabs>
	</n-card>
</template>


<script setup lang="ts">
import { NCard, NTabs, NTabPane, NForm, NFormItemRow, NInput, NButton, NInputNumber } from 'naive-ui';
import { ref } from 'vue';
import type { Ref } from 'vue';

const playerid = ref("");
const username = ref("");
const password = ref("");
const kind: Ref<"fast" | "normal"> = ref("fast");

const onlyNumber = (value: string) => {
	if (!value) {
		return true
	} else if ( /^\d+$/.test(value)) {
		return parseInt(value) >= 0 
	} else {
		return false
	}
};

const emits = defineEmits<{
	(e: 'on-click-add',
		kind: "fast" | "normal",
		playerid?: number,
		username?: string,
		password?: string
	): void
}>();

</script>