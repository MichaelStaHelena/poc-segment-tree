# Operações no visualizador de Contratos de Empréstimo

Este documento explica o visualizador interativo `animations/web/loan-contracts.html`, que aplica uma **segment tree** sobre uma carteira de contratos de empréstimo.

A diferença pra um segment tree tradicional: aqui cada nó guarda **um contrato inteiro** (objeto com vários atributos), não um número. Isso permite responder consultas como "qual o contrato mais urgente entre os contratos X e Y?" em `O(log n)`.

---

## O dado: `LoanContract`

Cada posição da carteira é um contrato com 4 atributos:

| Campo | Tipo | Significado |
|---|---|---|
| `id` | inteiro | Identificador único do contrato |
| `devedor` | texto | Nome de quem deve |
| `valor` | número (R$) | Saldo devedor em aberto |
| `dias_para_pagar` | inteiro | Dias restantes até o vencimento |

A carteira de exemplo tem 8 contratos (Alice, Bob, Carol, …, Hank), cada um nas posições 0 a 7.

---

## As 3 operações da árvore

| Operação | O que faz | Custo |
|---|---|---|
| `build` | Monta a árvore a partir da carteira inicial | `O(n)` |
| `query l, r` | Responde a pergunta para o intervalo `[l, r]` da carteira | `O(log n)` |
| `update pos, devedor, valor, dias` | Substitui o contrato na posição `pos` (renegociação, pagamento parcial, etc.) | `O(log n)` |

O esqueleto da árvore é sempre o mesmo. O que muda é **como dois nós são combinados** — isso é controlado pelos dois seletores:

- **`op`**: `min` / `max` / `sum`
- **`campo`**: `dias` / `valor`

---

## A matriz `op × campo` — o que cada combinação responde

| op | campo | Pergunta que responde | Faz sentido? |
|---|---|---|---|
| `min` | `dias` | Qual contrato vence **antes** no intervalo? (mais urgente) | ✅ |
| `max` | `dias` | Qual contrato vence **depois** no intervalo? (mais folga) | ✅ |
| `min` | `valor` | Qual o **menor saldo devedor** no intervalo? | ✅ |
| `max` | `valor` | Qual a **maior exposição individual** no intervalo? | ✅ |
| `sum` | `valor` | **Quanto a carteira deve no total** no intervalo? | ✅ |
| `sum` | `dias` | (somar prazos) | ❌ — explicação abaixo |

> Em `min` e `max`, o nó da árvore guarda **o contrato vencedor inteiro** (com nome, valor e dias). Em `sum`, o nó guarda apenas a **soma escalar** do campo escolhido.

---

## Por que `sum + dias` não faz sentido neste domínio

Esta é a pergunta importante. Tecnicamente o visualizador deixa selecionar `sum + dias` e calcula a soma — mas o resultado **não representa nada no mundo real**.

### O argumento

Vencimentos são **datas paralelas**, não eventos sequenciais.

Considere dois contratos:

- Contrato A: vence em **7 dias**
- Contrato B: vence em **30 dias**

Os dois prazos rodam **ao mesmo tempo**, cada um no seu próprio cronômetro. No dia 7, A é pago — e isso acontece **dentro** dos 30 dias do B, não depois. Você não fica "esperando A pra começar a esperar B".

Somar `7 + 30 = 37 dias` não representa:

- **Não é** tempo total de espera (você não espera 37 dias para nada).
- **Não é** soma de prazos úteis (prazo útil é um por contrato, não composto).
- **Não é** prazo médio (média seria `(7+30)/2`, não soma).
- **Não é** prazo da carteira (carteira não tem prazo único — tem distribuição).

### A analogia mais simples

Somar a idade de 4 pessoas numa sala não dá "a idade da sala". Idade é uma característica **individual de cada pessoa**, que roda no cronômetro de cada uma. Somar não compõe nada.

O mesmo vale para `dias_para_pagar`: é uma característica **individual de cada contrato**, contada do hoje até o vencimento daquele contrato. A soma é só um número sem unidade semântica.

### O que **faz** sentido para o campo `dias`

- `min(dias)` → quem vence primeiro (responde "quem cobrar agora?")
- `max(dias)` → quem tem mais folga (responde "quem deixar pro fim?")
- Distribuição / histograma → spread de vencimentos (não é segment tree)

E para o campo `valor`, **todas** as três operações fazem sentido:

- `min(valor)` → menor saldo
- `max(valor)` → maior saldo
- `sum(valor)` → exposição total — `R$ + R$ = R$ agregado`, é uma soma com unidade real

A diferença é que `valor` é uma **quantidade aditiva** (somar reais com reais resulta em reais), enquanto `dias_para_pagar` é um **prazo individual** (somar prazos paralelos não resulta em nada).

---

## Como usar o visualizador

1. **Selecione `op` e `campo`** — define a pergunta que a árvore vai responder.
2. **Clique `build`** — recalcula a árvore inteira a partir da carteira inicial. Cada vez que você muda `op` ou `campo` precisa rebuildar.
3. **`query l r`** — consulta o intervalo `[l, r]` da carteira. O visualizador anima a descida pela árvore mostrando os 3 casos do algoritmo (FORA, DENTRO, PARCIAL).
4. **`update pos, devedor, valor, dias`** — substitui o contrato na posição `pos` (simula renegociação). A árvore recalcula só o caminho da folha até a raiz, em `O(log n)`.

### Casos de uso reais (pergunta de negócio → combinação)

| Pergunta do time de cobrança | Combinação |
|---|---|
| "Qual contrato cobrar primeiro hoje?" | `min` + `dias` |
| "Qual cliente do segmento X tem a maior dívida?" | `max` + `valor` |
| "Quanto a carteira [X, Y] vale no total?" | `sum` + `valor` |
| "Qual o cliente com mais folga de prazo?" | `max` + `dias` |
| "Qual o menor ticket no segmento?" | `min` + `valor` |

---

## Resumo de uma linha

> Segment tree + objetos ricos = **uma árvore que responde várias perguntas diferentes sobre a mesma carteira**, com custo `O(log n)` por consulta. Só lembre que nem toda combinação `op × campo` tem significado no domínio — somar dias é o exemplo clássico.
