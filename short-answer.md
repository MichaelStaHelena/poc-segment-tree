# Segment Tree — Referência Rápida

Árvore binária sobre um array onde cada nó armazena o resultado agregado de um intervalo.
A raiz cobre o array inteiro; as folhas cobrem elementos individuais.

```
Array: [7, 3, 9, 5, 8, 2, 6, 4]

            [9]              ← raiz: max [0..7]
         /       \
      [9]         [8]        ← max [0..3] e [4..7]
     /   \       /   \
   [7]  [9]   [8]   [6]     ← max [0..1],[2..3],[4..5],[6..7]
   / \  / \   / \   / \
  [7][3][9][5][8][2][6][4]  ← folhas = array original
```

## Complexidade

| Operação      | Array simples | Segment Tree |
|---------------|:------------:|:------------:|
| Query range   | O(n)         | **O(log n)** |
| Update ponto  | O(1)         | O(log n)     |
| Update range  | O(n)         | O(log n) †   |

† com Lazy Propagation

---

## Operações

A mesma estrutura de árvore funciona para qualquer operação associativa.
Só mudam **duas coisas**: o **merge** e o **elemento neutro**.

| Operação | Elemento neutro | Merge                             |
|----------|:--------------:|-----------------------------------|
| SUM      | `0`            | `tree[no] = left + right`         |
| MAX      | `-inf`         | `tree[no] = max(left, right)`     |
| MIN      | `+inf`         | `tree[no] = min(left, right)`     |

O elemento neutro é o valor retornado quando um nó está **fora** do intervalo consultado —
deve ser o identidade da operação (soma com 0, max com -∞, min com +∞).

---

## Funções

### build — O(n)

Constrói a árvore de baixo pra cima. Nós folha recebem os valores do array;
nós internos recebem o merge dos dois filhos.

```python
def merge(a, b):
    return a + b        # SUM
    # return max(a, b)  # MAX
    # return min(a, b)  # MIN

# Chamada: build(1, 0, N - 1)
def build(no, inicio, fim):
    if inicio == fim:           # folha: guarda o valor diretamente
        tree[no] = array[inicio]
        return

    meio = (inicio + fim) // 2
    build(2 * no,     inicio, meio)      # filho esquerdo
    build(2 * no + 1, meio + 1, fim)     # filho direito

    tree[no] = merge(tree[2 * no], tree[2 * no + 1])
```

---

### query — O(log n)

Responde a consulta no intervalo `[l, r]` verificando 3 casos em cada nó.

```python
# Chamada: query(1, 0, N - 1, l, r)
def query(no, inicio, fim, l, r):

    # CASO 1: intervalo do nó está FORA do pedido → retorna neutro
    if r < inicio or fim < l:
        return 0          # neutro SUM
        # return -inf     # neutro MAX
        # return +inf     # neutro MIN

    # CASO 2: intervalo do nó está DENTRO do pedido → resposta pronta
    if l <= inicio and fim <= r:
        return tree[no]

    # CASO 3: sobreposição parcial → desce e combina
    meio = (inicio + fim) // 2
    esq = query(2 * no,     inicio, meio,    l, r)
    dir = query(2 * no + 1, meio + 1, fim,   l, r)
    return merge(esq, dir)
```

O **Caso 2** é o que garante O(log n): quando o nó já contém a resposta, para de descer.

---

### update — O(log n)

Atualiza um elemento e propaga a mudança de volta até a raiz.

```python
# Chamada: update(1, 0, N - 1, posicao, novo_valor)
def update(no, inicio, fim, posicao, novo_valor):

    # chegou na folha: atualiza e retorna
    if inicio == fim:
        tree[no] = novo_valor
        array[inicio] = novo_valor
        return

    meio = (inicio + fim) // 2

    # desce SÓ pelo lado que contém a posição
    if posicao <= meio:
        update(2 * no,     inicio, meio,    posicao, novo_valor)
    else:
        update(2 * no + 1, meio + 1, fim,   posicao, novo_valor)

    # ao subir, recalcula esse nó com o merge dos filhos atualizados
    tree[no] = merge(tree[2 * no], tree[2 * no + 1])
```

---

## Trace visual de uma query

**Pergunta:** qual o MAX no intervalo `[2, 5]`?
**Array:** `[7, 3, 9, 5, 8, 2, 6, 4]` — resposta esperada: `9`

```
query(1, 0, 7, l=2, r=5)

  nó 1  cobre [0..7]  → CASO 3 parcial  → desce
  ├── nó 2  cobre [0..3]  → CASO 3 parcial  → desce
  │   ├── nó 4  cobre [0..1]  → CASO 1 fora    → retorna -inf  ✗
  │   └── nó 5  cobre [2..3]  → CASO 2 dentro  → retorna 9     ✓
  │             max(-inf, 9) = 9
  └── nó 3  cobre [4..7]  → CASO 3 parcial  → desce
      ├── nó 6  cobre [4..5]  → CASO 2 dentro  → retorna 8     ✓
      └── nó 7  cobre [6..7]  → CASO 1 fora    → retorna -inf  ✗
                max(8, -inf) = 8

  resultado final: max(9, 8) = 9  ✓
```

Apenas **5 nós** visitados num array de 8 elementos.
Sem a árvore seriam 4 comparações lineares — com N = 1 milhão a diferença é ~20 vs ~1.000.000.

---

## Lazy Propagation — range update em O(log n)

Problema: atualizar **todos** os elementos de `[l, r]` (ex: somar 5 em cada um).
Sem lazy: seria necessário chamar `update` N vezes → O(n log n).
Com lazy: marca o nó com o update pendente e só propaga quando o filho for acessado.

```python
lazy = [0] * (4 * N)   # "débito" pendente de cada nó

def push_down(no):
    if lazy[no] != 0:
        for filho in (2 * no, 2 * no + 1):
            tree[filho] += lazy[no]   # aplica o débito
            lazy[filho] += lazy[no]   # repassa pro filho
        lazy[no] = 0                  # limpa o nó atual

def update_range(no, inicio, fim, l, r, valor):
    if r < inicio or fim < l:         # CASO 1: fora
        return
    if l <= inicio and fim <= r:      # CASO 2: dentro
        tree[no] += valor
        lazy[no] += valor
        return
    push_down(no)                     # CASO 3: propaga antes de descer
    meio = (inicio + fim) // 2
    update_range(2 * no,     inicio, meio,    l, r, valor)
    update_range(2 * no + 1, meio + 1, fim,   l, r, valor)
    tree[no] = tree[2 * no] + tree[2 * no + 1]

def query_range(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return 0
    if l <= inicio and fim <= r:
        return tree[no]
    push_down(no)                     # ← única diferença vs query simples
    meio = (inicio + fim) // 2
    return query_range(2 * no, inicio, meio, l, r) + \
           query_range(2 * no + 1, meio + 1, fim, l, r)
```

A regra é simples: **sempre chame `push_down` antes de descer** no Caso 3.

---

## Gotchas

### Por que `tree = [0] * (4 * N)` e não `2 * N`?

A árvore tem no máximo `2 * 2^⌈log₂(N)⌉` nós. Quando N não é potência de 2, o último
nível pode ter espaços vazios — e a fórmula `2*no + 1` pode acessar índices além de `2*N`.
`4*N` é o limite seguro que cobre qualquer N sem precisar calcular.

```
N = 5  →  árvore ocupa até índice 19  →  4*5 = 20  ✓
N = 5  →  2*N = 10  →  índice 19 estoura  ✗
```

### Por que a raiz fica no índice `1` e não `0`?

Com raiz em `1`, os filhos de qualquer nó `no` ficam em `2*no` e `2*no+1`.
Com raiz em `0`, a fórmula quebraria: filhos de `0` seriam `0` e `1` (loop infinito).

```
raiz = 1:  filhos de nó 3  →  6  e  7   ✓
raiz = 0:  filhos de nó 0  →  0  e  1   ✗ (nó 0 é filho de si mesmo)
```

O índice `0` do array simplesmente nunca é usado.

### Erros comuns na primeira implementação

| Erro | Sintoma | Correção |
|------|---------|----------|
| Usar `tree = [0] * (2 * N)` | `IndexError` ou resultados errados quando N não é potência de 2 | Use `4 * N` |
| Chamar `build(0, ...)` | Recursão infinita | Sempre comece em `build(1, 0, N-1)` |
| Esquecer `push_down` no query com lazy | Query retorna valor desatualizado | Chame `push_down` antes de descer no Caso 3 |
| Elemento neutro errado | Query em intervalo vazio retorna valor inválido | SUM→0, MAX→-inf, MIN→+inf |

---

## Quando usar

Quando você precisa de **muitas queries de intervalo com updates frequentes**
e O(n) por query é caro demais.

---

## Implementações completas no projeto

- [`simple-operations.py`](simple-operations.py) — MAX (notas), SUM (vendas), MIN (SpO2)
- [`simple-operations.rs`](simple-operations.rs) — mesma lógica em Rust
- [`loan-contracts.py`](loan-contracts.py) — objetos complexos com merge customizado
- [`loan-contracts.rs`](loan-contracts.rs) — mesma lógica em Rust