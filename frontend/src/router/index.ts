import { createRouter, createWebHistory, NavigationGuardNext, RouteLocationNormalized } from "vue-router"

import Login from "./views/Login.vue"
import Home from "./views/Home.vue"
import AlbumDetail from "./views/AlbumDetail.vue"
import ImageDetail from "./views/ImageDetail.vue"
import AlbumList from "./views/AlbumList.vue"
import UserPage from "./views/User.vue"
import AlbumUpload from "./views/AlbumUpload.vue"

// Subchildren for user pages
import UserAlbums from "../components/user/UserAlbums.vue"
import UserProfile from "../components/user/UserProfile.vue"
import UserSettings from "../components/user/UserSettings.vue"

import { useUser } from "../store/user"
// import { useBread } from "../store/bread"

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
        title: "Sign In",
        redirectOnAuth: "/home"
      }
    },
    {
      path: "/home",
      name: "Home",
      component: Home,
      meta: {
        title: "Home",
        requiresAuth: true
      }
    },
    {
      path: "/albums",
      name: "Albums",
      component: AlbumList,
      meta: {
        title: "All Albums",
        requiresAuth: true
      }
    },
    {
      path: "/album/:id",
      name: "AlbumDetail",
      component: AlbumDetail,
      meta: {
        title: "Album Detail",
        requiresAuth: true
      }
    },
    {
      path: "/album/:album/image/:image",
      name: "ImageDetail",
      component: ImageDetail,
      meta: {
        title: "Image Detail",
        requiresAuth: true,
        disableNav: true
      }
    },
    {
      path: "/upload",
      name: "Upload",
      component: AlbumUpload,
      meta: {
        title: "Upload",
        requiresAuth: true
      }
    },
    {
      path: "/user/",
      name: "User",
      component: UserPage,
      children: [
        {
          path: "/user/:id/profile",
          name: "UserProfile",
          component: UserProfile,
          meta: {
            title: "User Profile",
            requiresAuth: true
          }
        },
        {
          path: "/user/settings",
          name: "UserSettings",
          component: UserSettings,
          meta: {
            title: "User Settings",
            requiresAuth: true
          }
        },
        {
          path: "/user/:id/albums",
          name: "UserAlbums",
          component: UserAlbums,
          meta: {
            title: "User Albums",
            requiresAuth: true
          }
        }
      ]
    }
  ]
})

/**
 * Router Guards
 */

function _clearUser(next: NavigationGuardNext) {
  localStorage.removeItem("user")
  localStorage.removeItem("bearer_token")

  return next({ name: "Login" })
}

router.beforeResolve(async (to, from, next) => {
  const auth = useUser()

  if (to.meta.requiresAuth) {
    const token = localStorage.getItem("bearer_token")
    const key = localStorage.getItem("user")

    if (!token || !key) {
      return _clearUser(next)
    } else {
      // Verify if token is expired
      const verify = await auth.fetchUser(key)

      if (verify === "unauth") return _clearUser(next)
    }
  }

  // Handle authentication
  next()
})

export default router
