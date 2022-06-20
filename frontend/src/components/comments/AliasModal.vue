<script setup lang="ts">
import { ref, computed, onBeforeMount } from "vue"
import { useLoading } from "../../store/loading"
import { get } from "../../js/fetch"
import { isValidImage } from "../../js/utils"

import Search from "../form/Search.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import { useMagicKeys, whenever } from "@vueuse/core"

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

/**
 * Insert text
 */

function emitInsert(name: string) {
  emit("insert", name)
  // emit("close")
}

const keys = useMagicKeys()
whenever(keys["Escape"], () => {
  emit("close")
})
</script>

<template>
  <div class="alias-picker">
    <!-- <div class="modal-title">
      <h4>Alias list</h4>
      <button class="modal-close" @click="emit('close')">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div> -->

    <div class="modal-content">
      <div class="alias-search">
        <Search v-model:value="search" placeholder="Search for alias" />
      </div>

      <div class="alias-list">
        <template v-if="getLoading('aliases')">
          <LoadingSpin class="dark small center-parent" />
        </template>
        <template v-else>
          <template v-for="alias in filterList" :key="alias.name">
            <template v-if="alias.name && alias.content">
              <button class="alias-item" @click="emitInsert(alias.name)">
                <div class="alias-content">
                  <div class="alias-image" v-if="isValidImage(alias.content)">
                    <img :src="alias.content" alt="" load="lazy" lazyload="true" />
                  </div>
                  <p v-else>{{ alias.content }}</p>
                </div>

                <span class="alias-name">!{{ alias.name }}</span>
              </button>
            </template>
          </template>
        </template>
      </div>
    </div>
  </div>
</template>
