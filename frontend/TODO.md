# Fix

- [x] Ask for album sorting (now is reversed)
- [ ] Instead 100vh, use like 95 to show users can scroll
- [ ] Dont check for token on each page change
- [x] Fix using MY accent color on other user's profiles (user profile)
- [x] Add toast error on image album uplad
  - [x] Auto-clear of toasts

# Todo

- [ ] Figure out breadcrumbs again

  - [ ] On detail pages, overwrite breadcrumbs when data loads with dynamic stuff

- [ ] Album creation

  - [ ] Tag locations
  - [x] Tag users
  - [x] Set timeframe as a day (checkbox, but sets still from and to with the same date)
  - [ ] Save album as draft (exclude from all lists except user albums)

  - [x] Add option to update metadata per image
  - [x] Select a cover image

- [ ] Album editing

  - [ ] Add "edit album" and "delete album" for albums that the user created

- [ ] Album Detail

  - [x] Tagged users
    - [x] when clicked button, a white / dark block will slide from left or right
    - over the cover image and shows a simple list of tagged users within
  - [ ] map

- [ ] Album Comments

  - [ ] Figure out how to display on album detail
  - [ ] Design wrapper, comment & form to add comments
  - [ ] Implement

- [ ] Dark mode

  - [ ] Medium Dark
  - [x] Normal dak -> rename to dark contast

- [ ] Location on steroids

  - [ ] Each location is stored with its name, lat & long
    - [ ] Each image has location extracted from metadat??
  - [ ] Each album would have a world view with each location is pinned on the map

- [ ] Image detail

  - [x] Option to switch off header (for presentation mode)
  - [ ] Optiona to rotate image
  - [ ] top right button "metadata" which switches to 75 x 25 view with stuff about the image and controls on the right
  - [x] Arrow keys in albun detail navigation
  - [x] esc fucks off
  - [ ] Slideshow mode (set delay in details)
  - [x] Add programatical transition based on if the next image is prev or next (opacity slide left, opacity slide right)

- [ ] User Settings

  - [ ] Add setting a feature key (select with all album names and save album key)

- [ ] IMG improvement
  - [ ] Create a wrapper for every <img> element which will display a loading, instead of the white cube unfolding
  - [ ] @error handler to handle failed loads

# Pages To do Write up

- [ ] Image Detail
- [ ] Editing Album
- [ ] Homepage
- [ ] All albums ever
