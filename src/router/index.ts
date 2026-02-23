import { createRouter, createWebHistory } from "vue-router";
import Downloads from "../views/Downloads.vue";
import Settings from "../views/Settings.vue";

const routes = [
    { path: "/downloads", name: "Downloads", component: Downloads },
    { path: "/settings", name: "Settings", component: Settings },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;
