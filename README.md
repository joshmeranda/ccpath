# ccpath
Rename files to conform to a given naming convention.

## Overview
Ccpath provides the ability to convert one, or many paths to a specified naming convention. See the following example

```ls
[ccpath@localhost dir]$ ls
FILE_FOUR  'file one'   fileThree   file-two
[ccpath@localhost dir]$ ccpath --verbose snake 'file one' file-two fileThree FILE_FOUR
'FILE_FOUR' -> 'file_four'
'file one' -> 'file_one'
'fileThree' -> 'file_three'
'file-two' -> 'file_two'
[ccpath@localhost dir]$ ls
file_four  file_one  file_three  file_two
```

## Supported Naming Conventions

| name              | example       | description                                                                       |
| ----------------- | ------------- | --------------------------------------------------------------------------------- |
| Title Case        | 'Title Case'  | Every word is capitalized and joined with spaces                                  |
| Flat Case         | flatcase      | Every letter is under case and there is no boundary between words (lossy)         |
| Upper Flat Case   | UPPERFLATCASE | Every letter is capitalized and there is no boundary between words (lossy)        |
| Camel Case        | camelCase     | The first word is under case, and every word is capitalized with no word boundary |
| Upper Camel Case  | CamelCase     | All words are capitalized and joined with no word boundary                        |
| Snake Case        | snake_case    | All letters are lower case and words are joined with an '_'                       |
| Upper Snake Case  | SNAKE_CASE    | All letters are capitalized and words are joined with an '_'                      |
| Kebab Case        | kebab-case    | All letters are lower cased and words are joined with a '-'                       |