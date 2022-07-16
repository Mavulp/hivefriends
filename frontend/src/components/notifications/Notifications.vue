<script setup lang="ts">
import { onClickOutside } from "@vueuse/core"
import { ref, computed } from "vue"
import { useNotifications } from "../../store/notification"
import Button from "../Button.vue"
import NotificationItem from "./NotificationItem.vue"

const wrap = ref(null)
const emit = defineEmits<{
  (e: "close"): void
}>()

onClickOutside(wrap, () => {
  emit("close")
})

/**
 * List
 */

const notifications = useNotifications()
const items = computed(() => notifications.items.sort((a, b) => b.createdAt - a.createdAt))

/**
 * Manage
 */

function markRead() {}
</script>

<template>
  <div ref="wrap" class="notifications">
    <button @click="emit('close')" data-title-left="Close" class="hover-bubble close">
      <span class="material-icons">&#xe5cd;</span>
    </button>

    <div class="title-wrap">
      <h4>Notifications</h4>
      <button class="hover-bubble bubble-highlight">Mark as read</button>
    </div>

    <div class="notification-list-wrap">
      <template v-if="items.length === 0">
        <p class="no-notifs">You have no <b>notifications.</b></p>
      </template>
      <template v-else>
        <NotificationItem v-for="item in items" :data="item" :key="item.id" />
      </template>
    </div>
  </div>
</template>
