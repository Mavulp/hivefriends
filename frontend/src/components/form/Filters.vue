<script setup lang="ts">
import { ref } from "vue"
import { useLoading } from "../../store/loading"

import LoadingSpin from "../loading/LoadingSpin.vue"

const { getLoading } = useLoading()
</script>

<template>
  <div class="filters">
    <h6>Filters</h6>

    <div class="filters-loading" v-if="getLoading('options')">
      <LoadingSpin class="dark center-parent" />
    </div>

    <div class="form-filters" v-else>hehe</div>
  </div>
</template>
