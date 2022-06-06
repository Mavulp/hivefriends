<script setup lang="ts">
import { onBeforeMount, computed, ref, onBeforeUnmount, watch, watchEffect } from "vue"
import { useFilters } from "../../store/filters"
import { useLoading } from "../../store/loading"
import { useUser } from "../../store/user"
import { isEmpty } from "lodash"

import InputSelect from "./InputSelect.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import Button from "../Button.vue"

const { getLoading } = useLoading()
const filters = useFilters()
const user = useUser()

const emit = defineEmits<{
  (e: "call"): void
}>()

onBeforeMount(() => {
  filters.fetchOptions()
})

onBeforeUnmount(() => {
  filters.reset()
})

function clear() {
  filters.reset()
  emit("call")
}

watchEffect(() => {
  if (!isEmpty(filters.active)) {
    emit("call")
  }
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
  set: (value) => (filters.active.authors = value)
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
  set: (value) => {
    filters.active.years = value
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
        multiple
      />

      <InputSelect
        v-if="yearsOptions"
        label="Years"
        placeholder="Filter albums by event years"
        :options="yearsOptions"
        v-model:selected="years"
        multiple
      />

      <div class="filter-timeframe"></div>

      <template v-if="!isEmpty(filters.active)">
        <Button class="dark" @click="clear()">Clear</Button>
      </template>
      <!-- <pre>
        {{ filters.getAvailableFilters }}
      </pre> -->
    </div>
  </div>
</template>
