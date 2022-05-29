import { Image } from "../store/album"
import { LngLatBoundsLike } from "mapbox-gl"

export const map_access =
  "pk.eyJ1IjoiZG9sYW5za2UwMDAiLCJhIjoiY2wzZXd4YnVsMDNybzNibW9zNzlsdWtjcSJ9.FpKXu8VfkaVbW-fnyfPUsw"
export const map_dark = "mapbox://styles/dolanske000/cl3hpk2ji005514l4asj0h9nf"
export const map_light = "mapbox://styles/dolanske000/cl3hpjldc002615o43fx6c33f"

export function getBounds(images: Array<Image>): LngLatBoundsLike {
  const withLocation = images.filter((item) => item.location)

  return [
    [
      Math.min(...withLocation.map((image) => Number(image.location?.longitude))),
      Math.max(...withLocation.map((image) => Number(image.location?.latitude)))
    ],
    [
      Math.max(...withLocation.map((image) => Number(image.location?.longitude))),
      Math.min(...withLocation.map((image) => Number(image.location?.latitude)))
    ]
  ]
}
