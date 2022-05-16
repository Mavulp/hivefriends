<script setup lang="ts">
import { reactive, ref } from "vue"
import ButtonVue from "../Button.vue"

import InputCheckbox from "../form/InputCheckbox.vue"

const settings = reactive({
  private: false,
  // theme: "light-theme",
  darktheme: false,
  accent: "#e57d23",
  socials: []
})

type SocialLink = {
  name: string
  icon: string
  link: string
}

interface Settings {
  private: boolean
  theme: string
  accent: string
  socials: Array<SocialLink | []>
}

const themeOptions = ["light-theme", "dark-contrast", "dark-normal"]
</script>

<template>
  <div class="hi-user-page">
    <div class="hi-user-settings">
      <h1>Your Settings</h1>

      <ul class="user-settings-list">
        <li>
          <InputCheckbox
            label="Set yourself as private. You won't be mentioned in albums, your albums won't be publicly available"
            v-model:check="settings.private"
          />
        </li>
        <li>
          <InputCheckbox label="Use dark theme" v-model:check="settings.darktheme" />
        </li>
      </ul>

      <div class="user-actions">
        <Button>Delete Albums</Button>
        <Button>Delete User</Button>
      </div>
    </div>
  </div>
</template>
