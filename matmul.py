import rublas

mat_a = rublas.Matrix([
    [1,2],
    [4,5],
    [1,2],
])

mat_b = rublas.Matrix([
    [7, 8,  9, 46],
    [10,11, 3, 12],
])

mat_c = rublas.Matrix([
    [32, 2],
    [1,  3],
    [-3, 4]
])

if __name__ == "__main__":
    print(mat_a * mat_b)
    print(mat_a + mat_c)