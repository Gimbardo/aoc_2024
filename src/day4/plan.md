# Steps:

## Surround content

from

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

to

```
000000000000
0MMMSXXMASM0
0MSAMXMSMSA0
0AMXSXMAAMM0
0MSAMASMSMX0
0XMASAMXAMM0
0XXAMMXXAMA0
0SMSMSASXSS0
0SAXAMASAAA0
0MAMMMXMMMM0
0MXMXAXMASX0
000000000000
```

## Search for every x's index

from content to => array of indexes

## Search for x's with an m after, and define direction(s)

from content and array => array_directions [{ m_x: int, m_y: int, direction_x: int, direction_y: int }, {...}]

directions map:

[
  {x: -1, y: -1},
  {x: -1, y: 0},
  {x: -1, y: 1},
  {x: 0, y: -1},
  (...)
]

## Check for every element of array_directions if they complete the word in the selected direction

for every element inside array_direction => search for "a" and "s" in the selected direction, following the direction_array_map instructions on how to change x and y