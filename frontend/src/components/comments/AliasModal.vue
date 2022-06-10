<script setup lang="ts">
import { ref, reactive, computed, onBeforeMount } from "vue"
import { useLoading } from "../../store/loading"
import { get } from "../../js/fetch"
import { isValidImage } from "../../js/utils"

import Search from "../form/Search.vue"
import { useClipboard } from "@vueuse/core"

const emit = defineEmits<{
  (e: "close"): void
  (e: "insert", alias: string): void
}>()

type Alias = {
  name: string
  content: string
}

const { addLoading, getLoading, delLoading } = useLoading()

const list = ref<Array<Alias>>([])
const search = ref("")

onBeforeMount(async () => {
  addLoading("aliases")
  list.value = await get("/api/aliases").then((data) => data)
  delLoading("aliases")
})

const filterList = computed<Array<Alias>>(() => {
  return list.value.filter((item: Alias) => `!${item.name.toLowerCase()}`.includes(search.value.toLowerCase()))
})

// let called = false
// function loadImage(e: any) {
//   if (!called) {
//     console.log(e)
//     called = true
//   }
// }

/**
 * Copy
 */

const { copy, isSupported } = useClipboard()

/**
 * Insert text
 */

function emitInsert(name: string) {
  emit("insert", name)
  emit("close")
}
</script>

<template>
  <div class="modal-wrap modal-alias">
    <div class="modal-title">
      <h4>Alias list</h4>
      <button class="modal-close" @click="emit('close')">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div>

    <div class="modal-content">
      <div class="alias-search">
        <Search v-model:value="search" placeholder="Search for alias" />
      </div>

      <div class="alias-list">
        <template v-for="alias in filterList" :key="alias.name">
          <template v-if="alias.name && alias.content">
            <div class="alias-item" :class="[isValidImage(alias.content) ? 'alias-link' : 'alias-text']">
              <div class="alias-item-header">
                <div class="alias-image">
                  <img :src="alias.content" alt="" load="lazy" lazyload="true" />
                  <span class="material-icons">&#xe262; </span>
                </div>

                <p><b>!</b>{{ alias.name }}</p>

                <button
                  class="hover-bubble bubble-info"
                  data-title-top="Insert into comment"
                  @click="emitInsert(alias.name)"
                >
                  <span class="material-icons"> &#xe745; </span>
                </button>

                <button
                  class="hover-bubble bubble-orange"
                  data-title-top="Copy alias"
                  v-if="isSupported"
                  @click="copy(`!${alias.name}`)"
                >
                  <span class="material-icons"> &#xe14d; </span>
                </button>
              </div>

              <p class="alias-text-content">
                {{ alias.content }}
              </p>
            </div>
          </template>
        </template>
      </div>
    </div>
  </div>
</template>
