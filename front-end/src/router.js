import { createWebHistory, createRouter } from 'vue-router'

import MainPage from './components/MainPage.vue'
import LoginPage from './components/LoginPage.vue'
import RegisterPage from './components/RegisterPage.vue'

const routes = [
  { path: '/', name:'main', component: MainPage },
  { path: '/login', name:'login', component: LoginPage },
  { path: '/register', name:'register', component: RegisterPage },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach( (to, from, next) => {
  return next();
});

export default router