import { isObject } from "lodash"
import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"

interface State {
  user: User
  users: Array<User> | []
  logged: boolean
}

interface User {
  key: string
  username: string
  avatar: string
  albumsUploaded: Array<string>
  avatarUrl: string
  bio: string
  createdAt: number
}

export const useAuth = defineStore("auth", {
  state: () =>
    ({
      user: {},
      users: [],
      logged: false
    } as State),
  actions: {
    async signIn(credentials: { username: string; password: string }) {
      return post("/api/login/", credentials)
        .then(async (res) => {
          localStorage.setItem("bearer_token", res.bearerToken)

          await this.fetchUser(res.userKey)

          this.logged = true
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(isObject(error) ? error.message : String(error), "error")
        })
    },
    async fetchUser(key: string | number) {
      return get(`/api/users/${key}`)
        .then((response) => {
          this.user = response
          localStorage.setItem("user", JSON.stringify(response))
        })
        .catch((error) => {
          if (error.message === "Unauthorized") return "unauth"
        })
    },

    async fetchUsers() {
      return get("/api/users/")
        .then((response) => {
          this.users = response
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(isObject(error) ? error.message : String(error), "error")
        })
    },
    signInUserFromStorage(user: User) {
      this.user = user
      this.logged = true
    },
    signOut() {
      this.logged = false
      localStorage.removeItem("bearer_token")
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged,
    getKey: (state) => state.user.key,
    getUsername: (state) => (key: string | string[]) => {
      if (!key) return state.user.username

      const userFromList = state.users.find((item) => item.key === key)
      if (userFromList) return userFromList.username

      return state.user.key === key ? state.user.username : key
    }
  }
})
