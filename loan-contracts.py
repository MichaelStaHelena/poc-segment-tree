# loan-contracts.py
#
# Segment Tree onde cada no guarda um CONTRATO (objeto com varios atributos),
# nao um escalar. A mesma estrutura responde varias perguntas diferentes
# trocando so o merge e o elemento neutro - exatamente como em simple-operations.py.
#
# Demonstra os 5 casos com sentido semantico sobre uma carteira de 8 contratos:
#
#   Use case 1 - MAIS URGENTE        (min por dias)   -> contrato a cobrar primeiro
#   Use case 2 - MAIS FOLGADO        (max por dias)   -> contrato com mais prazo
#   Use case 3 - MENOR SALDO         (min por valor)  -> menor divida individual
#   Use case 4 - MAIOR EXPOSICAO     (max por valor)  -> maior divida individual
#   Use case 5 - EXPOSICAO TOTAL     (sum por valor)  -> R$ total da carteira
#
# Por que NAO existe "sum por dias"?
#   Vencimentos rodam em paralelo, nao em serie. Se A vence em 7 dias e B em 30,
#   A e pago no dia 7 DENTRO do prazo do B - voce nao espera 37 dias por nada.
#   Somar prazos individuais nao gera nada com unidade real, entao essa
#   combinacao foi excluida de proposito. Ver operations.md para detalhes.


# ==========================================================
# MODELO DO DADO: ContratoEmprestimo
# ==========================================================

class ContratoEmprestimo:
    def __init__(self, contrato_id, devedor, valor, dias_para_pagar):
        self.contrato_id     = contrato_id
        self.devedor         = devedor
        self.valor           = valor            # saldo devedor em aberto (R$)
        self.dias_para_pagar = dias_para_pagar  # dias restantes ate o vencimento

    def __repr__(self):
        return (f"Contrato #{self.contrato_id} | {self.devedor:<10} | "
                f"R$ {self.valor:>10,.2f} | {self.dias_para_pagar} dias restantes")


# carteira inicial - usada como ponto de partida em todos os 5 casos
INITIAL_CONTRATOS = [
    ContratoEmprestimo(1, "Alice",  5_000.00, 30),
    ContratoEmprestimo(2, "Bob",   12_000.00,  7),
    ContratoEmprestimo(3, "Carol",  3_200.00, 45),
    ContratoEmprestimo(4, "David",  8_500.00, 14),
    ContratoEmprestimo(5, "Eve",    2_100.00,  2),
    ContratoEmprestimo(6, "Frank",  9_900.00, 21),
    ContratoEmprestimo(7, "Grace",  6_700.00, 60),
    ContratoEmprestimo(8, "Hank",   4_400.00,  9),
]
C = len(INITIAL_CONTRATOS)
INF = float("inf")


def imprime_carteira(carteira, titulo="Carteira"):
    print(titulo + ":")
    for c in carteira:
        print(" ", c)
    print()


# ==========================================================
# USE CASE 1: contrato MAIS URGENTE (min por dias)
# ==========================================================
#
# Pergunta de negocio: "Qual contrato cobrar primeiro hoje no intervalo [l, r]?"
# Merge: o contrato com MENOS dias_para_pagar vence.
# Neutro: contrato sentinela com dias_para_pagar = +inf (nunca ganha o min).

contratos_urg = list(INITIAL_CONTRATOS)
NEUTRO_URG = ContratoEmprestimo(0, "-", 0.0, INF)
tree_urg = [NEUTRO_URG] * (4 * C)


def build_urg(no, inicio, fim):
    if inicio == fim:
        tree_urg[no] = contratos_urg[inicio]
        return
    meio = (inicio + fim) // 2
    build_urg(2 * no, inicio, meio)
    build_urg(2 * no + 1, meio + 1, fim)
    e, d = tree_urg[2 * no], tree_urg[2 * no + 1]
    # *** merge: menos dias_para_pagar vence ***
    tree_urg[no] = e if e.dias_para_pagar <= d.dias_para_pagar else d


def consulta_urg(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return NEUTRO_URG
    if l <= inicio and fim <= r:
        return tree_urg[no]
    meio = (inicio + fim) // 2
    e = consulta_urg(2 * no, inicio, meio, l, r)
    d = consulta_urg(2 * no + 1, meio + 1, fim, l, r)
    return e if e.dias_para_pagar <= d.dias_para_pagar else d


def atualiza_urg(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_urg[no] = novo
        contratos_urg[inicio] = novo
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_urg(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_urg(2 * no + 1, meio + 1, fim, posicao, novo)
    e, d = tree_urg[2 * no], tree_urg[2 * no + 1]
    tree_urg[no] = e if e.dias_para_pagar <= d.dias_para_pagar else d


print("==============================================")
print("Use case 1: contrato MAIS URGENTE (min por dias)")
print("==============================================")
imprime_carteira(contratos_urg)

build_urg(1, 0, C - 1)
print("Mais urgente da carteira toda:", tree_urg[1])
print()

print("Caso 1 - mais urgente entre contratos 1 e 4 (indices 0..3)")
r = consulta_urg(1, 0, C - 1, 0, 3)
print("  Segment tree responde:", r)
print("  Conferindo na mao:    ", min(contratos_urg[0:4], key=lambda x: x.dias_para_pagar))
print()

print("Caso 2 - mais urgente entre contratos 5 e 8 (indices 4..7)")
r = consulta_urg(1, 0, C - 1, 4, 7)
print("  Segment tree responde:", r)
print("  Conferindo na mao:    ", min(contratos_urg[4:8], key=lambda x: x.dias_para_pagar))
print()

print("Caso 3 - Eve renegociou: novo prazo = 90 dias")
atualiza_urg(1, 0, C - 1, 4, ContratoEmprestimo(5, "Eve", 1_500.00, 90))
r = consulta_urg(1, 0, C - 1, 0, 7)
print("  Mais urgente agora:   ", r)
print("  Conferindo na mao:    ", min(contratos_urg, key=lambda x: x.dias_para_pagar))


# ==========================================================
# USE CASE 2: contrato MAIS FOLGADO (max por dias)
# ==========================================================
#
# Pergunta de negocio: "Qual contrato no intervalo tem mais prazo - pode esperar?"
# Merge: o contrato com MAIS dias_para_pagar vence.
# Neutro: contrato sentinela com dias_para_pagar = -inf (nunca ganha o max).

contratos_folga = list(INITIAL_CONTRATOS)
NEUTRO_FOLGA = ContratoEmprestimo(0, "-", 0.0, -INF)
tree_folga = [NEUTRO_FOLGA] * (4 * C)


def build_folga(no, inicio, fim):
    if inicio == fim:
        tree_folga[no] = contratos_folga[inicio]
        return
    meio = (inicio + fim) // 2
    build_folga(2 * no, inicio, meio)
    build_folga(2 * no + 1, meio + 1, fim)
    e, d = tree_folga[2 * no], tree_folga[2 * no + 1]
    # *** merge: mais dias_para_pagar vence ***
    tree_folga[no] = e if e.dias_para_pagar >= d.dias_para_pagar else d


def consulta_folga(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return NEUTRO_FOLGA
    if l <= inicio and fim <= r:
        return tree_folga[no]
    meio = (inicio + fim) // 2
    e = consulta_folga(2 * no, inicio, meio, l, r)
    d = consulta_folga(2 * no + 1, meio + 1, fim, l, r)
    return e if e.dias_para_pagar >= d.dias_para_pagar else d


def atualiza_folga(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_folga[no] = novo
        contratos_folga[inicio] = novo
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_folga(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_folga(2 * no + 1, meio + 1, fim, posicao, novo)
    e, d = tree_folga[2 * no], tree_folga[2 * no + 1]
    tree_folga[no] = e if e.dias_para_pagar >= d.dias_para_pagar else d


print()
print()
print("==============================================")
print("Use case 2: contrato MAIS FOLGADO (max por dias)")
print("==============================================")
imprime_carteira(contratos_folga)

build_folga(1, 0, C - 1)
print("Mais folgado da carteira toda:", tree_folga[1])
print()

print("Caso 1 - mais folgado entre contratos 1 e 4 (indices 0..3)")
r = consulta_folga(1, 0, C - 1, 0, 3)
print("  Segment tree responde:", r)
print("  Conferindo na mao:    ", max(contratos_folga[0:4], key=lambda x: x.dias_para_pagar))


# ==========================================================
# USE CASE 3: MENOR SALDO devedor (min por valor)
# ==========================================================
#
# Pergunta de negocio: "Qual contrato no intervalo tem o MENOR saldo em aberto?"
# Merge: o contrato com MENOS valor vence.
# Neutro: contrato sentinela com valor = +inf.

contratos_menor = list(INITIAL_CONTRATOS)
NEUTRO_MENOR = ContratoEmprestimo(0, "-", INF, 0)
tree_menor = [NEUTRO_MENOR] * (4 * C)


def build_menor(no, inicio, fim):
    if inicio == fim:
        tree_menor[no] = contratos_menor[inicio]
        return
    meio = (inicio + fim) // 2
    build_menor(2 * no, inicio, meio)
    build_menor(2 * no + 1, meio + 1, fim)
    e, d = tree_menor[2 * no], tree_menor[2 * no + 1]
    # *** merge: menos valor vence ***
    tree_menor[no] = e if e.valor <= d.valor else d


def consulta_menor(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return NEUTRO_MENOR
    if l <= inicio and fim <= r:
        return tree_menor[no]
    meio = (inicio + fim) // 2
    e = consulta_menor(2 * no, inicio, meio, l, r)
    d = consulta_menor(2 * no + 1, meio + 1, fim, l, r)
    return e if e.valor <= d.valor else d


def atualiza_menor(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_menor[no] = novo
        contratos_menor[inicio] = novo
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_menor(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_menor(2 * no + 1, meio + 1, fim, posicao, novo)
    e, d = tree_menor[2 * no], tree_menor[2 * no + 1]
    tree_menor[no] = e if e.valor <= d.valor else d


print()
print()
print("==============================================")
print("Use case 3: MENOR SALDO devedor (min por valor)")
print("==============================================")
imprime_carteira(contratos_menor)

build_menor(1, 0, C - 1)
print("Menor saldo da carteira toda:", tree_menor[1])
print()

print("Caso 1 - menor saldo entre contratos 1 e 4 (indices 0..3)")
r = consulta_menor(1, 0, C - 1, 0, 3)
print("  Segment tree responde:", r)
print("  Conferindo na mao:    ", min(contratos_menor[0:4], key=lambda x: x.valor))


# ==========================================================
# USE CASE 4: MAIOR EXPOSICAO individual (max por valor)
# ==========================================================
#
# Pergunta de negocio: "Qual contrato no intervalo representa a MAIOR divida?"
# Merge: o contrato com MAIS valor vence.
# Neutro: contrato sentinela com valor = -inf.

contratos_maior = list(INITIAL_CONTRATOS)
NEUTRO_MAIOR = ContratoEmprestimo(0, "-", -INF, 0)
tree_maior = [NEUTRO_MAIOR] * (4 * C)


def build_maior(no, inicio, fim):
    if inicio == fim:
        tree_maior[no] = contratos_maior[inicio]
        return
    meio = (inicio + fim) // 2
    build_maior(2 * no, inicio, meio)
    build_maior(2 * no + 1, meio + 1, fim)
    e, d = tree_maior[2 * no], tree_maior[2 * no + 1]
    # *** merge: mais valor vence ***
    tree_maior[no] = e if e.valor >= d.valor else d


def consulta_maior(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return NEUTRO_MAIOR
    if l <= inicio and fim <= r:
        return tree_maior[no]
    meio = (inicio + fim) // 2
    e = consulta_maior(2 * no, inicio, meio, l, r)
    d = consulta_maior(2 * no + 1, meio + 1, fim, l, r)
    return e if e.valor >= d.valor else d


def atualiza_maior(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_maior[no] = novo
        contratos_maior[inicio] = novo
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_maior(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_maior(2 * no + 1, meio + 1, fim, posicao, novo)
    e, d = tree_maior[2 * no], tree_maior[2 * no + 1]
    tree_maior[no] = e if e.valor >= d.valor else d


print()
print()
print("==============================================")
print("Use case 4: MAIOR EXPOSICAO individual (max por valor)")
print("==============================================")
imprime_carteira(contratos_maior)

build_maior(1, 0, C - 1)
print("Maior exposicao da carteira toda:", tree_maior[1])
print()

print("Caso 1 - maior exposicao entre contratos 5 e 8 (indices 4..7)")
r = consulta_maior(1, 0, C - 1, 4, 7)
print("  Segment tree responde:", r)
print("  Conferindo na mao:    ", max(contratos_maior[4:8], key=lambda x: x.valor))


# ==========================================================
# USE CASE 5: EXPOSICAO TOTAL (sum por valor)
# ==========================================================
#
# Pergunta de negocio: "Quanto a carteira [l, r] deve no TOTAL?"
# Merge: soma dos valores.
# Neutro: 0.0 (elemento neutro da soma).
#
# Aqui o tree NAO guarda contratos - guarda escalares (R$ acumulado), porque
# o agregado da soma e um numero, nao um contrato. As folhas extraem so o
# campo `valor` do contrato; os nos internos somam.

contratos_soma = list(INITIAL_CONTRATOS)
tree_soma = [0.0] * (4 * C)


def build_soma(no, inicio, fim):
    if inicio == fim:
        # *** folha guarda so o campo `valor` do contrato ***
        tree_soma[no] = contratos_soma[inicio].valor
        return
    meio = (inicio + fim) // 2
    build_soma(2 * no, inicio, meio)
    build_soma(2 * no + 1, meio + 1, fim)
    # *** merge: soma os dois filhos ***
    tree_soma[no] = tree_soma[2 * no] + tree_soma[2 * no + 1]


def consulta_soma(no, inicio, fim, l, r):
    if r < inicio or fim < l:
        return 0.0
    if l <= inicio and fim <= r:
        return tree_soma[no]
    meio = (inicio + fim) // 2
    return (consulta_soma(2 * no, inicio, meio, l, r)
          + consulta_soma(2 * no + 1, meio + 1, fim, l, r))


def atualiza_soma(no, inicio, fim, posicao, novo):
    if inicio == fim:
        tree_soma[no] = novo.valor
        contratos_soma[inicio] = novo
        return
    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_soma(2 * no, inicio, meio, posicao, novo)
    else:
        atualiza_soma(2 * no + 1, meio + 1, fim, posicao, novo)
    tree_soma[no] = tree_soma[2 * no] + tree_soma[2 * no + 1]


print()
print()
print("==============================================")
print("Use case 5: EXPOSICAO TOTAL (sum por valor)")
print("==============================================")
imprime_carteira(contratos_soma)

build_soma(1, 0, C - 1)
print(f"Exposicao total da carteira: R$ {tree_soma[1]:,.2f}")
print()

print("Caso 1 - exposicao total entre contratos 1 e 4 (indices 0..3)")
total = consulta_soma(1, 0, C - 1, 0, 3)
manual = sum(c.valor for c in contratos_soma[0:4])
print(f"  Segment tree responde: R$ {total:,.2f}")
print(f"  Conferindo na mao:     R$ {manual:,.2f}")
print()

print("Caso 2 - Eve quitou parcialmente, novo saldo = R$ 500,00")
atualiza_soma(1, 0, C - 1, 4, ContratoEmprestimo(5, "Eve", 500.00, 2))
print(f"  Exposicao total agora: R$ {tree_soma[1]:,.2f}")
print(f"  Conferindo na mao:     R$ {sum(c.valor for c in contratos_soma):,.2f}")
