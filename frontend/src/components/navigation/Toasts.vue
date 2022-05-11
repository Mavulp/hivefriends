<script setup lang="ts">
import { useToast } from "../../store/toast"

const toast = useToast()
</script>

<template>
  <div class="hi-toasts">
    <transition-group name="toast">
      <div v-for="(item, index) in toast.items" :key="item.text + index" class="hi-toast-item" :class="[item.type]">
        <p>{{ item.text }}</p>
        {{ index }}

        <button @click="toast.del(index)">
          <span class="material-icons">&#xe5cd;</span>
        </button>
      </div>
    </transition-group>
  </div>
</template>

<style lang="scss" scoped>
.toast-enter-active,
.toast-leave-active {
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  margin-bottom: 0;
  height: 0;
  line-height: 0;
}
</style>
