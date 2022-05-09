import { createRouter, createWebHistory } from "vue-router"

import Login from "./views/Login.vue"
import Home from "./views/Home.vue"
import AlbumDetail from "./views/AlbumDetail.vue"
import ImageDetail from "./views/ImageDetail.vue"
import Albums from "./views/Albums.vue"

/**
 * Router Setup
 */
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
    },
    {
      path: "/album/all",
      name: "Albums",
      component: Albums,
      meta: {
        title: "All Albums"
      }
    },
    {
      path: "/album/:id",
      name: "Albums",
      component: AlbumDetail,
      meta: {
        // This will be updated on route load
        title: "Album Detail"
      }
    },
    {
      path: "/album/:id/image/:image",
      name: "Albums",
      component: ImageDetail,
      meta: {
        // This will be also update
        title: "Image Detail"
      }
    }
  ]
})

/**
 * Router Guards
 */
router.afterEach((to) => {
  document.title = `${to.meta.title} :: hi!friends`
})

export default router
