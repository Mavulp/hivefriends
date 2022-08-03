<script setup lang="ts">
import { onClickOutside, useMagicKeys, whenever } from "@vueuse/core"
import { isEmpty } from "lodash"
import { ref, computed, watch, useAttrs } from "vue"
import { useActivity } from "../../store/activity"
import { useAlbums } from "../../store/album"
import { useLoading } from "../../store/loading"

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

const data = computed(() => activity.items)

/**
 * Manage
 */

function markRead() {}
</script>

<template>
  <div ref="wrap" class="activity">
    <button @click="emit('close')" data-title-left="Close" class="hover-bubble close">
      <span class="material-icons">&#xe5cd;</span>
    </button>

    <div class="title-wrap">
      <h4>Lastest Activity</h4>
      <!-- <button class="hover-bubble bubble-highlight">Mark as read ({{ items.length }})</button> -->
    </div>

    <div class="activity-list-wrap">
      <span v-if="getLoading('activity', 'albums')">LOADING</span>
      <ActivityItem v-for="(item, index) in data" :data="item" :key="index" />
    </div>
  </div>
</template>
