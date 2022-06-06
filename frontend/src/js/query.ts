import { Options } from "../store/filters"
import { isEmpty, isArray } from "lodash"

interface QueryOptions {
  draft: boolean
  filters: Options
}

export function query(options: QueryOptions) {
  if (Object.keys(options).length === 0) return ""
  let q = ""

  const add = (key: string, value: any) => (q += `&${key}=${value}`)

  const { draft, filters } = options

  if (!isEmpty(filters)) {
    if (filters.years) {
      const sorted = filters.years.map((year) => Number(year)).sort()

      const from = new Date(sorted.at(0) ?? 0, 0).getTime() / 1000
      const to = new Date(sorted.at(-1) ?? 0, 0).getTime() / 1000

      add("from", from)
      add("to", to)
    } else {
      Object.entries(filters).map(([key, value]) => {
        add(key, isArray(value) ? value.join(",") : value)
      })
    }
  }

  add("draft", draft)

  // Replace first "&" with "?"
  return "?" + q.substring(1)
}
