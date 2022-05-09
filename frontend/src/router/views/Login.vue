<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import Btn from "../../components/Button.vue"
import { reactive } from "vue"
import { getRanMinMax } from "../../js/utils"

const form = reactive({
  username: null,
  password: null
})

const placeholder = [
  "dolanspaghetti",
  "tmtupomegranate",
  "apjriehat",
  "stormtrooper11",
  "clobatenjoyer",
  "garticphonehater"
]

function submit() {}
</script>

<template>
  <div class="route-login">
    <div class="route-login-split has-block">
      <div class="block">Bunch of friends all around the world cherishing moments spent together.</div>
    </div>

    <div class="route-login-split has-form">
      <form @submit.prevent="submit" class="form-wrap">
        <InputText v-model:value="form.username" label="Username" :placeholder="placeholder[getRanMinMax(0, 3)]" />
        <InputText v-model:value="form.password" label="Password" type="password" placeholder="***************" />

        <!-- <button class="button"></button> -->
        <Btn
          class="btn-login-special"
          type="submit"
          size="56px"
          :class="{ 'btn-disabled': !form.username || !form.password }"
        >
          <template #default>
            <span>Sign In</span>
          </template>
        </Btn>
      </form>

      <p class="copy-text">Copyright {{ new Date().getFullYear() }} Mavulp (hivecom devs)</p>
    </div>
  </div>
</template>
