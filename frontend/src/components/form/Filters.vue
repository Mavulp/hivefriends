<script setup lang="ts">
import { onBeforeMount, computed, ref, onBeforeUnmount, watch, watchEffect } from "vue"
import { useFilters, Options } from "../../store/filters"
import { useLoading } from "../../store/loading"
import { useUser } from "../../store/user"
import { isEmpty } from "lodash"

import InputSelect from "./InputSelect.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import Button from "../Button.vue"

const { getLoading } = useLoading()
const filter = useFilters()
const user = useUser()

const emit = defineEmits<{
  (e: "call"): void
}>()

const {
  disable = [],
  filters,
  loading = false
} = defineProps<{
  disable?: Array<string>
  filters?: Options
  loading?: boolean
}>()

onBeforeMount(() => {
  filter.fetchOptions(filters)
})

onBeforeUnmount(() => {
  filter.reset()
})

function clear() {
  filter.clear()
  emit("call")
}

watch(
  () => filter.active,
  (value) => {
    emit("call")
  },
  { deep: true }
)

/**
 * Authors
 */

const authorOptions = computed(() => {
  const authors = filter.getAvailableFilters("authors")

  return authors.map((item: string) => ({
    label: user.getUsername(item),
    value: item
  }))
})

const authors = computed<Array<string>>({
  get: () => filter.getActiveFilter("authors"),
  set: (value) => {
    // TODO: move into function
    // maybe move to action (fix typing)
    filter.active.authors = value

    if (!value || isEmpty(value)) {
      delete filter.active.authors
    }
  }
})

/**
 * Years
 */
const yearsOptions = computed(() => {
  const years = filter.getAvailableFilters("years")

  return years.map((year: number) => {
    return {
      label: year.toString(),
      value: year.toString()
    }
  })
})

const years = computed<Array<string>>({
  get: () => filter.getActiveFilter("years"),
  set: (value) => {
    filter.active.years = value

    if (!value || isEmpty(value)) {
      delete filter.active.years
    }
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
    <h6>
      Filters
      <LoadingSpin class="dark small" v-if="loading" />
    </h6>

    <slot />

    <div class="filters-loading" v-if="getLoading('options')">
      <LoadingSpin class="dark center-parent" />
    </div>

    <div class="form-filters" v-else>
      <!-- Authors -->

      <InputSelect
        v-if="authorOptions && !disable?.includes('authors')"
        label="Authors"
        placeholder="Filter by authors"
        :options="authorOptions"
        v-model:selected="authors"
        multiple
      />

      <InputSelect
        v-if="yearsOptions && !disable?.includes('years')"
        label="Years"
        placeholder="Filter albums by event years"
        :options="yearsOptions"
        v-model:selected="years"
        multiple
      />

      <div class="filter-timeframe"></div>

      <template v-if="!isEmpty(filter.active)">
        <Button class="btn-blue" @click="clear()">Clear</Button>
      </template>
      <!-- <pre>
        {{ filters.getAvailableFilters }}
      </pre> -->
    </div>
  </div>
</template>
