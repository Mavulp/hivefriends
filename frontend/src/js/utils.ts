export function getRanMinMax(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

export function formatDate(date: Date | number) {
  if (typeof date === "number") date *= 1000

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

export function RGB_TO_HEX(r: number | string, g?: number | string, b?: number | string) {
  if (!g && !b && typeof r === "string") {
    const [_r, _g, _b] = r.split(",")

    r = _r
    g = _g
    b = _b
  }

  r = Number(r)
  g = Number(g)
  b = Number(b)

  return (
    "#" +
    [r, g, b]
      .map((x) => {
        const hex = x.toString(16)
        return hex.length === 1 ? "0" + hex : hex
      })
      .join("")
  )
}

export function formatFileSize(bytes: string | number, round?: boolean) {
  if (typeof bytes === "string") {
    bytes = Number(bytes)
  }

  if (isNaN(bytes)) return 0

  if (bytes / 1000000 > 1) return round ? Math.round(bytes / 1000000) + "MB" : bytes / 1000000 + "MB"
  return round ? Math.round(bytes / 1000) + "KB" : bytes / 1000 + "KB"
}

export async function fetchFlag(code: string, type: string = "svg"): Promise<string> {
  const url = `https://countryflagsapi.com/${type}/${code}`
  return fetch(url).then((r) => r.text())
}

export function flag(code: string, type: string = "svg") {
  return `https://countryflagsapi.com/${type}/${code}`
}

export function median(numbers: Array<number>) {
  const sorted = Array.from(numbers).sort((a, b) => a - b)
  const middle = Math.floor(sorted.length / 2)

  if (sorted.length % 2 === 0) {
    return (sorted[middle - 1] + sorted[middle]) / 2
  }

  return sorted[middle]
}

export function sanitize(text: string) {
  if (!text) return null

  const regex = /\bon\w+\=\"?[\w\:\(\)\']+\"?/g
  return text.replaceAll(regex, "")
}

export const formats = [".jpeg", ".gif", ".png", ".apng", ".svg", ".bmp", ".bmp", ".ico", ".jpg", ".webp"]

export function isValidImage(text: string) {
  return formats.some((format) => text.endsWith(format))
}
