# zfc
zfc - file management utility. Helps you to complete, read, delete and create files 🌺

# Read file
1. If you want to output the text of a file:
  - `akumi <file>`
  - `akumi main.rs`

2. If you want to output the text of a file with line numbering:
  - `akumi <file> -n` (or --numeration)
  - `akumi main.rs -n`

3. If you want to output the text of a file with a certain number of lines:
  - `akumi <file> -l <lines count>` ( or --lines)
  - `akumi main.rs -l 3`
  - `akumi main.rs -l 3 -n` (*for numbering*)

# File operation
1. Delete file:
   - `akumi main.rs -d` ( or --delete)

2. Create file:
   - `akumi main.rs -c` (or --create)

3. Completing the text of a file:
   - `akumi main.rs -a "hello world"` (or --append)
