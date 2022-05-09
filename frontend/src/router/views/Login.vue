<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import Btn from "../../components/Button.vue"
import { ref, reactive } from "vue"
import { getRanMinMax } from "../../js/utils"

const form = reactive({
  username: null,
  password: null
})

const placeholders = [
  "dolanspaghetti",
  "tmtupomegranate",
  "apjriehat",
  "stormtrooper11",
  "clobatenjoyer",
  "quoteshater"
]
const placeholder = ref(placeholders[getRanMinMax(0, 5)])

function submit() {
  console.log("?>??")
}
</script>

<template>
  <div class="route-login">
    <div class="route-login-split has-block">
      <div class="block">Bunch of friends all around the world cherishing moments spent together.</div>
    </div>

    <div class="route-login-split has-form">
      <form @submit.prevent="submit" class="form-wrap">
        <InputText v-model:value="form.username" label="Username" :placeholder="placeholder" />
        <InputText v-model:value="form.password" label="Password" type="password" placeholder="***************" />

        <!-- <button class="button"></button> -->
        <Btn class="btn-login" type="submit" size="56px" :class="{ 'btn-disabled': !form.username || !form.password }">
          <template #default>
            <span>Sign In</span>
          </template>
        </Btn>
      </form>

      <p class="copy-text">Copyright {{ new Date().getFullYear() }} Mavulp (hivecom devs)</p>
    </div>
  </div>
</template>
