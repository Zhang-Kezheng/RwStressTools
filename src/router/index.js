import { createMemoryHistory, createRouter } from 'vue-router'

import SendView from '../pages/SendView.vue'
import ReceiveView from '../pages/ReceiveView.vue'
import SettingView from '../pages/SettingView.vue'
import HistoryView from '../pages/HistoryView.vue'
const routes = [
    { path: '/SendView', component: SendView , meta: {keepAlive: true}},
    { path: '/ReceiveView', component: ReceiveView , meta: {keepAlive: true}},
    { path: '/SettingView', component: SettingView, meta: {keepAlive: true} },
    { path: '/HistoryView', component: HistoryView , meta: {keepAlive: true}},
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router