<script setup lang="ts">
import Button from "../../components/Button.vue"
// import { ref } from "vue"
</script>

<template>
  <div class="hi-home">
    <h1>hi!friends</h1>
    <p>
      Hello, welcome to hi!friends. Unfortunately homepage is the second last item on the backlog. Wanna see all the
      albums tho?
    </p>
    <Button :to="{ name: 'Albums' }" class="btn-red">View Albums</Button>
  </div>
</template>
