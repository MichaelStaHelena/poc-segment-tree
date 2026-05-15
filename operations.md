# Operações no visualizador de Contratos de Empréstimo

Este documento explica as operações disponíveis sobre uma carteira de contratos de empréstimo,
aplicadas via **segment tree** — tanto no visualizador interativo `animations/web/loan-contracts.html`
quanto nas implementações em [`loan-contracts.py`](loan-contracts.py) e [`loan-contracts.rs`](loan-contracts.rs).

A diferença para uma segment tree tradicional: aqui cada nó guarda **um contrato inteiro**
(objeto com vários atributos), não um número. Isso permite responder consultas como
"qual o contrato mais urgente entre os contratos X e Y?" em `O(log n)`.

---

## O dado: `ContratoEmprestimo`

Cada posição da carteira é um contrato com 4 atributos:

| Campo             | Tipo       | Significado                              |
|-------------------|------------|------------------------------------------|
| `contrato_id`     | inteiro    | Identificador único do contrato          |
| `devedor`         | texto      | Nome de quem deve                        |
| `valor`           | número (R$)| Saldo devedor em aberto                  |
| `dias_para_pagar` | inteiro    | Dias restantes até o vencimento          |

A carteira de exemplo tem 8 contratos (Alice, Bob, Carol, …, Hank), nas posições 0 a 7.

---

## As 3 funções da árvore

| Função        | Nome no código       | O que faz                                             | Custo    |
|---------------|----------------------|-------------------------------------------------------|----------|
| build         | `build_*`            | Monta a árvore a partir da carteira inicial           | `O(n)`   |
| consulta      | `consulta_*`         | Responde a pergunta para o intervalo `[l, r]`         | `O(log n)`|
| atualiza      | `atualiza_*`         | Substitui um contrato (renegociação, pagamento, etc.) | `O(log n)`|

O esqueleto das 3 funções é sempre o mesmo. O que muda entre os use cases é **apenas a função `merge_*`**
e o **elemento neutro** — ambos definidos uma única vez por use case e chamados nos 3 lugares.

---

## A função `merge_*` — o coração de cada use case

Cada use case define seu próprio merge e neutro:

| Use case               | Função merge (Python)                                      | Neutro                   |
|------------------------|------------------------------------------------------------|--------------------------|
| 1 — Mais urgente       | `return e if e.dias_para_pagar <= d.dias_para_pagar else d`| `dias_para_pagar = +inf` |
| 2 — Mais folgado       | `return e if e.dias_para_pagar >= d.dias_para_pagar else d`| `dias_para_pagar = -inf` |
| 3 — Menor saldo        | `return e if e.valor <= d.valor else d`                    | `valor = +inf`           |
| 4 — Maior exposição    | `return e if e.valor >= d.valor else d`                    | `valor = -inf`           |
| 5 — Exposição total    | `return a + b` (escalar)                                   | `0.0`                    |

> Nos use cases 1–4, o merge retorna **o contrato vencedor inteiro** (com nome, valor e dias).
> No use case 5, o merge retorna um **escalar (R$)** — por isso a árvore guarda `float`, não objetos.

---

## A matriz `op × campo` — o que cada combinação responde

| op    | campo  | Pergunta que responde                                 | Use case        | Faz sentido? |
|-------|--------|-------------------------------------------------------|-----------------|:------------:|
| `min` | `dias` | Qual contrato vence **antes** no intervalo?           | 1 — Mais urgente | ✅           |
| `max` | `dias` | Qual contrato vence **depois** no intervalo?          | 2 — Mais folgado | ✅           |
| `min` | `valor`| Qual o **menor saldo devedor** no intervalo?          | 3 — Menor saldo  | ✅           |
| `max` | `valor`| Qual a **maior exposição individual** no intervalo?   | 4 — Maior exposição | ✅        |
| `sum` | `valor`| **Quanto a carteira deve no total** no intervalo?     | 5 — Exposição total | ✅        |
| `sum` | `dias` | (somar prazos individuais)                            | —               | ❌ — ver abaixo |

---

## Por que `sum + dias` não faz sentido neste domínio

Esta é a pergunta importante. Tecnicamente é possível calcular — mas o resultado
**não representa nada no mundo real**.

### O argumento

Vencimentos são **datas paralelas**, não eventos sequenciais.

Considere dois contratos:

- Contrato A: vence em **7 dias**
- Contrato B: vence em **30 dias**

Os dois prazos rodam **ao mesmo tempo**, cada um no seu próprio cronômetro. No dia 7, A é pago —
e isso acontece **dentro** dos 30 dias de B, não depois.

Somar `7 + 30 = 37 dias` não representa:

- **Não é** tempo total de espera (você não espera 37 dias para nada).
- **Não é** soma de prazos úteis (prazo útil é um por contrato, não composto).
- **Não é** prazo médio (média seria `(7+30)/2`, não soma).
- **Não é** prazo da carteira (carteira não tem prazo único — tem distribuição).

### A analogia mais simples

Somar a idade de 4 pessoas numa sala não dá "a idade da sala". Idade é uma característica
**individual de cada pessoa**, que roda no cronômetro de cada uma. Somar não compõe nada.

O mesmo vale para `dias_para_pagar`: é uma característica **individual de cada contrato**,
contada do hoje até o vencimento daquele contrato. A soma é só um número sem unidade semântica.

### O que faz sentido para cada campo

| Campo             | `min` | `max` | `sum` |
|-------------------|:-----:|:-----:|:-----:|
| `dias_para_pagar` | ✅ quem vence primeiro | ✅ quem tem mais folga | ❌ sem significado |
| `valor`           | ✅ menor saldo | ✅ maior saldo | ✅ exposição total (R$ + R$ = R$) |

A diferença é que `valor` é uma **quantidade aditiva** (somar reais resulta em reais),
enquanto `dias_para_pagar` é um **prazo individual** (somar prazos paralelos não resulta em nada).

---

## Como usar o visualizador

1. **Selecione `op` e `campo`** — define a pergunta que a árvore vai responder.
2. **Clique `build`** — monta a árvore. Necessário a cada troca de `op` ou `campo`.
3. **`consulta l r`** — consulta o intervalo `[l, r]`. O visualizador anima a descida pela árvore
   mostrando os 3 casos: FORA, DENTRO, PARCIAL.
4. **`atualiza pos`** — substitui o contrato na posição `pos`. A árvore recalcula só o caminho
   folha → raiz em `O(log n)`.

### Casos de uso reais

| Pergunta do time de cobrança                        | Combinação         |
|-----------------------------------------------------|--------------------|
| "Qual contrato cobrar primeiro hoje?"               | `min` + `dias`     |
| "Qual cliente do segmento X tem a maior dívida?"    | `max` + `valor`    |
| "Quanto a carteira [X, Y] vale no total?"           | `sum` + `valor`    |
| "Qual o cliente com mais folga de prazo?"           | `max` + `dias`     |
| "Qual o menor ticket no segmento?"                  | `min` + `valor`    |

---

## Resumo de uma linha

> Segment tree + objetos ricos = **uma árvore que responde várias perguntas diferentes sobre a mesma carteira**,
> com custo `O(log n)` por consulta. Só lembre que nem toda combinação `op × campo` tem significado
> no domínio — somar dias é o exemplo clássico.

---

## Referências

- [`loan-contracts.py`](loan-contracts.py) — implementação Python com os 5 use cases
- [`loan-contracts.rs`](loan-contracts.rs) — implementação Rust com testes unitários
- [`loan-contracts-explained.md`](loan-contracts-explained.md) — explicação detalhada de cada função e use case
