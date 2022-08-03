<script setup lang="ts">
import { onClickOutside, useMagicKeys, whenever } from "@vueuse/core"
import { ref, computed, onBeforeMount, onMounted, watch, useAttrs } from "vue"
import { useActivity } from "../../store/activity"
import { useLoading } from "../../store/loading"
import Button from "../Button.vue"
import NotificationItem from "./NotificationItem.vue"

const { getLoading } = useLoading()
const activity = useActivity()
const attrs = useAttrs()
const keys = useMagicKeys()
const wrap = ref(null)
const emit = defineEmits<{
  (e: "close"): void
}>()

whenever(keys["Escape"], () => emit("close"))
onClickOutside(wrap, () => emit("close"))

watch(
  () => attrs.class,
  (val) => {
    if (val) {
      query()
    }
  }
)

async function query() {
  if (!getLoading("activity")) {
    activity.fetchActivity()
  }
}

/**
 * List
 */

const data = computed(() => activity.items)

// const activity = useNotifications()
// const items = computed(() => activity.items.sort((a, b) => b.createdAt - a.createdAt))

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
      <h4>Activity Log</h4>
      <!-- <button class="hover-bubble bubble-highlight">Mark as read ({{ items.length }})</button> -->
    </div>

    <div class="activity-list-wrap">
      <span v-if="getLoading('activity')"></span>

      <pre>
        {{ data }}
      </pre>
      <!-- <template v-if="items.length === 0">
        <p class="no-notifs">You have no <b>activity.</b></p>
      </template>
      <template v-else>
        <NotificationItem v-for="item in items" :data="item" :key="item.id" />
      </template> -->
    </div>
  </div>
</template>
