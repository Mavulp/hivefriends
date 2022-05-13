export function getRanMinMax(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

// export function isNil(value: any) {
//   return value === undefined || value === null
// }

// function isObj(value: any) {
//   let type = typeof value
//   return value != null && (type == "object" || type == "function")
// }

// export function isEmpty(value: any) {
//   if (isNil(value)) return true
//   if (isObj(value)) return Object.keys(value).length === 0
//   return value.length === 0
// }

export function isFunc(func: any) {
  return Object.prototype.toString.call(func) == "[object Function]"
}
