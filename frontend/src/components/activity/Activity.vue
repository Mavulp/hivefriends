<script setup lang="ts">
import { onClickOutside, useMagicKeys, whenever } from "@vueuse/core"
import { ref, computed, watch, useAttrs, provide } from "vue"
import { useActivity } from "../../store/activity"
import { useAlbums } from "../../store/album"
import { useLoading } from "../../store/loading"
import { formatDate } from "../../js/utils"

import LoadingSpin from "../loading/LoadingSpin.vue"
import Button from "../Button.vue"
import ActivityItem from "./ActivityItem.vue"

const { getLoading } = useLoading()
const activity = useActivity()
const attrs = useAttrs()
const keys = useMagicKeys()
const album = useAlbums()

const wrap = ref(null)
const emit = defineEmits<{
  (e: "close"): void
}>()

const props = defineProps<{
  limit?: boolean
}>()

whenever(keys["Escape"], () => emit("close"))
onClickOutside(wrap, () => {
  if (attrs.class === "active") {
    setTimeout(() => {
      emit("close")
    }, 5)
  }
})

watch(
  () => attrs.class,
  (val) => {
    if (val === "active") {
      query()
    }
  }
)

async function query() {
  if (!getLoading("activity")) {
    activity.fetchActivity()
    album.fetchAlbums()
  }
}

/**
 * List
 */

const sorted = computed(() => {
  console.log(props.limit);
  

  if (props.limit) {
    const spliced = Object.entries(activity.sortedItems).splice(0, 15)

    return spliced.reduce((group, [key,value]) => {
      group[key] = value
      return group
    }, {} as Record<string, any>)
  }

  return activity.sortedItems
})

//@ts-ignore
provide("is-in-header", !attrs?.class?.includes("activity-home") ?? false)
</script>

<template>
  <div ref="wrap" class="activity">
    <button @click="emit('close')" data-title-left="Close" class="hover-bubble close">
      <span class="material-icons">&#xe5cd;</span>
    </button>

    <div class="title-wrap">
      <h5>Latest activity</h5>
    </div>

    <div class="activity-list-wrap">
      <LoadingSpin class="dark center-parent" v-if="getLoading('activity', 'albums')" />
      <template v-else>
        <div class="activity-group" v-for="(items, day) in sorted">
          <div class="activity-group-title">
            <strong>
              {{ formatDate(new Date(day).getTime() / 1000) }}
            </strong>
            <div class="line"></div>
            <span>{{ items.length }}</span>
          </div>
          <ActivityItem v-for="(item, index) in items" :data="item" :key="index" />
        </div>
      </template>
    </div>
  </div>
</template>
