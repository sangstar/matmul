## ruBLAS
Linear algebra implementations in Rust with bindings for Python.

## Installation
Make sure Rust and Python are installed on your machine, then clone the repository.

```bash
git clone https://github.com/sangstar/ruBLAS.git
```

Install `maturin` via pip.

```bash
pip install maturin
```

Build the crate and install it as a python module:

```bash
cd ruBLAS
maturin develop
```

For confirmation that the installation was successful, you can run this script
to see if the module is functioning properly.

```bash
python3 -m matmul
```