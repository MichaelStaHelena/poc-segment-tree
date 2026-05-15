# Loan Contracts — Segment Tree com Objetos

Os arquivos `loan-contracts.py` e `loan-contracts.rs` mostram que a Segment Tree
**não precisa guardar escalares** — ela pode guardar qualquer objeto, desde que você
defina um **merge** e um **elemento neutro** compatíveis.

A mesma estrutura de árvore responde 5 perguntas de negócio sobre uma carteira de
8 contratos de empréstimo, trocando só o merge e o neutro em cada caso.

---

## O modelo: ContratoEmprestimo

Cada folha da árvore é um contrato com 4 atributos.

**Python**
```python
class ContratoEmprestimo:
    def __init__(self, contrato_id, devedor, valor, dias_para_pagar):
        self.contrato_id     = contrato_id
        self.devedor         = devedor
        self.valor           = valor            # saldo devedor em aberto (R$)
        self.dias_para_pagar = dias_para_pagar  # dias restantes até o vencimento
```

**Rust**
```rust
#[derive(Clone)]
struct ContratoEmprestimo {
    contrato_id:     u32,
    devedor:         String,
    valor:           f64,   // saldo devedor em aberto (R$)
    dias_para_pagar: i32,   // dias restantes até o vencimento
}
```

O `#[derive(Clone)]` no Rust é obrigatório porque a árvore precisa copiar contratos
durante o build e o update. Em Python isso é transparente — objetos são referências.

---

## A carteira inicial

```
#   Devedor   Valor (R$)   Dias
1   Alice      5.000,00    30
2   Bob       12.000,00     7
3   Carol      3.200,00    45
4   David      8.500,00    14
5   Eve        2.100,00     2   ← mais urgente E menor saldo
6   Frank      9.900,00    21
7   Grace      6.700,00    60  ← mais folgado
8   Hank       4.400,00     9
                          ----
    Bob       12.000,00       ← maior exposição individual
    Total     51.800,00       ← exposição total
```

---

## O padrão dos 5 use cases

Cada use case tem a mesma estrutura de 3 funções:

| Função       | O que faz                                              |
|--------------|--------------------------------------------------------|
| `build_*`    | Constrói a árvore a partir da carteira — O(n)          |
| `consulta_*` | Responde a pergunta para o intervalo `[l, r]` — O(log n) |
| `atualiza_*` | Atualiza um contrato e propaga a mudança — O(log n)    |

### O merge como função separada

Ambos os arquivos extraem o critério de comparação numa função `merge_*` dedicada,
chamada pelas três funções de cada use case (`build`, `consulta`, `atualiza`).

**Python**
```python
def merge_urg(e, d):
    return e if e.dias_para_pagar <= d.dias_para_pagar else d

# usado em build, consulta e atualiza:
tree_urg[no] = merge_urg(tree_urg[2 * no], tree_urg[2 * no + 1])
```

**Rust**
```rust
fn merge_urg(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.dias_para_pagar <= b.dias_para_pagar { a.clone() } else { b.clone() }
}

// usado em build, consulta e atualiza:
tree[no] = merge_urg(&e, &d);
```

O benefício é que a lógica de negócio (qual campo, qual critério) fica em **um único lugar**.
Se o critério mudar, você edita só `merge_urg` — não três funções.
A alternativa inline (`e if e.dias <= d.dias else d` repetido três vezes) funciona,
mas exige três edições para qualquer mudança e esconde o padrão.

A única coisa que muda entre eles:

| Use case          | Campo comparado    | Critério | Neutro              |
|-------------------|--------------------|----------|---------------------|
| 1 — Mais urgente  | `dias_para_pagar`  | menor    | `+inf` / `i32::MAX` |
| 2 — Mais folgado  | `dias_para_pagar`  | maior    | `-inf` / `i32::MIN` |
| 3 — Menor saldo   | `valor`            | menor    | `+inf` / `f64::INFINITY` |
| 4 — Maior exposição | `valor`          | maior    | `-inf` / `f64::NEG_INFINITY` |
| 5 — Exposição total | `valor` (escalar)| soma     | `0.0`               |

---

## Use case 1 — Contrato MAIS URGENTE (min por dias)

**Pergunta:** "Qual contrato cobrar primeiro no intervalo `[l, r]`?"

O contrato com **menos** `dias_para_pagar` vence o merge.
O neutro precisa **perder sempre** numa comparação de mínimo, então usa `+inf`.

### Elemento neutro e merge

**Python** — neutro como constante global, merge como função:
```python
INF = float("inf")
NEUTRO_URG = ContratoEmprestimo(0, "-", 0.0, INF)   # dias = +inf → nunca vence o min

def merge_urg(e, d):
    return e if e.dias_para_pagar <= d.dias_para_pagar else d
```

**Rust** — neutro como função, merge como função:
```rust
fn neutro_urg() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", 0.0, i32::MAX)
}

fn merge_urg(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.dias_para_pagar <= b.dias_para_pagar { a.clone() } else { b.clone() }
}
```

> **Diferença restante:** Python define o neutro como constante (`NEUTRO_URG = ...`) porque
> não tem problema de ownership — o mesmo objeto pode ser referenciado em vários lugares.
> Rust precisa de uma função que cria uma nova instância a cada chamada, porque cada retorno
> precisa ter ownership independente.

### build_urg

Constrói a árvore. Cada nó interno guarda o **contrato mais urgente** do seu intervalo.

**Python**
```python
def build_urg(no, inicio, fim):
    if inicio == fim:
        tree_urg[no] = contratos_urg[inicio]   # folha = contrato do array
        return
    meio = (inicio + fim) // 2
    build_urg(2 * no, inicio, meio)
    build_urg(2 * no + 1, meio + 1, fim)
    tree_urg[no] = merge_urg(tree_urg[2 * no], tree_urg[2 * no + 1])
```

**Rust**
```rust
fn build_urg(
    contratos: &[ContratoEmprestimo], tree: &mut [ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        tree[no] = contratos[inicio].clone();
        return;
    }
    let meio = (inicio + fim) / 2;
    build_urg(contratos, tree, 2 * no, inicio, meio);
    build_urg(contratos, tree, 2 * no + 1, meio + 1, fim);
    let e = tree[2 * no].clone();
    let d = tree[2 * no + 1].clone();
    tree[no] = merge_urg(&e, &d);   // merge extraído
}
```

> **Diferença de design:** Rust recebe `contratos` e `tree` como parâmetros explícitos
> porque não tem estado global. Python acessa `contratos_urg` e `tree_urg` diretamente
> por closure de escopo. Nas funções Rust, o `.clone()` é necessário para satisfazer o
> borrow checker — você não pode ter duas referências mutáveis ao mesmo Vec ao mesmo tempo.

### consulta_urg

Os 3 casos são idênticos ao template genérico — só o retorno do neutro e o merge mudam.

**Python**
```python
def consulta_urg(no, inicio, fim, l, r):
    if r < inicio or fim < l:              # CASO 1: fora → retorna neutro
        return NEUTRO_URG
    if l <= inicio and fim <= r:           # CASO 2: dentro → resposta pronta
        return tree_urg[no]
    meio = (inicio + fim) // 2             # CASO 3: parcial → desce e combina
    e = consulta_urg(2 * no, inicio, meio, l, r)
    d = consulta_urg(2 * no + 1, meio + 1, fim, l, r)
    return merge_urg(e, d)
```

**Rust**
```rust
fn consulta_urg(
    tree: &[ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize, l: usize, r: usize,
) -> ContratoEmprestimo {
    if r < inicio || fim < l { return neutro_urg(); }
    if l <= inicio && fim <= r { return tree[no].clone(); }
    let meio = (inicio + fim) / 2;
    let e = consulta_urg(tree, 2 * no, inicio, meio, l, r);
    let d = consulta_urg(tree, 2 * no + 1, meio + 1, fim, l, r);
    merge_urg(&e, &d)
}
```

### atualiza_urg

Desce até a folha, troca o contrato e recalcula o merge de volta até a raiz.

**Python**
```python
def atualiza_urg(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_urg[no] = novo
        contratos_urg[inicio] = novo       # sincroniza o array original
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_urg(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_urg(2 * no + 1, meio + 1, fim, posicao, novo)
    tree_urg[no] = merge_urg(tree_urg[2 * no], tree_urg[2 * no + 1])
```

**Rust**
```rust
fn atualiza_urg(
    contratos: &mut [ContratoEmprestimo], tree: &mut [ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize, posicao: usize, novo: ContratoEmprestimo,
) {
    if inicio == fim {
        tree[no] = novo.clone();
        contratos[inicio] = novo;
        return;
    }
    let meio = (inicio + fim) / 2;
    if posicao <= meio {
        atualiza_urg(contratos, tree, 2 * no, inicio, meio, posicao, novo);
    } else {
        atualiza_urg(contratos, tree, 2 * no + 1, meio + 1, fim, posicao, novo);
    }
    let e = tree[2 * no].clone();
    let d = tree[2 * no + 1].clone();
    tree[no] = merge_urg(&e, &d);
}
```

---

## Use case 2 — Contrato MAIS FOLGADO (max por dias)

**Pergunta:** "Qual contrato no intervalo pode esperar mais?"

Mesma estrutura do use case 1 — só o critério inverte: agora **mais** `dias_para_pagar` vence.
O neutro precisa perder no máximo, então usa `-inf`.

**Python**
```python
NEUTRO_FOLGA = ContratoEmprestimo(0, "-", 0.0, -INF)   # dias = -inf → nunca vence o max

def merge_folga(e, d):
    return e if e.dias_para_pagar >= d.dias_para_pagar else d
```

**Rust**
```rust
fn neutro_folga() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", 0.0, i32::MIN)
}

fn merge_folga(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.dias_para_pagar >= b.dias_para_pagar { a.clone() } else { b.clone() }
}
```

> As funções `build_folga`, `consulta_folga` são idênticas ao use case 1 trocando
> `merge_urg` → `merge_folga` e `neutro_urg` → `neutro_folga`.
> O use case 2 não tem `atualiza_folga` no arquivo — não era necessário para a demo,
> mas o padrão seria exatamente o mesmo do `atualiza_urg`.

---

## Use case 3 — MENOR SALDO devedor (min por valor)

**Pergunta:** "Qual contrato no intervalo tem o menor saldo em aberto?"

Agora o campo comparado muda de `dias_para_pagar` para `valor`. O critério é mínimo.

**Python**
```python
NEUTRO_MENOR = ContratoEmprestimo(0, "-", INF, 0)   # valor = +inf → nunca vence o min

def merge_menor(e, d):
    return e if e.valor <= d.valor else d
```

**Rust**
```rust
fn neutro_menor() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", f64::INFINITY, 0)
}

fn merge_menor(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.valor <= b.valor { a.clone() } else { b.clone() }
}
```

---

## Use case 4 — MAIOR EXPOSIÇÃO individual (max por valor)

**Pergunta:** "Qual contrato no intervalo representa a maior dívida?"

Campo: `valor`. Critério: máximo.

**Python**
```python
NEUTRO_MAIOR = ContratoEmprestimo(0, "-", -INF, 0)   # valor = -inf → nunca vence o max

def merge_maior(e, d):
    return e if e.valor >= d.valor else d
```

**Rust**
```rust
fn neutro_maior() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", f64::NEG_INFINITY, 0)
}

fn merge_maior(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.valor >= b.valor { a.clone() } else { b.clone() }
}
```

---

## Use case 5 — EXPOSIÇÃO TOTAL (sum por valor)

**Pergunta:** "Quanto a carteira `[l, r]` deve no total?"

Este use case é diferente dos outros em um ponto importante: **a soma de dois contratos
não é um contrato** — é um número (R$). Por isso a árvore guarda `float`/`f64`, não objetos.
As folhas extraem só o campo `valor`; os nós internos somam.

### build_soma

**Python**
```python
tree_soma = [0.0] * (4 * C)   # árvore de escalares, não de objetos

def build_soma(no, inicio, fim):
    if inicio == fim:
        tree_soma[no] = contratos_soma[inicio].valor   # extrai só o campo valor
        return
    meio = (inicio + fim) // 2
    build_soma(2 * no, inicio, meio)
    build_soma(2 * no + 1, meio + 1, fim)
    tree_soma[no] = tree_soma[2 * no] + tree_soma[2 * no + 1]   # merge: soma
```

**Rust**
```rust
fn build_soma(
    contratos: &[ContratoEmprestimo], tree: &mut [f64],   // tree é Vec<f64>, não Vec<Contrato>
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        tree[no] = contratos[inicio].valor;   // extrai só o campo valor
        return;
    }
    let meio = (inicio + fim) / 2;
    build_soma(contratos, tree, 2 * no, inicio, meio);
    build_soma(contratos, tree, 2 * no + 1, meio + 1, fim);
    tree[no] = tree[2 * no] + tree[2 * no + 1];   // merge: soma
}
```

### consulta_soma

**Python**
```python
def consulta_soma(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return 0.0          # neutro da soma
    if l <= inicio and fim <= r:
        return tree_soma[no]
    meio = (inicio + fim) // 2
    return (consulta_soma(2 * no, inicio, meio, l, r)
          + consulta_soma(2 * no + 1, meio + 1, fim, l, r))
```

**Rust**
```rust
fn consulta_soma(
    tree: &[f64],
    no: usize, inicio: usize, fim: usize, l: usize, r: usize,
) -> f64 {
    if r < inicio || fim < l { return 0.0; }
    if l <= inicio && fim <= r { return tree[no]; }
    let meio = (inicio + fim) / 2;
    consulta_soma(tree, 2 * no, inicio, meio, l, r)
      + consulta_soma(tree, 2 * no + 1, meio + 1, fim, l, r)
}
```

### atualiza_soma

Recebe um `ContratoEmprestimo` novo mas grava só o `valor` na árvore.

**Python**
```python
def atualiza_soma(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_soma[no] = novo.valor      # grava o escalar, não o objeto
        contratos_soma[inicio] = novo   # sincroniza o array de contratos
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_soma(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_soma(2 * no + 1, meio + 1, fim, posicao, novo)
    tree_soma[no] = tree_soma[2 * no] + tree_soma[2 * no + 1]
```

**Rust**
```rust
fn atualiza_soma(
    contratos: &mut [ContratoEmprestimo], tree: &mut [f64],
    no: usize, inicio: usize, fim: usize, posicao: usize, novo: ContratoEmprestimo,
) {
    if inicio == fim {
        tree[no] = novo.valor;        // grava o escalar, não o objeto
        contratos[inicio] = novo;
        return;
    }
    let meio = (inicio + fim) / 2;
    if posicao <= meio {
        atualiza_soma(contratos, tree, 2 * no, inicio, meio, posicao, novo);
    } else {
        atualiza_soma(contratos, tree, 2 * no + 1, meio + 1, fim, posicao, novo);
    }
    tree[no] = tree[2 * no] + tree[2 * no + 1];
}
```

---

## Por que não existe "sum por dias"?

Os 4 use cases de min/max retornam um **contrato existente** — o objeto já está na árvore.
A soma retorna um **valor derivado** (R$ total) — nenhum contrato individual representa isso.

Prazos funcionam em paralelo, não em série. Se Alice vence em 30 dias e Bob em 7,
Bob é pago no dia 7 **dentro** do prazo de Alice — você não espera 37 dias por nada.
Somar prazos produziria um número sem unidade real de negócio.

---

## Resumo: o que muda em cada use case

```
                    ┌─────────────────────────────────────────────────────┐
                    │              Segment Tree com Objetos                │
                    │                                                       │
  Use case 1        │  campo: dias_para_pagar   critério: min              │
  Use case 2        │  campo: dias_para_pagar   critério: max              │
  Use case 3        │  campo: valor             critério: min              │
  Use case 4        │  campo: valor             critério: max              │
  Use case 5        │  campo: valor             critério: sum → escalar    │
                    │                                                       │
                    │  build / consulta / atualiza: IDÊNTICOS              │
                    │  só o merge() e o neutro mudam                       │
                    └─────────────────────────────────────────────────────┘
```

---

## Diferenças Python vs Rust — resumo

| Aspecto | Python | Rust |
|---------|--------|------|
| Estado da árvore | variáveis globais | passado como parâmetro |
| Merge | função separada `merge_*()` | função separada `merge_*()` |
| Neutro | constante global (`NEUTRO_* = ...`) | função que cria nova instância `neutro_*()` |
| Cópia de objetos | transparente (referência) | `.clone()` explícito |
| Tipo `inf` | `float("inf")` / `-float("inf")` | `f64::INFINITY` / `f64::NEG_INFINITY` / `i32::MAX` |
| Árvore de escalares (use case 5) | `[0.0] * (4 * C)` | `vec![0.0f64; 4 * c]` |

---

## Arquivos de referência

- [`loan-contracts.py`](loan-contracts.py) — implementação Python completa com demos
- [`loan-contracts.rs`](loan-contracts.rs) — implementação Rust com demos e testes unitários
