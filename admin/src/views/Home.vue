<template>
	<n-space vertical>
		<n-layout has-sider>
			<n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed"
				show-trigger @collapse="collapsed = true" @expand="collapsed = false">
				<n-menu v-model:value="activeKey" :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22"
					:options="menuOptions" 
					:on-update:value="updateMenu"
					/>
			</n-layout-sider>
			<n-layout>
				<Suspense>
					<template #default>
						<router-view />
					</template>
					<template #fallback>
						Loading...
					</template>
				</Suspense>
			</n-layout>
		</n-layout>
	</n-space>
</template>

<script lang="ts">
import ClientPage from "./Client.vue";
import { RouterView, useRouter } from "vue-router";
import type { MenuOption } from 'naive-ui';
import { Component, defineComponent, ref, h, Suspense } from 'vue';
import { NSpace, NLayout, NLayoutSider, NMenu, NIcon } from "naive-ui";
import {UserMultiple, CloudServiceManagement, Network3} from "@vicons/carbon";

function renderIcon (icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
	{
		label: 'Client',
		key: 'client',
		icon: renderIcon(UserMultiple)
	},
	{
		label: 'GM',
		key: 'gm',
		icon: renderIcon(CloudServiceManagement)
	},
	{
		label: 'Bench',
		key: 'bench',
		icon: renderIcon(Network3)
	}
]

export default defineComponent({
	setup() {
		const router = useRouter();

		function updateMenu(key: string, _item: MenuOption) {
			router.push({path: `/${key}`})
		}

		return {
			activeKey: ref<string | null>(null),
			collapsed: ref(false),
			updateMenu,
			menuOptions
		}
	},
	components: {
		ClientPage,
		NSpace,
		NLayout,
		NLayoutSider,
		NMenu
	}
})
</script>