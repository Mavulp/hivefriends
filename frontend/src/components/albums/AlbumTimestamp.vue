<script setup lang="ts">
import dayjs from 'dayjs'
import { normalDateFormat } from '../../js/time'

interface Props {
  timeframe: {
    from: number
    to: number
  }
}

const { timeframe } = defineProps<Props>()
</script>

<template>
  <div v-if="timeframe.from && timeframe.to" class="album-timestamp">
    <!-- Single day -->
    <span v-if="timeframe.from === timeframe.to">
      {{ dayjs(timeframe.from * 1000).format(normalDateFormat) }}
    </span>

    <!-- Start to end -->
    <template v-else>
      <span v-if="timeframe.from">{{ dayjs(timeframe.from * 1000).format(normalDateFormat) }}</span>
      <div class="timestamp-divider" />
      <span v-if="timeframe.to">{{ dayjs(timeframe.to * 1000).format(normalDateFormat) }}</span>
    </template>
  </div>
</template>
