import { defineStore } from "pinia"
import { post } from "../js/fetch"
import { useToast } from "./toast"

export const useAuth = defineStore("auth", {
  state: () => ({
    user: null,
    logged: false
  }),
  actions: {
    async signIn(credentials: { username: string; password: string }) {
      post("/api/login", credentials)
        .then((res) => {
          localStorage.setItem("bearer_token", res)
        })
        .catch((error) => {
          const toast = useToast()
          toast.add(String(error), "error")
        })
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged
  }
})
