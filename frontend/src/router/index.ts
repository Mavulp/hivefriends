import { createRouter, createWebHistory } from "vue-router"

import Login from "./views/Login.vue"
import Home from "./views/Home.vue"

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/:pathMatch(.*)*",
      redirect: { name: "Login" }
    },
    {
      path: "/",
      name: "Login",
      component: Login
    },
    {
      path: "/home",
      name: "Home",
      component: Home
    }
  ]
})

export default router
