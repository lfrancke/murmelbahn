# Murmelbahn Changelog

## 2026-06-13

- Support for the GraviTrax 2.8 "SkyTrax" update
  - SkyTrax courses (save format version 7) now parse
  - The bill of materials and GraviSheet output include the SkyTrax tiles and the connector piece
- GraviSheet output now matches the current "Tracks to Build" columns, including the 2025 Advent Calendar slope tiles and the Electric Cannon
- Courses using the 2023 Light Stones pieces (light stackers, light bases, releasers) no longer risk a crash, and the warning that they might not work has been removed
- The parser now tolerates unknown tile and enum values from future app updates instead of failing to read the whole course
- Updated to Rust edition 2024 and refreshed dependencies
- Still open (carried over from the entries below): the height-dependent
  calculations (rail lengths, stacker counts) for releasers, Space Tube and
  Vertical Cannon are still not verified, so piece counts for courses that use
  them may be off even though they no longer fail to load

## 2025-01-01

- Tracks from Gravitrax Autumn 2024 update should now be readable or at least not cause failures anymore
  - New tiles are not fully supported yet though (in height calculation etc.): Space Tube and Vertical Cannon
  - The same still goes for the "releasers" from Autumn of 2023
- GraviSheet output has been adjusted 
                                                             
## 2023-12-28

- Better support for the 2023 update, but it's probably not finished yet.
  I assume calculating heights/rails etc. with releasers won't yet work properly.
- Added Balance Starter Set
- Sets are now ordered alphabetically in the "Buildable" section
- Add a disclaimer at the top, including contact information
- Add docs for how to add a course

## 2023-11-02

- Initial support for the 2023 Gravitrax Update (Lightstones)

## 2023-05-01

- [#24](https://github.com/lfrancke/murmelbahn/issues/24): Show more details for buildable courses
   
## 2023-03-18

- [#17](https://github.com/lfrancke/murmelbahn/issues/17): The buildable courses now return a table with links to the app and other things instead of just a list

## 2023-02-21

- [#9](https://github.com/lfrancke/murmelbahn/issues/9): Store inventory in localStorage
- Return buildable courses ordered by creation date (in the database), this means that newly added courses will always appear at the end of the list

## 2023-02-15

- [#15](https://github.com/lfrancke/murmelbahn/issues/15): Fix Gravisheet output
- [#12](https://github.com/lfrancke/murmelbahn/issues/12): Support ZiplineAdded2019 courses
- [#14](https://github.com/lfrancke/murmelbahn/issues/14): Better guess the number of marbles needed
- [#8](https://github.com/lfrancke/murmelbahn/issues/8): Adds all remaining sets

## 2023-02-11

- Added Starter-Set Obstacle and Starter-Set Race
- Started changelog :)
