export function getRanMinMax(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

export function isFunc(func: any) {
  return Object.prototype.toString.call(func) == "[object Function]"
}

export function formatDate(date: Date | number) {
  if (typeof date === "number" && date.toString().length < 13) date *= 1000

  const d = new Date(date)

  return d.toLocaleDateString("en-GB", { weekday: "long", year: "numeric", month: "long", day: "numeric" })
}

export function delay(ms: number) {
  return new Promise((resolve) => setTimeout(() => resolve(true), ms))
}

export function HEX_to_RGB(hex: string): [number, number, number] {
  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result ? [parseInt(result[1], 16), parseInt(result[2], 16), parseInt(result[3], 16)] : [0, 0, 0]
}

export function TEXT_CONTRAST(r: number, g: number, b: number): string {
  var yiq = (r * 299 + g * 587 + b * 114) / 1000
  return yiq >= 128 ? "black" : "white"
}
