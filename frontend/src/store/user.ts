import { isNil } from "lodash"
import { defineStore } from "pinia"
import { get, post, put } from "../js/fetch"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { useLoading } from "./loading"

export interface User {
  // key: string
  username: string
  displayName: string
  bio: string
  avatarKey: string
  bannerKey: string
  accentColor: string
  featredAlbumKey: string
  met: Array<string> | []
  albumsUploaded: Array<string> | []
  createdAt: number
  country: string | null
}

export interface UserSettings {
  password: string
  displayName: string
  bio: string
  avatarKey: string
  bannerKey: string
  accentColor: string
  featredAlbumKey: string
  // private: boolean
  colorTheme: "light-theme" | "dark-normal" | "dark-contrast"
}

export interface State {
  user: User
  users: Array<User> | []
  logged: boolean
  settings: UserSettings
  public_token: string | undefined
}

export const useUser = defineStore("user", {
  state: () =>
    ({
      user: {},
      users: [],
      logged: false,
      settings: {},
      public_token: undefined
    } as State),
  actions: {
    async signIn(credentials: { username: string; password: string }) {
      const { addLoading, delLoading } = useLoading()

      addLoading("login")

      return post("/api/login", credentials)
        .then(async (res) => {
          localStorage.setItem("bearer_token", res.bearerToken)

          await this.fetchUser(res.username)

          this.logged = true
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("login"))
    },
    async fetchUser(username: string | number, notme?: boolean) {
      const { addLoading, delLoading } = useLoading()

      addLoading("user")

      return get(`/api/users/${username}`)
        .then((response) => {
          if (notme) {
            const index = this.users.findIndex((item: User) => item.username === username)
            if (index > -1) {
              this.users[index] = response
            }
          } else {
            this.user = response
            this.logged = true
            // Set app accent
            document.documentElement.style.setProperty("--color-highlight", response.accentColor)
            localStorage.setItem("user", response.username)
          }
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading("user")
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
          if (this.logged) {
            const toast = useToast()
            toast.add(error.message, "error")
          }
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
          if (this.logged) {
            const toast = useToast()
            toast.add(error.message, "error")
          }
        })
        .finally(() => {
          delLoading("settings")
        })
    },
    async setSetting(key: string, value: any) {
      return put("/api/settings/", {
        [key]: value
      })
        .then(() => {
          Reflect.set(this.settings, key, value)

          this.fetchUsers()
          this.fetchUser(this.user.username)
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
    },

    async changePassword(form: { old: string | number; new: string | number }) {
      const toast = useToast()

      return put("/api/settings/password", form)
        .then(() => {
          toast.add("Succesfully updated password. Make sure you remember it", "success")
        })
        .catch((error: FetchError) => {
          toast.add(error.message, "error")
        })
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged,
    getKey: (state) => state.user.username,
    getUser: (state) => (username: string, field?: string) => {
      const u =
        state.users.find((item) => item.username === username) ||
        state.users.find((item) => item.displayName === username)
      if (!u) return null

      if (!field) return u

      return Reflect.get(u, field)
    },
    getUsername:
      (state) =>
      (username?: string | string[] | undefined): string => {
        if (Array.isArray(username)) username = username[0]

        if (!username) return state.user.displayName ?? state.user.username

        const userFromList = state.users.find((item) => item.username === username)
        if (userFromList) return userFromList.displayName ?? userFromList.username

        return state.user.username === username ? state.user.displayName ?? state.user.username : username
      }
    // getUsers: (state) => {
    //   if (state.users)
    // },
  }
})
