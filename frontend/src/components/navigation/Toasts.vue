<script setup lang="ts">
import { useToast } from "../../store/toast"
import Toast from "./Toast.vue"

const toast = useToast()
</script>

<template>
  <div class="hi-toasts">
    <transition-group name="toast">
      <Toast v-for="(item, index) in toast.items" :key="item.text + index" :data="item" :index="index" />
    </transition-group>
  </div>
</template>

<style lang="scss" scoped>
// .toast-enter-active,
// .toast-leave-active {
// }

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  margin-bottom: 0;
  height: 0;
  line-height: 0;
}
</style>
