import { createRouter, createWebHistory } from 'vue-router'
import MainConsole from '../views/MainConsole.vue'
import Overlay from '../views/Overlay.vue'

const routes = [
  { 
    path: '/', 
    name: 'Main',
    component: MainConsole 
  },
  { 
    path: '/overlay', 
    name: 'Overlay',
    component: Overlay
  }
]

export const router = createRouter({
  history: createWebHistory(),
  routes
})