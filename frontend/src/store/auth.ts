import { defineStore } from "pinia"
import { post } from "../js/fetch"

export const useAuth = defineStore("auth", {
  state: () => ({
    user: null,
    logged: false
  }),
  actions: {
    signIn(credentials: { username: string; password: string }) {
      post("/api/login", {})
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged
  }
})
