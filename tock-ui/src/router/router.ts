import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import AppManagementView from '../views/AppManagementView.vue';
import TerminalView from '../views/TerminalView.vue';


const routes = [
  { path: '/', redirect: '/connected-devices' },
  { path: '/connected-devices', name: 'CONNECTED DEVICES', component: HomeView },
  { path: '/app-management', name: 'APP MANAGEMENT', component: AppManagementView },
  { path: '/terminal', name: 'TERMINAL', component: TerminalView },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;