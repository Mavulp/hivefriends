import { isArray } from "lodash"
import { defineStore } from "pinia"
import { FetchError } from "../js/global-types"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { get } from "../js/fetch"
import { query } from "../js/query"

export interface Options {
  authors?: string[]
  hasDrafts?: boolean
  timeframes?: Array<{ from: number; to: number }>
  years?: number[] | string[]
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
    clear() {
      this.active = {}
    },
    reset() {
      this.active = {}
      this.available = {}
    },
    async fetchOptions(filters?: Options) {
      const { addLoading, delLoading } = useLoading()

      addLoading("options")

      const q = query({ filters })

      // /api/albums/filters?draft=true&from=10&to=100
      // Also can use filters to narrow down possible filtering
      return get(`/api/albums/filters${q}`)
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
