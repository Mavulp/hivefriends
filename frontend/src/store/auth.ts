import { defineStore } from "pinia"
import { post } from "../js/fetch"
import { useToast } from "./toast"

interface User {
  username: string
  avatar: string
}

export const useAuth = defineStore("auth", {
  state: () => ({
    user: { username: "test" },
    logged: false
  }),
  actions: {
    async signIn(credentials: { username: string; password: string }) {
      return post("/api/login", credentials)
        .then((res) => {
          localStorage.setItem("bearer_token", res.bearerToken)
          this.logged = true
        })
        .catch((error) => {
          const toast = useToast()
          toast.add(String(error), "error")
        })
    },
    signInToken(token: string) {
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
