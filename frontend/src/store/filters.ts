import { isArray } from "lodash"
import { defineStore } from "pinia"
import { FetchError } from "../js/global-types"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { get } from "../js/fetch"

interface Options {
  authors?: string[]
  hasDrafts?: boolean
  timeframes?: Array<{ from: number; to: number }>
  years?: number[]
}

interface Filters {
  active: Options
  available: Options
}

export const useFilters = defineStore("filters", {
  state: () =>
    ({
      active: {},
      available: {}
    } as Filters),
  actions: {
    // set(key: string, value: string | Array<string> | null) {
    //   // If filters dont exist or value is null, remove the active filters completely
    //   if (!this.active[key] || !value) this.active[key] = new Set()

    //   if (typeof value === "string") {
    //     this.active[key].add(value)
    //   } else if (isArray(value)) {
    //     value.map((item) => this.active[key].add(item))
    //   }
    // },
    // del(key: string) {
    //   this.active[key] = new Set()
    // },
    reset() {
      this.active = {}
      this.available = {}
    },
    async fetchOptions() {
      const { addLoading, delLoading } = useLoading()

      addLoading("options")

      // /api/albums/filters?draft=true&from=10&to=100
      // Also can use filters to narrow down possible filtering
      return get(`/api/albums/filters`)
        .then((response: Options) => {
          if (response.timeframes) {
            const all = response.timeframes
              .map((time: any) => {
                return [new Date(time.from * 1000).getFullYear(), new Date(time.to * 1000).getFullYear()]
              })
              .flat()

            // This removes duplicates
            response.years = [...new Set(all)]
          }

          this.available = response
          console.log(response)

          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading("options")
        })
    }
  },
  getters: {
    getAvailableFilters: (state) => (key: string) => Reflect.get(state.available, key) ?? [],
    getActiveFilters: (state) => state.active,
    getActiveFilter: (state) => (key: string) => Reflect.get(state.active, key) ?? []
  }
})
