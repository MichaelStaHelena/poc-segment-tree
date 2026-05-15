# POC — Segment Tree

Prova de conceito explorando Segment Tree com Python e Rust, aplicada a cinco casos de uso reais sobre contratos de empréstimo.

---

## Implementações

| Arquivo | Conteúdo |
|---------|----------|
| [`simple-operations.py`](simple-operations.py) | MAX (notas), SUM (vendas), MIN (SpO₂) em Python |
| [`simple-operations.rs`](simple-operations.rs) | Mesma lógica em Rust |
| [`loan-contracts.py`](loan-contracts.py) | 5 use cases com objetos `ContratoEmprestimo` em Python |
| [`loan-contracts.rs`](loan-contracts.rs) | Mesma lógica em Rust + testes unitários |
| [`benchmark.py`](benchmark.py) | Comparação: força bruta O(n) vs Segment Tree O(log n) |

## Visualizador

[`animations/web/loan-contracts.html`](animations/web/loan-contracts.html) — árvore interativa com animação do traversal (build, query, update).

---

## Os 5 use cases — contratos de empréstimo

| Use case | Operação | Campo | Pergunta de negócio |
|----------|:--------:|-------|---------------------|
| 1 — Mais urgente   | min | `dias_para_pagar` | Qual contrato cobrar primeiro no intervalo? |
| 2 — Mais folgado   | max | `dias_para_pagar` | Qual contrato pode esperar? |
| 3 — Menor saldo    | min | `valor`           | Menor dívida individual no intervalo? |
| 4 — Maior exposição| max | `valor`           | Maior dívida individual no intervalo? |
| 5 — Exposição total| sum | `valor`           | Total da carteira no intervalo? |

---

## Como rodar

```bash
# Python
python simple-operations.py
python loan-contracts.py
python benchmark.py

# Rust
rustc simple-operations.rs && ./simple-operations
rustc loan-contracts.rs && ./loan-contracts

# Testes Rust
rustc --test loan-contracts.rs -o loan-contracts-test && ./loan-contracts-test
```

---

## Documentação

| Arquivo | Conteúdo |
|---------|----------|
| [`short-answer.md`](short-answer.md) | Referência rápida: estrutura, operações, trace visual, lazy, gotchas |
| [`loan-contracts-explained.md`](loan-contracts-explained.md) | Explicação detalhada de cada função e use case (Python + Rust) |
| [`operations.md`](operations.md) | Domínio de contratos: matriz op×campo, por que sum+dias não faz sentido |
| [`binary-tree-vs-segment-tree.md`](binary-tree-vs-segment-tree.md) | Diferença entre BST e Segment Tree |
| [`advanced-topics.md`](advanced-topics.md) | Lazy propagation, variantes, BIT vs Segment Tree, armadilhas |
| [`perguntas-e-respostas.md`](perguntas-e-respostas.md) | Banco de Q&A organizado por tema |
| [`presentation.md`](presentation.md) | Script de apresentação (versão 5 min e 15 min) |
