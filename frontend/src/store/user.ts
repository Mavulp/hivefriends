import { isObject } from "lodash"
import { defineStore } from "pinia"
import { get, post, put } from "../js/fetch"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { useLoading } from "./loading"

export interface User {
  key: string
  displayName: string
  username: string
  albumsUploaded: Array<string>
  avatarKey: string
  bannerKey: string
  bio: string
  createdAt: number
}

export interface UserSettings {
  password: string
  displayName: string
  bio: string
  avatarKey: string
  bannerKey: string
  accentColor: string
  featredAlbumKey: string
  private: boolean
  colorTheme: "light-theme" | "dark-theme" | "dark-contrast"
}

export interface State {
  user: User
  users: Array<User> | []
  logged: boolean
  settings: UserSettings
}

export const useUser = defineStore("user", {
  state: () =>
    ({
      user: {},
      users: [],
      logged: false,
      settings: {}
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
          toast.add(error.message, "error")
        })
    },
    async fetchUser(key: string | number) {
      return get(`/api/users/${key}`)
        .then((response) => {
          this.user = response
          localStorage.setItem("user", response.key)
        })
        .catch((error) => {
          if (error.message === "Unauthorized") return "unauth"
        })
    },

    async fetchUsers() {
      return get("/api/users/")
        .then((response) => {
          this.users = response
          this.logged = true

          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
    },
    signOut() {
      this.logged = false
      localStorage.removeItem("bearer_token")
    },

    async fetchSettings() {
      const { addLoading, delLoading } = useLoading()

      addLoading("settings")

      return get("/api/settings/")
        .then((response) => {
          this.settings = response

          const r = document.querySelector(":root")
          if (r) {
            r.removeAttribute("class")
            r.classList.add(response.colorTheme)
          }

          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading("settings")
        })
    },
    async setSetting(key: string, value: any) {
      console.log(key, value)

      return put("/api/settings/", {
        [key]: value
      })
        .then((response) => {
          // @ts-ignore
          this.settings[key] = value

          // @ts-ignore
          this.user[key] = value

          console.log(this.user)
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged,
    getKey: (state) => state.user.key,
    getUsername: (state) => (key?: string | string[] | undefined) => {
      if (!key) return state.user.displayName ?? state.user.username

      const userFromList = state.users.find((item) => item.key === key)
      if (userFromList) return userFromList.displayName ?? userFromList.username

      return state.user.key === key ? state.user.displayName ?? state.user.username : key
    }
  }
})
