# HBFileSorter

A Humble Bundle book sorter for Calibre import, because i keep deleting the "temporary" python script


## Documentation

Move the executable to the directory containing all the loose `.pdf` and `.epub` files. 
When executing the executable, it creates multiple folders named after the `.pdf` or `.epub` files and places the corresponding files in them. 
This allows for bulk importing of books with multiple extensions.


## Run Locally

Clone the project

```bash
  git clone git@github.com:GertJan84/HBFileSorter.git
```

Go to the project directory

```bash
  cd HBFileSorter
```

Install dependencies

```bash
  cargo install
```

Build executable

```bash
  cargo build --release
```


## License

[MIT](https://choosealicense.com/licenses/mit/)

