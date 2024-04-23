# Condense
Condense the white space in a text file.
I feel like I always run into a collection of text files that for one reason or another are filled to the gills 
with terrible white space.
This cli utility helps to condense these files and remove duplicative white space characters. 

## Usage
Condense a string from `stdin` or read a file in.
If modifying a file, pass the `--inplace` flag to modify the file where it is.

```bash
$ condense --help
Remove repetitive white space characters

Usage: condense [OPTIONS] [DATA]

Arguments:
  [DATA]  An input string to condense

Options:
  -f, --file <FILE>  Should read from file instead of stdin
  -a, --aggressive   Whether to consider all consecutive white spaces as duplicates
      --one          aggressivly condense input to one line with only single spaces
  -i, --inplace      If sent a file file, edit in place
  -h, --help         Print help
  -V, --version      Print version
```
You can run the util over every file in a directory with

```bash
find <my_dir> -type f -print | xargs -I {} ./condense -f {}
```

And if you use [Parallel](https://www.gnu.org/software/parallel/), then you should be able to 
do the following to condense ever file in a directory concurrently

```bash
# I haven't tested this yet
find <my_dir> -type f -print | parallel -j4 ./condense -f {}
```
