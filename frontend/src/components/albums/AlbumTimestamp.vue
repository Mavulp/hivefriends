<script setup lang="ts">
import { formatDate } from "../../js/utils"

interface Props {
  timeframe: {
    from: number
    to: number
  }
}

const { timeframe } = defineProps<Props>()
</script>

<template>
  <div class="album-timestamp" v-if="timeframe.from && timeframe.to">
    <!-- Single day -->
    <span v-if="timeframe.from === timeframe.to">
      {{ formatDate(timeframe.from) }}
    </span>

    <!-- Start to end -->
    <template v-else>
      <span v-if="timeframe.from">{{ formatDate(timeframe.from) }}</span>
      <div class="timestamp-divider"></div>
      <span v-if="timeframe.to">{{ formatDate(timeframe.to) }}</span>
    </template>
  </div>
</template>
