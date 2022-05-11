import { defineStore } from "pinia"
import { post } from "../js/fetch"

export const useAuth = defineStore("auth", {
  state: () => ({
    user: null,
    logged: false
  }),
  actions: {
    signIn(credentials: { username: string; password: string }) {
      post("/api/login", credentials)
        .then((res) => {
          console.log(res)
        })
        .catch((error) => console.log(error))
    }
  },
  getters: {
    isLoggedIn: (state) => state.logged
  }
})
