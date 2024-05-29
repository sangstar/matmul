## ruBLAS
Linear algebra implementations in Rust with bindings for Python.


```bash
python3 -c "import rublas; print(rublas.matmul([[1,2],[3,4]],[[5,6],[7,8]]))"
[[19, 22], [43, 50]]
```

## Installation
Make sure Rust and Python are installed on your machine, then clone the repository.

```bash
git clone https://github.com/sangstar/ruBLAS.git
```

Install `maturin` via pip.

```bash
pip install maturin
```

Finally, build the crate and install it as a python module.

```bash
cd ruBLAS
maturin develop
```