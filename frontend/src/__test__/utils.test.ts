import { assert, expect, test } from "vitest"

import * as _utils from "../js/utils"

test("isFunc", () => {
  expect(_utils.isFunc(() => ({}))).toBeTruthy()
  expect(_utils.isFunc({})).toBeFalsy()
  expect(_utils.isFunc("hello")).toBeFalsy()
  expect(_utils.isFunc(new Date())).toBeFalsy()
})

// test("formatDate", () => {
//   expect(_utils.formatDate())
// })
