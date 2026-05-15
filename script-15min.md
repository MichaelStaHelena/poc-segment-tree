# Script — Apresentação 15 Minutos (Time)

Apresentação para engenheiros de software. Tom: par pra par.
Estrutura: hook → conceito → demo ao vivo → trade-offs → reflexão → perguntas.

---

## 🎤 Abertura — **2 min**

> **[Sem slides. Fala direto.] **

"Recebi um tema: **segment tree**.

Primeira reação: mais uma estrutura de dados de livro que nunca vou usar no dia a dia.

Aí me fiz uma pergunta: **onde isso aparece no que a gente faz de verdade?**

Trabalho com contratos de empréstimo. Perguntas como
*'qual o contrato mais urgente nesse range de IDs?'*,
*'qual a maior exposição num intervalo de clientes?'*,
*'qual o total da carteira de um segmento?'* — isso aparece o tempo todo.

Então a POC virou isso: **segment tree aplicada a contratos de empréstimo**.

E o que descobri é que a estrutura é mais elegante do que eu esperava — e mais útil do que parece à primeira vista."

---

## 📐 O Que É — **3 min**

> **[Abrir `animations/web/loan-contracts.html` no navegador. Deixar visível enquanto fala.]**

"Segment tree é uma **árvore binária** onde cada nó guarda um resumo de um pedaço do array.

As folhas guardam os valores reais. Os nós internos guardam agregados — máximo, mínimo, soma — de um intervalo.

A sacada é simples: em vez de percorrer o array elemento por elemento para responder uma pergunta, você **combina poucos blocos prontos**.

Para 1 milhão de elementos, cada consulta faz cerca de **20 passos**. Não importa o tamanho — é sempre O(log n).

Tem três operações:
- **Build** — constrói a árvore uma vez, O(n)
- **Range query** — consulta um intervalo, O(log n)
- **Point update** — atualiza uma posição, O(log n)

Range update em intervalo também é O(log n), com lazy propagation — não entra hoje, mas está documentado se quiserem se aprofundar.

O que mais me surpreendeu: **a estrutura é sempre a mesma**. O que muda é só a função de merge — máximo, mínimo ou soma. Isso resolve famílias inteiras de problemas com o mesmo código."

---

## 🖥️ Demo ao Vivo — **5 min**

> **[Usar `animations/web/loan-contracts.html` — já está aberto.]**

"Deixa eu mostrar na prática com contratos de empréstimo.

Temos um array de contratos — cada um com devedor, valor e dias para pagar.

**[Selecionar operação: MIN | campo: dias_para_pagar]**

Primeira pergunta: *qual o contrato mais urgente entre as posições 1 e 4?*
A árvore percorre os nós relevantes e retorna o mínimo de dias — **sem olhar os outros contratos**.

**[Executar a query, mostrar o traversal destacado na tela]**

Vejam os nós que ficam em cinza — a árvore nem toca neles. É exatamente isso que dá O(log n).

**[Trocar operação: MAX | campo: valor]**

Segunda pergunta: *qual a maior exposição entre as posições 2 e 5?*

**[Executar]**

Mesma estrutura, merge diferente. O código da árvore não mudou — só a função que combina os nós.

**[Trocar operação: SUM | campo: valor]**

Terceira: *qual o total da carteira nesses contratos?*

**[Executar]**

Agora um update: um contrato foi renegociado, prazo estendido.

**[Fazer point update em uma posição]**

A árvore recalcula só o caminho daquele nó até a raiz. O restante permanece intacto.

Isso é o núcleo: **divide o array em intervalos canônicos, pré-computa respostas, combina na hora da consulta, atualiza só o caminho afetado**."

---

## ⚠️ Quando NÃO Usar — **2 min**

> **[Slide ou fala direta — sem precisar de visual elaborado]**

"Segment tree tem overhead. Antes de usar, vale a checagem:

| Situação | Use em vez disso |
|----------|-----------------|
| Array estático, só soma | **Prefix sum** — O(1) query |
| Array estático, min/max | **Sparse table** — O(1) query |
| Só soma + point update | **Fenwick tree** — código 3× menor |
| n pequeno (< ~500) | Loop simples |
| Dados no banco com índice | Deixa o banco resolver |

A segment tree brilha quando você tem **intervalo arbitrário + update frequente + merge customizado**. Fora desse triângulo, avalie antes de implementar."

---

## 💭 O Que Eu Aprenderia Diferente — **1 min**

"Três coisas que faria diferente se começasse de novo:

**Primeiro** — começaria direto pelo caso real, contratos de empréstimo, antes dos exemplos genéricos. Quando você ancora no domínio, o algoritmo faz mais sentido desde o início.

**Segundo** — construiria o visualizador mais cedo. Visualizar o traversal da árvore clarificou em 10 minutos o que horas de leitura não conseguiram.

**Terceiro** — implementaria a versão iterativa antes da recursiva. É menor, mais próxima do que aparece em produção, e força você a entender o layout de memória de verdade.

No final, o que mais ficou foi isso: **a mesma estrutura resolve domínios completamente diferentes só trocando o merge**. Isso é design de software — não só algoritmo."

---

## ❓ Perguntas — **2 min**

> **[Banco completo em `perguntas-e-respostas.md`]**

Respostas rápidas para as mais prováveis:

- *"Quando usar Fenwick vs segment tree?"* → Só soma/prefixo → Fenwick. Min, max, merge customizado → segment tree.
- *"E lazy propagation?"* → Range update em O(log n). Está em `advanced-topics.md` com código e armadilha clássica.
- *"Tem em produção?"* → Editores de texto (índice de linhas), trading (order book), monitoramento (métricas por janela). Qualquer lugar com "lista muda + perguntas sobre pedaços".
- *"Por que não usar banco?"* → Banco resolve a maioria. Segment tree entra quando você precisa de O(log n) garantido na aplicação — sem round-trip, sem query planner.

---

## ⏱️ Checklist de Tempo

- [ ] Abertura: **2 min**
- [ ] O que é: **3 min**
- [ ] Demo ao vivo: **5 min**
- [ ] Quando não usar: **2 min**
- [ ] O que aprenderia diferente: **1 min**
- [ ] Perguntas: **2 min**
- [ ] **Total: 15 min**

---

## 🗂️ Arquivos para Ter Abertos

1. `animations/web/loan-contracts.html` — demo principal
2. `perguntas-e-respostas.md` — referência para Q&A
3. `advanced-topics.md` — se entrarem em lazy ou variantes
