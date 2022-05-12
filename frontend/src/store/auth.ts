import { isObject } from "@vue/shared"
import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useToast } from "./toast"

interface State {
  user: User
  logged: boolean
}

interface User {
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
      logged: false
    } as State),
  actions: {
    async signIn(credentials: { username: string; password: string }) {
      return post("/api/login", credentials)
        .then(async (res) => {
          localStorage.setItem("bearer_token", res.bearerToken)

          await this.fetchUser(res.userKey)

          this.logged = true
        })
        .catch((error) => {
          const toast = useToast()
          toast.add(isObject(error) ? error.message : String(error), "error")
        })
    },
    async fetchUser(key: string | number) {
      return get(`/api/users/${key}`).then((response) => {
        console.log(response)
        this.user = response
        localStorage.setItem("user", JSON.stringify(response))
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
    isLoggedIn: (state) => state.logged
  }
})
