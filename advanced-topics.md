# Segment Tree — Tópicos Avançados

Material de aprofundamento para quem já entendeu o básico.

Este documento cobre:
1. **Lazy propagation** — range updates em O(log n), com a armadilha clássica
2. **Variantes** — iterativa "flat", persistente, 2D, merge sort tree
3. **Segment Tree vs Fenwick Tree (BIT)** — quando escolher cada uma
4. **Recursiva vs iterativa** — layout de memória (4n vs 2n)
5. **Armadilhas e depuração**
6. **Origem histórica**

---

## 1. Lazy Propagation

### O problema

Atualizar **um único elemento** é O(log n) — o caminho da folha até a raiz tem altura logarítmica.
Mas e atualizar **um intervalo inteiro**? Ex: "some +10 em todas as posições de 2 a 5"?

Fazer isso folha por folha é O(n). Para manter O(log n), usamos **lazy propagation**.

### A ideia

> "Não atualize os descendentes agora. Deixe uma **marca** dizendo *'há um +10 pendente para este intervalo'* e só propague essa marca para os filhos quando **realmente precisar** descer."

Cada nó passa a ter dois valores:
- `tree[node]` — o agregado "correto" para o intervalo daquele nó
- `lazy[node]` — o delta pendente que ainda não foi propagado para os filhos

### Fluxo mental

```
Entrar no nó:
  Tem lazy pendente?
    Sim → aplica delta ao valor do nó,
          acumula delta nos filhos (se não for folha),
          zera lazy do nó atual.
    Não → segue.

  Relação com o intervalo pedido?
    Fora           → retorna (neutro).
    Totalmente dentro → atualiza nó, marca lazy nos filhos. Não desce.
    Parcial        → desce para os dois filhos, combina.
```

### ⚠️ A armadilha clássica (soma de intervalo)

Quando o agregado é **soma** e você aplica `+x` a um nó que cobre um segmento de tamanho `len`:

```python
# ERRADO — só funcionaria para min/max:
tree[node] += x

# CERTO — soma precisa multiplicar pelo tamanho do segmento:
tree[node] += x * len
```

Se o intervalo tem 8 elementos e você "adiciona 3 em cada", a soma total não sobe 3 — sobe 24. Esse detalhe é o bug mais comum em implementações de lazy para soma.

### Para min/max, não há essa armadilha

```python
# min: aplicar +x mantém todo mundo com o mesmo delta
tree[node] += x     # basta somar no agregado
```

Porque o mínimo de `[a+x, b+x, c+x]` é `min(a,b,c) + x` — o delta passa direto.

### Ver o código pronto em [operations.py](operations.py)

A classe `LazySegmentTree` implementa exatamente isso — com o `* (end - start + 1)` aplicado corretamente.

---

## 2. Variantes

A segment tree é um **esqueleto** — várias estruturas especializadas são montadas sobre ele.

| Variante              | Quando usar                             | Custo típico         | Trade-off                                               |
|-----------------------|-----------------------------------------|----------------------|---------------------------------------------------------|
| **Lazy propagation**  | Muitos updates em intervalo             | query/update O(log n)| Precisa de `lazy[]` e disciplina de `push`              |
| **Iterativa "flat"**  | Código enxuto, constantes menores       | build/query/update O(log n) | Trabalha com `[l, r)` (semiaberto) — confunde         |
| **Persistente**       | Consultar **versões antigas** da árvore | update cria O(log n) nós novos | Código com ponteiros/versões é mais complexo          |
| **2D**                | Consultas em **submatrizes**            | O(log² n)            | Memória e implementação crescem bastante                |
| **Merge sort tree**   | Buscas ordenadas por intervalo          | build O(n log n), query O(log² n) | Cada nó guarda uma **lista ordenada** do seu intervalo |

### Persistente, em uma frase
Em vez de sobrescrever, um update **cria apenas os O(log n) nós no caminho alterado**, compartilhando o resto com a versão anterior. Permite ter N árvores "empilhadas no tempo" com custo total O(n + u·log n) em espaço.

### 2D, em uma frase
"Árvore de árvores": cada nó da árvore externa (por linha) contém uma segment tree interna (por coluna). Responde "soma/máx de um retângulo" em O(log² n).

### Merge sort tree, em uma frase
Cada nó guarda uma **cópia ordenada** do seu segmento. Permite responder "quantos valores em `[l, r]` são ≤ k?" em O(log² n) — útil para rankings e k-ésimo elemento.

---

## 3. Segment Tree vs Fenwick Tree (BIT)

Fenwick tree **não é pior** — é mais especializada. A forma normal é ótima para **prefixos** com memória O(n) e código muito curto.

| Critério                         | Fenwick tree (BIT)                        | Árvore de segmentos                        |
|----------------------------------|-------------------------------------------|--------------------------------------------|
| **Estrutura base**               | Um vetor de somas parciais                | Partição binária explícita de intervalos   |
| **Melhor caso**                  | Prefixos e somas                          | Intervalos arbitrários, merges gerais      |
| **Memória**                      | O(n)                                      | O(n), tipicamente 4n recursiva / 2n iterativa |
| **Range update + range query**   | Possível, mas requer "truques" (2 BITs)   | Natural via lazy propagation               |
| **Min/max e agregados custom**   | Menos natural, com limitações             | Natural — basta trocar `merge` e neutro    |
| **Facilidade de código**         | Bem mais curto (~10 linhas)               | Mais geral, porém mais detalhado           |

### Regra prática

- **Só soma/prefixo com point update?** → Fenwick. Mais simples, constantes menores.
- **Min, max, XOR, GCD?** → Segment tree. BIT tem limitações em operações não-invertíveis.
- **Range update em intervalo?** → Segment tree com lazy. Mais natural.
- **Código curto em maratona de programação?** → Fenwick, se couber no problema.

### Por que BIT tem limitação com min/max?
BIT explora que a soma é **invertível** (tem subtração). Para reverter uma soma de prefixo e obter a soma de um intervalo, você faz `prefix(r) - prefix(l-1)`. **Mínimo não tem inverso** — não existe "anti-mínimo". Por isso min/max em BIT funciona só em casos restritos.

---

## 4. Recursiva vs Iterativa

Duas formas de implementar a mesma estrutura, com layouts diferentes.

### Recursiva ("implícita", top-down)
- Começa no nó `1` (raiz).
- Filhos do nó `v` são `2v` e `2v+1`.
- **Memória reservada: 4n** (cobre qualquer `n`, potência de 2 ou não).
- Mais fácil de ensinar — a estrutura "dividir, resolver filhos, combinar" fica explícita.
- Escolha padrão quando há lazy propagation elaborada.

### Iterativa "flat" (bottom-up)
- Folhas ocupam o trecho `[n, 2n)` do vetor.
- Pais calculados subindo: `tree[i]` = `merge(tree[2i], tree[2i+1])`.
- **Memória: ~2n** (metade da recursiva).
- Trabalha naturalmente com intervalos **semiabertos `[l, r)`**.
- Código mais curto e com constantes menores — preferida em competições.

### Regra prática

| Situação                                   | Escolha    |
|--------------------------------------------|------------|
| Estudando ou ensinando                     | Recursiva  |
| Merge rico (lazy, árvore 2D)               | Recursiva  |
| Código curto, tempo apertado               | Iterativa  |
| Muita memória em jogo                      | Iterativa  |

---

## 5. Armadilhas e Depuração

### Lista curta de revisão antes de submeter

- [ ] Convenção: é `[l, r]` (inclusivo) ou `[l, r)` (semiaberto)? Decida no **início** e mantenha.
- [ ] Array: **0-based** ou **1-based**? Não misture.
- [ ] Elemento neutro correto:
  - soma → `0`
  - min → `+infinito`
  - max → `-infinito`
  - XOR → `0`
  - GCD → `0`
  - produto → `1`
- [ ] Em lazy: sempre **`push` antes de descer** em `query()` e `update()`.
- [ ] Em lazy de **soma**: lembrar de multiplicar o delta pelo tamanho do segmento.
- [ ] Não assumir que a árvore é perfeita quando `n` não é potência de 2 — use `mid = (l + r) // 2` e respeite limites.

### Estratégia de depuração

> **Teste contra força bruta em arrays pequenos.**

```python
import random

def brute_force_sum(arr, l, r):
    return sum(arr[l:r+1])

# gera teste aleatório
arr = [random.randint(0, 10) for _ in range(8)]
st = SegmentTree(arr, op="sum")

for _ in range(1000):
    l = random.randint(0, 7)
    r = random.randint(l, 7)
    assert st.range_query(l, r) == brute_force_sum(arr, l, r)
```

Se bater em 1000 iterações aleatórias com `n=8`, provavelmente está correto.

### Em C++

Compile com sanitizers ao investigar comportamentos estranhos:
```bash
g++ -std=c++17 -O0 -g -fsanitize=address,undefined segtree.cpp
```
Pega acesso fora do vetor e comportamento indefinido na hora.

---

## 6. Origem Histórica

A ideia de segment tree **não nasceu em competições de programação**. Ela vem da **geometria computacional** — de problemas como "quais retângulos cobrem este ponto?" e "quais intervalos se sobrepõem a este?"

A referência clássica é o trabalho de **Jon Bentley (1980)** em verificação de layouts VLSI — ele precisava descobrir rapidamente quais fios de um chip cruzavam uma dada coordenada.

Na forma moderna usada em programação competitiva e entrevistas, ela é uma **árvore binária implícita sobre um vetor**, onde cada nó armazena uma agregação de um intervalo contíguo. Mas a essência é a mesma: **dividir o espaço em intervalos canônicos e pré-computar respostas para eles.**

### Frase-chave para memorizar

> **Segment tree é divisão e conquista transformada em estrutura de dados para intervalos dinâmicos.**

---

## Mnemônico final

> *"Pré-compute respostas de metades; consulte combinando poucas metades; atualize só o caminho afetado; adie updates de intervalo com lazy."*

Essas quatro frases descrevem build, query, point update e range update — a estrutura inteira.
