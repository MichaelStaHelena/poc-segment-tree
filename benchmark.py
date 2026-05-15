# benchmark.py
#
# Compara forca bruta O(n) vs Segment Tree O(log n)
# em consultas de range query sobre arrays grandes.
#
# Como rodar:
#   python benchmark.py

import time
import random


# ==========================================================
# SEGMENT TREE (soma) - O(log n) por consulta
# ==========================================================

def build(arr, tree, no, inicio, fim):
    if inicio == fim:
        tree[no] = arr[inicio]
        return
    meio = (inicio + fim) // 2
    build(arr, tree, 2 * no, inicio, meio)
    build(arr, tree, 2 * no + 1, meio + 1, fim)
    tree[no] = tree[2 * no] + tree[2 * no + 1]


def range_query(tree, no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return 0
    if l <= inicio and fim <= r:
        return tree[no]
    meio = (inicio + fim) // 2
    return (range_query(tree, 2 * no, inicio, meio, l, r) +
            range_query(tree, 2 * no + 1, meio + 1, fim, l, r))


# ==========================================================
# FORCA BRUTA - O(n) por consulta
# ==========================================================

def brute_force(arr, l, r):
    return sum(arr[l:r + 1])


# ==========================================================
# BENCHMARK
# ==========================================================

def rodar_benchmark(n, num_queries):
    print(f"  Array: {n:,} elementos  |  Queries: {num_queries:,}")

    arr = [random.randint(1, 100) for _ in range(n)]

    # pre-build da segment tree (custo unico, nao entra no benchmark)
    tree = [0] * (4 * n)
    build(arr, tree, 1, 0, n - 1)

    # gera as queries antes de medir (sem contar geracao no tempo)
    queries = [(random.randint(0, n - 1), random.randint(0, n - 1)) for _ in range(num_queries)]
    queries = [(min(l, r), max(l, r)) for l, r in queries]

    # --- forca bruta ---
    t0 = time.perf_counter()
    for l, r in queries:
        brute_force(arr, l, r)
    t_bruta = time.perf_counter() - t0

    # --- segment tree ---
    t0 = time.perf_counter()
    for l, r in queries:
        range_query(tree, 1, 0, n - 1, l, r)
    t_seg = time.perf_counter() - t0

    speedup = t_bruta / t_seg if t_seg > 0 else float("inf")

    print(f"  Forca bruta  : {t_bruta:.3f}s")
    print(f"  Segment tree : {t_seg:.3f}s")
    print(f"  Speedup      : {speedup:.0f}x mais rapido")
    print()


print("=" * 52)
print("  BENCHMARK: Forca Bruta vs Segment Tree")
print("=" * 52)
print()

cenarios = [
    (10_000,   10_000),
    (100_000,  10_000),
    (1_000_000, 10_000),
]

for n, q in cenarios:
    rodar_benchmark(n, q)

print("Conclusao: o speedup cresce com n.")
print("O(n) escala linear. O(log n) mal sente o aumento.")
