<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import Btn from "../../components/Button.vue"
import { ref, reactive, computed } from "vue"
import { getRanMinMax } from "../../js/utils"
import { useAuth } from "../../store/auth"
import { useFormValidation, required, minLength, asyncValidation } from "../../js/error"

const auth = useAuth()

type SignInForm = {
  username: string
  password: string
}

const form = reactive<SignInForm>({ username: "", password: "" })

const placeholders = [
  "dolanspaghetti",
  "tmtupomegranate",
  "apjriehat",
  "stormtrooper11",
  "clobatenjoyer",
  "quoteshater"
]
const placeholder = ref(placeholders[getRanMinMax(0, 5)])

async function submit() {
  validate()
    .then(() => {
      // Submit
      if (form.username && form.password) {
        auth.signIn(form)
        reset()
      }
    })
    .catch((errors) => {
      console.log(errors)
    })
}

const rules = computed(() => ({
  username: {
    required
  },
  password: {
    required,
    minLength: minLength(3)
  }
}))

// Setup validation
const { errors, validate, reset } = useFormValidation(form, rules, { autoclear: true })
</script>

<template>
  <div class="route-login">
    <div class="route-login-split has-block">
      <div class="block">Bunch of friends all around the world cherishing moments spent together.</div>
    </div>

    <div class="route-login-split has-form">
      <form @submit.prevent="submit" class="form-wrap">
        <InputText :error="errors.username" v-model:value="form.username" label="Username" :placeholder="placeholder" />
        <InputText
          :error="errors.password"
          v-model:value="form.password"
          label="Password"
          type="password"
          placeholder="***************"
        />

        <!-- <Btn class="btn-login" type="submit" size="56px" :class="{ 'btn-disabled': !form.username || !form.password }">
          <template #default>
            <span>Sign In</span>
          </template>
        </Btn> -->

        <Btn class="btn-login" type="submit" size="56px">
          <template #default>
            <span>Sign In</span>
          </template>
        </Btn>
      </form>

      <p class="copy-text">Copyright {{ new Date().getFullYear() }} Mavulp (hivecom devs)</p>
    </div>
  </div>
</template>
