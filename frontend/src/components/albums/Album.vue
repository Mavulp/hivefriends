<script setup lang="ts">
interface Props {
  data: {
    title: string
    users: Array<string>
    timestamps: {
      from: string
      to: string
    }
    photos: number
  }
  large?: boolean
}

const { large = false, data } = defineProps<Props>()
</script>

<template>
  <div class="album" :class="{ 'album-large': large }">
    <div class="album-title">
      <div class="album-timestamp">
        <span>{{ data.timestamps.from }}</span>
        <div class="timestamp-divider"></div>
        <span>{{ data.timestamps.to }}</span>
      </div>
      <h2>{{ data.title }}</h2>

      <div class="album-desc">
        <span>{{ data.users.length }} People</span>
        <span>{{ data.photos }} Photos</span>
      </div>
    </div>

    <div class="album-image"></div>
  </div>
</template>
