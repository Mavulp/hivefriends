<script setup lang="ts">
import { onClickOutside, useMagicKeys, whenever } from '@vueuse/core'
import { computed, ref, useAttrs, watch } from 'vue'
import { useActivity } from '../../store/activity'
import { useAlbums } from '../../store/album'
import { useLoading } from '../../store/loading'
import { formatDate } from '../../js/utils'

import LoadingSpin from '../loading/LoadingSpin.vue'
import ActivityItem from './ActivityItem.vue'

const props = defineProps<{
  limit?: boolean
}>()
const emit = defineEmits<{
  (e: 'close'): void
}>()
const { getLoading } = useLoading()
const activity = useActivity()
const attrs = useAttrs()
const keys = useMagicKeys()
const album = useAlbums()

const wrap = ref(null)
whenever(keys.Escape, () => emit('close'))
onClickOutside(wrap, () => {
  if (attrs.class === 'active') {
    setTimeout(() => {
      emit('close')
    }, 5)
  }
})

watch(
  () => attrs.class,
  (val) => {
    if (val === 'active')
      query()
  },
)

async function query() {
  if (!getLoading('activity')) {
    activity.fetchActivity()
    album.fetchAlbums()
  }
}

/**
 * List
 */

const sorted = computed(() => {
  if (props.limit) {
    const spliced = Object.entries(activity.sortedItems).splice(0, 2)

    return spliced.reduce((group, [key, value]) => {
      group[key] = value
      return group
    }, {} as Record<string, any>)
  }

  return activity.sortedItems
})
</script>

<template>
  <div ref="wrap" class="activity">
    <button data-title-left="Close" class="hover-bubble close" @click="emit('close')">
      <span class="material-icons">&#xe5cd;</span>
    </button>

    <div class="title-wrap">
      <h5>Latest activity</h5>
    </div>

    <div class="activity-list-wrap">
      <LoadingSpin v-if="getLoading('activity', 'albums')" class="dark center-parent" />
      <template v-else>
        <div v-for="(items, day) in sorted" :key="day" class="activity-group">
          <div class="activity-group-title">
            <strong>
              {{ formatDate(new Date(day).getTime() / 1000) }}
            </strong>
            <div class="line" />
            <span>{{ items.length }}</span>
          </div>
          <ActivityItem v-for="(item, index) in items" :key="index" :data="item" />
        </div>
      </template>
    </div>

    <div v-if="props.limit" style="width:100%;">
      <hr>
      <router-link :to="{ name: 'RouteActivity' }" class="hover-bubble highlight">
        View older posts
      </router-link>
    </div>
  </div>
</template>
