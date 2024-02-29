import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import {createRouter, createWebHashHistory} from "vue-router";
import LoadProject from "./components/LoadProject.vue";
import Project from "./components/Project.vue";

const routes = [
    {
        path: "/",
        component: LoadProject,
    },
    {
        path: "/project/:name",
        component: Project,
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

createApp(App).use(router).mount("#app");
