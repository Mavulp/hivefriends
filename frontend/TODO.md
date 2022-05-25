# Fix

- [x] Expired token issues
- [ ] Fix image list showing error instead of loading in album lists
- [ ] Changing accent does not instantly update it for user
  - might have to inser into
  - destructured pinia instances lose reactivity, change from destructured to named import
    - just check for useUser() and redo them all just to be sure
- [ ] Accent colors being quirky
  - [ ] Could be related to previous issue

# Todo

- [ ] Figure out breadcrumbs again

  - [ ] On detail pages, overwrite breadcrumbs when data loads with dynamic stuff

- [ ] Album creation

  - [ ] Save album as draft (exclude from all lists except user albums)
  - [ ] Re-order image during album creation

- [ ] Album editing

  - [ ] Add "edit album" and "delete album" for albums that the user created
  - [ ] List all images, can delete and update imagekeys array in album then
  - [ ] Delete album
  - [ ] Re-order image during album editing

- [ ] Album Detail

  - [ ] a completely new full-screen view of map markers (images could appear within these markers as popups)
  - [ ] Delete album if author
  - [ ] Add a button "user albums" or "all albums" to get back to album list
  - [ ] Share link

- [ ] Dark mode

  - [ ] Medium Dark
  - [x] Normal dak -> rename to dark contast

- [ ] Album List

  - [ ] Add filtering to album lists

- [ ] Add link sharing image & title (like github does)

## Post Launch TODO

- [ ] Image detail
  - [ ] Slideshow mode (set delay in details)
    - [ ] padding is removed from image detail
    - [ ] controls fade, appear only if mouse has moved (disappear after like 5 seconds)
