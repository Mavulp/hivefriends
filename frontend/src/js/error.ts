import { reactive, ref } from "vue"
import { isEmpty, isFunc, isNil } from "./utils"

/**
 * Types
 */

export type Error = {
  type: string | null
  invalid: boolean
  errors: Array<string>
}

export interface Errors {
  [key: string]: Error
}

export type ValidationRule = {
  _validate: Function
  _message: Function
}

export type Rule = {
  [key: string]: ValidationRule
}

/**
 * Composable
 */

export function useFormValidation(form: object, rules: any) {
  // Create default errors object

  // TODO: add option to reset form on each keystroke
  // FIXME: fix errors adding onto each other (do a Set instead of array)
  // FIXME required should also check if length > 0

  const anyError = ref<boolean>(false)

  const errors = reactive<Errors>({
    ...Object.keys(form).reduce(
      (a, v) => ({
        ...a,
        [v]: {
          type: null,
          invalid: false,
          errors: []
        }
      }),
      {}
    )
  })

  function validate() {
    return new Promise(async (resolve, reject) => {
      anyError.value = false
      for (const [key, value] of Object.entries(form)) {
        const itemRules: Rule = rules.value[key]

        for (const [ruleKey, ruleData] of Object.entries(itemRules)) {
          const { _message, _validate }: ValidationRule = ruleData

          const result = await _validate(value)

          if (!result) {
            anyError.value = true

            // Is error
            errors[key].type = ruleKey
            errors[key].invalid = true
            errors[key].errors.push(_message())
          }
        }
      }

      if (anyError.value) {
        reject()
      } else {
        resolve(true)
      }
    })
  }

  return {
    errors,
    validate
  }
}

/**
 * Validations
 *
 * @Rule Returns FALSE if failed, returns TRUE if passed
 */

export const required = {
  _validate(value: any) {
    return !isEmpty(value)
  },
  _message() {
    return "Value is required"
  }
}

export const minLength = (len: number) => {
  return {
    _validate(value: any) {
      if (isNil(value)) return false

      return value?.length ? value.length >= len : false
    },
    _message() {
      return `Value must be at least ${len} characters long`
    }
  }
}
