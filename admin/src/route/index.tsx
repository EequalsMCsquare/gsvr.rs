import { createRouter, createWebHashHistory } from "vue-router";

const ClientVue = () => import("../views/Client.vue");
const GmVue = () => import("../views/Gm.vue");
const BenchVue = () => import("../views/Bench.vue");

export default createRouter({
	history: createWebHashHistory(),
	routes: [
		{
			path: "/",
			redirect: "/client"
		},
		{
			path: "/client",
			name: "client-page",
			component: ClientVue
		},
		{
			path: "/gm",
			name: "gm-page",
			component: GmVue
		},
		{
			path: "/bench",
			name: "bench-page",
			component: BenchVue
		}
	]
})