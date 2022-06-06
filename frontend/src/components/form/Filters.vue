<script setup lang="ts">
import { onBeforeMount, computed, ref } from "vue"
import { useFilters } from "../../store/filters"
import { useLoading } from "../../store/loading"

import InputSelect from "./InputSelect.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import { useUser } from "../../store/user"

const { getLoading } = useLoading()
const filters = useFilters()
const user = useUser()

const emit = defineEmits<{
  (e: "call"): void
}>()

onBeforeMount(() => {
  filters.fetchOptions()
})

/**
 * Authors
 */

const authorOptions = computed(() => {
  const authors = filters.getAvailableFilters("authors")

  return authors.map((item: string) => ({
    label: user.getUsername(item),
    value: item
  }))
})

const authors = computed<Array<string>>({
  get: () => filters.getActiveFilter("authors"),
  set(value) {
    // setting
  }
})

/**
 * Years
 */
const yearsOptions = computed(() => {
  const years = filters.getAvailableFilters("years")

  return years.map((year: number) => {
    return {
      label: year.toString(),
      value: year.toString()
    }
  })
})

const years = computed<Array<string>>({
  get: () => filters.getActiveFilter("years"),
  set(value) {
    // setting
  }
})

/**
 * Timeframe
 */

// const [timeframeFromOptions, timeframetoOptions] = computed(() => {
//   const timeframes = filters.getAvailableFilters('timeframes')

//   const from = []
//   const to = []

//   return timeframes.map((timeframe: { from: number; to: number }) => {
//     from.push({
//       label: from.toString()
//     })
//   })
// })
</script>

<template>
  <div class="filters">
    <h6>Filters</h6>

    <slot />

    <div class="filters-loading" v-if="getLoading('options')">
      <LoadingSpin class="dark center-parent" />
    </div>

    <div class="form-filters" v-else>
      <!-- Authors -->

      <InputSelect
        v-if="authorOptions"
        label="Authors"
        placeholder="Filter by authors"
        :options="authorOptions"
        v-model:selected="authors"
      />

      <InputSelect
        v-if="yearsOptions"
        label="Years"
        placeholder="Filter albums by event years"
        :options="yearsOptions"
        v-model:selected="years"
      />

      <div class="filter-timeframe"></div>
      <!-- <pre>
        {{ filters.getAvailableFilters }}
      </pre> -->
    </div>
  </div>
</template>
