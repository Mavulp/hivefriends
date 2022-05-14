import { createRouter, createWebHistory, RouteLocationNormalized } from "vue-router"

import Login from "./views/Login.vue"
import Home from "./views/Home.vue"
import AlbumDetail from "./views/AlbumDetail.vue"
import ImageDetail from "./views/ImageDetail.vue"
import AlbumList from "./views/AlbumList.vue"
import User from "./views/User.vue"
import AlbumUpload from "./views/AlbumUpload.vue"

// Subchildren for user pages
import UserAlbums from "../components/user/UserAlbums.vue"
import UserProfile from "../components/user/UserProfile.vue"
import UserSettings from "../components/user/UserSettings.vue"

import { useAuth } from "../store/auth"
import { useBread } from "../store/bread"

import { isEmpty } from "lodash"

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
        breadcrumb: "Sign in to hi!friends",
        redirectOnAuth: "/home"
      }
    },
    {
      path: "/home",
      name: "Home",
      component: Home,
      meta: {
        title: "Home",
        breadcrumb: "Latest albums",
        requiresAuth: true
      }
    },
    {
      path: "/albums",
      name: "Albums",
      component: AlbumList,
      meta: {
        title: "All Albums",
        breadcrumb: "All albums from all users",
        requiresAuth: true
      }
    },
    {
      path: "/album/:id",
      name: "AlbumDetail",
      component: AlbumDetail,
      meta: {
        title: "_album_name_",
        breadcrumb: "_album_name_",
        requiresAuth: true
      }
    },
    {
      path: "/album/:id/image/:image",
      name: "ImageDetail",
      component: ImageDetail,
      meta: {
        title: "_id_",
        breadcrumb: "_id_",
        requiresAuth: true
      }
    },
    {
      path: "/upload",
      name: "Upload",
      component: AlbumUpload,
      meta: {
        title: "Upload",
        breadcrumb: "Upload a new album",
        requiresAuth: true
      }
    },
    {
      path: "/user/",
      name: "User",
      component: User,
      children: [
        {
          path: "/user/:id/profile",
          name: "UserProfile",
          component: UserProfile,
          meta: {
            title: "_id_'s Profile",
            breadcrumb: "_id_'s Profile",
            requiresAuth: true
          }
        },
        {
          path: "/user/settings",
          name: "UserSettings",
          component: UserSettings,
          meta: {
            title: "User Settings",
            breadcrumb: "User Settings",
            requiresAuth: true
          }
        },
        {
          path: "/user/:id/albums",
          name: "UserAlbums",
          component: UserAlbums,
          meta: {
            title: "_id_'s Albums",
            breadcrumb: "All albums uploaded by _id_",
            requiresAuth: true
          }
        }
      ]
    }
  ]
})

function _formatMeta(field: string, route: RouteLocationNormalized) {
  const auth = useAuth()

  let val = `${route.meta[field]}`

  if (!val) return ""

  if (isEmpty(route.params)) return val

  for (const key of Object.keys(route.params)) {
    if (!val.includes(key)) continue

    val = val.replace(`_${key}_`, auth.getUsername(`${route.params[key]}`))
  }

  return val
}

/**
 * Router Guards
 */
router.afterEach((to) => {
  const bread = useBread()
  bread.set(_formatMeta("breadcrumb", to), _formatMeta("title", to))
})

router.beforeResolve(async (to, from, next) => {
  if (to.meta.requiresAuth) {
    const token = localStorage.getItem("bearer_token")
    const user = localStorage.getItem("user")

    if (!token || !user) {
      localStorage.removeItem("user")
      localStorage.removeItem("bearer_token")

      return next({ name: "Login" })
    } else {
      const auth = useAuth()
      auth.signInUserFromStorage(JSON.parse(user))
    }
  }

  // Handle authentication
  next()
})

export default router
