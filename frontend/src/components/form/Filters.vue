<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, watch } from 'vue'
import { isEmpty } from 'lodash'
import type { Options } from '../../store/filters'
import { useFilters } from '../../store/filters'
import { useLoading } from '../../store/loading'
import { useUser } from '../../store/user'

import LoadingSpin from '../loading/LoadingSpin.vue'
import Button from '../Button.vue'
import InputSelect from './InputSelect.vue'

const {
  disable = [],
  filters,
  loading = false,
} = defineProps<{
  disable?: Array<string>
  filters?: Options
  loading?: boolean
}>()
const emit = defineEmits<{
  (e: 'call'): void
}>()
const { getLoading } = useLoading()
const filter = useFilters()
const user = useUser()

onBeforeMount(() => {
  filter.fetchOptions(filters)
})

onBeforeUnmount(() => {
  filter.reset()
})

function clear() {
  filter.clear()
  emit('call')
}

watch(
  () => filter.active,
  (value) => {
    emit('call')
  },
  { deep: true },
)

/**
 * Authors
 */

const authorOptions = computed(() => {
  const authors = filter.getAvailableFilters('authors')

  return authors.map((item: string) => ({
    label: user.getUsername(item),
    value: item,
  }))
})

const authors = computed<Array<string>>({
  get: () => filter.getActiveFilter('authors'),
  set: (value) => {
    // TODO: move into function
    // maybe move to action (fix typing)
    filter.active.authors = value

    if (!value || isEmpty(value))
      delete filter.active.authors
  },
})

/**
 * Years
 */
const yearsOptions = computed(() => {
  const years = filter.getAvailableFilters('years')

  return years.map((year: number) => {
    return {
      label: year.toString(),
      value: year.toString(),
    }
  })
})

const years = computed<Array<string>>({
  get: () => filter.getActiveFilter('years'),
  set: (value) => {
    filter.active.years = value

    if (!value || isEmpty(value))
      delete filter.active.years
  },
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
    <!-- <h6>
      Filters
      <LoadingSpin v-if="loading" class="dark small" />
    </h6> -->

    <slot />

    <div v-if="getLoading('options')" class="filters-loading">
      <LoadingSpin class="dark center-parent" />
    </div>

    <div v-else class="form-filters">
      <!-- Authors -->

      <InputSelect
        v-if="authorOptions && !disable?.includes('authors')"
        v-model:selected="authors"
        label="Authors"
        placeholder="Filter by authors"
        :options="authorOptions"
        multiple
      />

      <InputSelect
        v-if="yearsOptions && !disable?.includes('years')"
        v-model:selected="years"
        label="Years"
        placeholder="Filter albums by event years"
        :options="yearsOptions"
        multiple
      />

      <div class="filter-timeframe" />

      <template v-if="!isEmpty(filter.active)">
        <Button class="btn-blue" @click="clear()">
          Clear
        </Button>
      </template>
      <!-- <pre>
        {{ filters.getAvailableFilters }}
      </pre> -->
    </div>
  </div>
</template>
