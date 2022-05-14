export function getRanMinMax(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

export function isFunc(func: any) {
  return Object.prototype.toString.call(func) == "[object Function]"
}

export function formatDate(date: Date | number) {
  if (typeof date === "number" && date.toString().length === 10) date *= 1000

  const d = new Date(date)

  return d.toLocaleDateString("en-GB", { weekday: "long", year: "numeric", month: "long", day: "numeric" })
}

export function delay(ms: number) {
  return new Promise((resolve) => setTimeout(() => resolve(true), ms))
}
