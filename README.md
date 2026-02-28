# jam-doc-rs
Analyze the quality of a music library and recommend improvements

## Commands

### `lib`

Lists all registered libraries of music.

- `add <path>`
  - Adds a path as a library
- `remove <path>`
  - Removes a path a library

### `index`

Indexes all music files in each library, asynchronously

  - `purge`
    - Purges the index
  
### `diagnose`

Lists all available diagnostic criteria

  - `bitrate <floor>`
    - Lists paths for all tracks with bitrates < `floor`
  - `tagver <v>`
    - Lists paths for all tracks with id3 tag version != `v`
  - `incomplete <field0>, <field1>...<fieldN>`
    - Lists paths for all tracks missing one or more of the specified fields.
