import { NavigationGuardNext, RouteLocationNormalized } from "vue-router"
import { useUser } from "../../store/user"
import { useBread } from "../../store/bread"
import { isArray } from "lodash"

function _clearUser(next: NavigationGuardNext) {
  localStorage.removeItem("user")
  localStorage.removeItem("bearer_token")

  return next({ name: "Login" })
}

export default async function (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) {
  const auth = useUser()
  const bread = useBread()
  bread.clear()

  const token = localStorage.getItem("bearer_token")
  const key = localStorage.getItem("user")

  // Save token
  if (to.name === "PublicAlbumDetail") {
    const _public_token: string = isArray(to.params.token) ? to.params.token[0] : to.params.token
    auth.public_token = _public_token
  }

  // Lock user into album details & image detail routes
  if (auth.public_token) {
    if (to.name == "ImageDetail" || to.name == "PublicAlbumDetail") {
      if (token && key && token) {
        // This will only happen when a public album detail is loaded
        // but user has been logged in previously with a valid user & bearer token
        // We want to "sign in" the user and show the normal album view

        auth.public_token = undefined
        return next({
          name: "AlbumDetail",
          params: { id: to.params.id }
        })
      }

      return next()
    } else {
      return next(from.path)
    }
  }

  if (to.meta.requiresAuth) {
    if (!token || !key) {
      return _clearUser(next)
    } else if (!auth.logged) {
      await auth.fetchUser(key)
    }
  }

  // else if (to.name !== "Login") {
  //   console.log(token, key, auth.public_token)

  //   // Abort public paths and redirect to album instead
  //   if (token && key && auth.public_token) {

  //   }

  //   // Public route
  //   if (auth.public_token) {
  //     return next()
  //   }
  // }

  next()
}
