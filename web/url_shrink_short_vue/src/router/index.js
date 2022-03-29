import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router'
import Index from '../views/Index.vue'

const routes = [
    {
        path: '/',
        name: 'index',
        component: Index
    },
]

const router = createRouter({
    history: createWebHashHistory(process.env.BASE_URL),
    routes
})

export default router
