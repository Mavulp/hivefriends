# Fix

- [ ] Expired token issues
- [ ] Fix image list showing error instead of loading in album lists

# Todo

- [ ] Scan all markers when displaying a map, and check for large differences. Change map zoom according to that
  - [ ] https://docs.mapbox.com/mapbox-gl-js/example/fitbounds/
- [ ] Figure out breadcrumbs again

  - [ ] On detail pages, overwrite breadcrumbs when data loads with dynamic stuff

- [ ] Album creation

  - [ ] Save album as draft (exclude from all lists except user albums)

- [ ] Album editing

  - [ ] Add "edit album" and "delete album" for albums that the user created
  - [ ] List all images, can delete and update imagekeys array in album then
  - [ ] Delete album

- [ ] Album Detail

  - [ ] a completely new full-screen view of map markers (images could appear within these markers as popups)
  - [ ] Delete album if author
  - [ ] Add a button "user albums" or "all albums" to get back to album list
  - [ ] Share link

- [ ] Dark mode

  - [ ] Medium Dark
  - [x] Normal dak -> rename to dark contast

- [ ] Image detail

  - [ ] Add an uploader field which you can click to go to their profile (could have the pfp & author tag as well for extra color)
  - [ ] Share link
  - [ ] Slideshow mode (set delay in details)
    - [ ] padding is removed from image detail
    - [ ] controls fade, appear only if mouse has moved (disappear after like 5 seconds)
  - [x] Comments
    - [x] Design template first
    - [x] Implement
    - [ ] Show timestamp (short date, but include time)
    - [ ] If user adds an image, automatically turn it into <img> element.
    - [ ] if user adds a link, automatically turnt it it to <a>

- [ ] User profile

  - [ ] Hide "more albums" if user only has 2 or less albums

- [ ] Album List

  - [ ] Add filtering to album lists

- [ ] Homepage

  - [ ] Design and implement

- [ ] Add link sharing image & title (like github does)
