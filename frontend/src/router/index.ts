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
      path: "/login",
      name: "Login",
      component: Login,
      meta: {
        title: "Sign In"
      }
    },
    {
      path: "/home",
      name: "Home",
      component: Home,
      meta: {
        title: "Home"
      }
    }
  ]
})

router.afterEach((to) => {
  document.title = `${to.meta.title} :: hi!friends`
})

export default router
