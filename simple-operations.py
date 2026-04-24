# simple-operations.py
#
# Demonstracao das operacoes basicas de uma Segment Tree.
# Tres casos de uso pra mostrar que a mesma estrutura resolve
# problemas bem diferentes trocando so o "merge" e o elemento neutro.
#
#   Use case 1 - EDUCACAO: maior nota entre dois alunos         (MAX)
#   Use case 2 - FINANCAS: soma de vendas num intervalo          (SOMA)
#   Use case 3 - SAUDE:    menor saturacao de oxigenio no turno  (MIN)


# ==========================================================
# USE CASE 1: NOTAS DA TURMA (operacao de MAXIMO)
# ==========================================================
#
# Cenario: uma turma de 8 alunos. A professora quer responder:
#   1) Qual a MAIOR nota entre os alunos X e Y?
#   2) Um aluno refez a prova, como atualizar a resposta?

# notas de cada um dos 8 alunos (indices 0 a 7)
notas = [7, 3, 9, 5, 8, 2, 6, 4]
M = len(notas)

# a arvore fica num vetor de tamanho 4*M (tamanho seguro que sempre cabe).
# comeca com -1 porque nenhuma nota vale menos que isso - serve como
# "ainda nao tem nada aqui" e nao atrapalha as comparacoes de max.
tree_notas = [-1] * (4 * M)


# ----------------------------------------------------------
# build_max - monta a arvore a partir do array de notas
# ----------------------------------------------------------
# chamada inicial:  build_max(1, 0, M - 1)
#   no     = indice do nó atual dentro de tree_notas[] (comeca em 1 = raiz)
#   inicio = primeiro indice do array que esse nó cobre
#   fim    = ultimo indice do array que esse nó cobre
def build_max(no, inicio, fim):

    # se o intervalo tem um unico elemento, esse nó e uma FOLHA.
    # folhas guardam o valor direto do array.
    if inicio == fim:
        tree_notas[no] = notas[inicio]
        return

    # se tem mais de um elemento, divide em duas metades
    meio = (inicio + fim) // 2

    # monta o filho esquerdo (cobre [inicio, meio]) na posicao 2*no
    build_max(2 * no, inicio, meio)

    # monta o filho direito (cobre [meio+1, fim]) na posicao 2*no + 1
    build_max(2 * no + 1, meio + 1, fim)

    # depois que os filhos estao prontos, esse nó guarda o MAIOR deles.
    # ou seja: cada nó interno = max do pedaco do array que ele cobre.
    tree_notas[no] = max(tree_notas[2 * no], tree_notas[2 * no + 1])


# ----------------------------------------------------------
# consulta_max - retorna a maior nota em notas[l..r]
# ----------------------------------------------------------
def consulta_max(no, inicio, fim, l, r):

    # CASO 1: o intervalo desse nó esta totalmente FORA do pedido.
    # retorna -1 (neutro: nao altera o max com notas >= 0).
    if r < inicio or fim < l:
        return -1

    # CASO 2: o intervalo desse nó esta totalmente DENTRO do pedido.
    # a resposta ja esta pronta em tree_notas[no] - nem precisa descer!
    # esse e o truque que faz a consulta ser O(log n) em vez de O(n).
    if l <= inicio and fim <= r:
        return tree_notas[no]

    # CASO 3: sobreposicao parcial - desce nos dois filhos e combina
    meio = (inicio + fim) // 2
    maior_esq = consulta_max(2 * no, inicio, meio, l, r)
    maior_dir = consulta_max(2 * no + 1, meio + 1, fim, l, r)
    return max(maior_esq, maior_dir)


# ----------------------------------------------------------
# atualiza_max - muda a nota de um aluno
# ----------------------------------------------------------
def atualiza_max(no, inicio, fim, posicao, novo_valor):

    # chegamos na folha da posicao que queremos mudar? atualiza e volta.
    if inicio == fim:
        tree_notas[no] = novo_valor
        notas[inicio] = novo_valor   # mantem o array original sincronizado
        return

    meio = (inicio + fim) // 2

    # desce SO pelo lado que contem a posicao (o outro nao mudou).
    # por isso a atualizacao e O(log n): so o caminho folha->raiz e visitado.
    if posicao <= meio:
        atualiza_max(2 * no, inicio, meio, posicao, novo_valor)
    else:
        atualiza_max(2 * no + 1, meio + 1, fim, posicao, novo_valor)

    # ao voltar da recursao, um dos filhos mudou.
    # entao recalcula esse nó pegando o max dos dois filhos.
    tree_notas[no] = max(tree_notas[2 * no], tree_notas[2 * no + 1])


# ----------------------------------------------------------
# imprime_arvore - desenha a arvore em niveis (so pro caso 1)
# ----------------------------------------------------------
# Mostra a arvore por linhas, do topo pras folhas, com cada valor
# centralizado. Serve pra enxergar que cada nivel resume o array
# em blocos maiores - a raiz resume tudo, as folhas sao o array.
# (Assume M potencia de 2 - caso nosso com M = 8.)
def imprime_arvore():
    niveis = M.bit_length()      # pra M = 8 da 4 niveis (0 a 3)
    largura = M * 3              # cada folha ocupa ~3 chars no rodape

    print("Arvore por niveis (cada no = max do pedaco que cobre):")
    for nivel in range(niveis):
        nos = 2 ** nivel                    # 1, 2, 4, 8 nos por nivel
        slot = largura // nos               # largura de cada no na linha
        linha = "  "
        for i in range(nos):
            valor = tree_notas[nos + i]     # indices: 1 / 2,3 / 4..7 / 8..15
            linha += str(valor).center(slot)
        print(linha)
    print()


# ----------------------------------------------------------
# Demonstracao: casos de uso das notas
# ----------------------------------------------------------

print("==============================================")
print("Use case 1: notas da turma (MAX)")
print("==============================================")
print("Notas dos 8 alunos:", notas)

build_max(1, 0, M - 1)
print("Arvore montada. Maior nota da turma =", tree_notas[1])
print()
imprime_arvore()


# Caso 1: maior nota entre os alunos 3 e 6
# (aluno 3 = indice 2, aluno 6 = indice 5)
print("Caso 1 - maior nota entre os alunos 3 e 6")
n1 = consulta_max(1, 0, M - 1, 2, 5)
print("  Segment tree responde:", n1)
print("  Conferindo na mao:    max de", notas[2:6], "=", max(notas[2:6]))
print()


# Caso 2: aluno 5 refez a prova e tirou 10
print("Caso 2 - aluno 5 refez a prova: nova nota = 10")
atualiza_max(1, 0, M - 1, 4, 10)   # posicao 4 = aluno 5
print("  Notas apos atualizacao:", notas)
print("  Nova maior nota da turma:", tree_notas[1])
print()


# Caso 3: mesma pergunta do caso 1, com a nota nova
print("Caso 3 - maior nota entre os alunos 3 e 6 (agora)")
n3 = consulta_max(1, 0, M - 1, 2, 5)
print("  Segment tree responde:", n3)
print("  Conferindo na mao:    max de", notas[2:6], "=", max(notas[2:6]))
print()


# Caso 4: maior nota entre os alunos 6 e 8
print("Caso 4 - maior nota entre os alunos 6 e 8")
n4 = consulta_max(1, 0, M - 1, 5, 7)
print("  Segment tree responde:", n4)
print("  Conferindo na mao:    max de", notas[5:8], "=", max(notas[5:8]))


# ==========================================================
# USE CASE 2: VENDAS DIARIAS (operacao de SOMA)
# ==========================================================
#
# Mesmo raciocinio, outro problema:
#   1) Qual o total de vendas entre o dia X e o dia Y?
#   2) Uma venda foi corrigida, como atualizar a resposta?
#
# A estrutura da arvore e IGUAL a do use case 1. So mudam duas coisas:
#   - o merge: em vez de max dos filhos, a gente SOMA os filhos
#   - o elemento neutro: em vez de -1, usa 0 (neutro da soma)
#
# Essa e a maior sacada da segment tree: o "esqueleto" e sempre o
# mesmo. Trocando o merge e o neutro resolve min, max, soma, gcd, xor, etc.

# vendas de cada um dos 8 dias
vendas = [4, 1, 3, 5, 2, 6, 1, 2]
N = len(vendas)

# tree de soma comeca com 0 (neutro da soma)
tree = [0] * (4 * N)


# ----------------------------------------------------------
# build - monta a arvore de soma
# ----------------------------------------------------------
def build(no, inicio, fim):

    if inicio == fim:
        tree[no] = vendas[inicio]
        return

    meio = (inicio + fim) // 2
    build(2 * no, inicio, meio)
    build(2 * no + 1, meio + 1, fim)

    # *** UNICA DIFERENCA pro build_max ***
    # em vez de max, soma os dois filhos
    tree[no] = tree[2 * no] + tree[2 * no + 1]


# ----------------------------------------------------------
# consulta - soma no intervalo [l, r]
# ----------------------------------------------------------
def consulta(no, inicio, fim, l, r):

    # CASO 1: fora do pedido. retorna 0 (neutro da soma).
    if r < inicio or fim < l:
        return 0

    # CASO 2: totalmente dentro - resposta pronta
    if l <= inicio and fim <= r:
        return tree[no]

    # CASO 3: parcial - desce e combina
    meio = (inicio + fim) // 2
    soma_esq = consulta(2 * no, inicio, meio, l, r)
    soma_dir = consulta(2 * no + 1, meio + 1, fim, l, r)

    # *** UNICA DIFERENCA pra consulta_max ***
    # soma em vez de max
    return soma_esq + soma_dir


# ----------------------------------------------------------
# atualiza - muda o valor de uma posicao
# ----------------------------------------------------------
def atualiza(no, inicio, fim, posicao, novo_valor):

    if inicio == fim:
        tree[no] = novo_valor
        vendas[inicio] = novo_valor
        return

    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza(2 * no, inicio, meio, posicao, novo_valor)
    else:
        atualiza(2 * no + 1, meio + 1, fim, posicao, novo_valor)

    # *** UNICA DIFERENCA pro atualiza_max ***
    # soma em vez de max
    tree[no] = tree[2 * no] + tree[2 * no + 1]


# ----------------------------------------------------------
# Demonstracao: casos de uso das vendas
# ----------------------------------------------------------

print()
print()
print("==============================================")
print("Use case 2: vendas diarias (SOMA)")
print("==============================================")
print("Vendas dos 8 dias:", vendas)

build(1, 0, N - 1)
print("Arvore montada. Soma total =", tree[1])
print()


# Caso 1: soma das vendas do dia 3 ao dia 6
print("Caso 1 - soma das vendas do dia 3 ao dia 6")
v1 = consulta(1, 0, N - 1, 2, 5)
print("  Segment tree responde:", v1)
print("  Conferindo na mao:    ", vendas[2], "+", vendas[3], "+", vendas[4], "+", vendas[5], "=", sum(vendas[2:6]))
print()


# Caso 2: corrigindo o dia 4 de 5 para 7
print("Caso 2 - corrigindo o dia 4: de 5 para 7")
atualiza(1, 0, N - 1, 3, 7)
print("  Vendas apos correcao:", vendas)
print("  Nova soma total:     ", tree[1])
print()


# Caso 3: mesma pergunta com o valor corrigido
print("Caso 3 - soma do dia 3 ao dia 6 (depois da correcao)")
v3 = consulta(1, 0, N - 1, 2, 5)
print("  Segment tree responde:", v3)
print("  Conferindo na mao:    ", sum(vendas[2:6]))


# ==========================================================
# USE CASE 3: SATURACAO DE OXIGENIO DO PACIENTE (operacao de MINIMO)
# ==========================================================
#
# Um paciente internado teve a saturacao de oxigenio (SpO2) medida
# de 2 em 2 horas. Saturacao BAIXA e perigosa - por isso o sistema
# precisa responder:
#   1) Qual a MENOR saturacao num intervalo de horas?
#   2) Uma medicao foi corrigida, como atualizar?
#
# Agora a operacao e MIN - a primeira que NAO e max nem soma.
# A estrutura da arvore continua exatamente a mesma. Mudam duas
# coisas, como sempre:
#   - o merge: em vez de max (ou soma), MIN dos filhos
#   - o elemento neutro: em vez de -1 ou 0, usa "infinito"
#     (neutro do min - ninguem e maior que ele, entao nao atrapalha)

# saturacoes a cada 2 horas (06h, 08h, 10h, 12h, 14h, 16h, 18h, 20h)
spo2 = [98, 97, 92, 94, 96, 99, 95, 93]
S = len(spo2)

# INF = infinito. E o neutro do min: min(qualquer coisa, INF) = qualquer coisa.
INF = float("inf")
tree_spo2 = [INF] * (4 * S)


# ----------------------------------------------------------
# build_min - monta a arvore de minimo
# ----------------------------------------------------------
def build_min(no, inicio, fim):

    if inicio == fim:
        tree_spo2[no] = spo2[inicio]
        return

    meio = (inicio + fim) // 2
    build_min(2 * no, inicio, meio)
    build_min(2 * no + 1, meio + 1, fim)

    # *** UNICA DIFERENCA pro build_max / build ***
    # min em vez de max (ou soma)
    tree_spo2[no] = min(tree_spo2[2 * no], tree_spo2[2 * no + 1])


# ----------------------------------------------------------
# consulta_min - menor valor em spo2[l..r]
# ----------------------------------------------------------
def consulta_min(no, inicio, fim, l, r):

    # CASO 1: fora do pedido. retorna INF (neutro do min).
    if r < inicio or fim < l:
        return INF

    # CASO 2: totalmente dentro - resposta pronta
    if l <= inicio and fim <= r:
        return tree_spo2[no]

    # CASO 3: parcial - desce e combina
    meio = (inicio + fim) // 2
    menor_esq = consulta_min(2 * no, inicio, meio, l, r)
    menor_dir = consulta_min(2 * no + 1, meio + 1, fim, l, r)

    # *** UNICA DIFERENCA pra consulta_max / consulta ***
    return min(menor_esq, menor_dir)


# ----------------------------------------------------------
# atualiza_min - muda uma medicao
# ----------------------------------------------------------
def atualiza_min(no, inicio, fim, posicao, novo_valor):

    if inicio == fim:
        tree_spo2[no] = novo_valor
        spo2[inicio] = novo_valor
        return

    meio = (inicio + fim) // 2
    if posicao <= meio:
        atualiza_min(2 * no, inicio, meio, posicao, novo_valor)
    else:
        atualiza_min(2 * no + 1, meio + 1, fim, posicao, novo_valor)

    # *** UNICA DIFERENCA pro atualiza_max / atualiza ***
    tree_spo2[no] = min(tree_spo2[2 * no], tree_spo2[2 * no + 1])


# ----------------------------------------------------------
# Demonstracao: casos de uso da saturacao
# ----------------------------------------------------------

print()
print()
print("==============================================")
print("Use case 3: saturacao de oxigenio (MIN)")
print("==============================================")
print("Medicoes de SpO2 % (06h, 08h, 10h, 12h, 14h, 16h, 18h, 20h):")
print(" ", spo2)

build_min(1, 0, S - 1)
print("Arvore montada. Menor saturacao do dia =", tree_spo2[1], "%")
print()


# Caso 1: menor saturacao entre 10h e 16h (indices 2 a 5)
print("Caso 1 - menor saturacao entre 10h e 16h")
s1 = consulta_min(1, 0, S - 1, 2, 5)
print("  Segment tree responde:", s1)
print("  Conferindo na mao:    min de", spo2[2:6], "=", min(spo2[2:6]))
print()


# Caso 2: medicao das 10h estava errada, o valor certo era 90
print("Caso 2 - corrigindo medicao das 10h: de 92 para 90")
atualiza_min(1, 0, S - 1, 2, 90)
print("  SpO2 apos correcao:          ", spo2)
print("  Menor saturacao do dia agora:", tree_spo2[1])
print()


# Caso 3: repete a pergunta com o valor corrigido
print("Caso 3 - menor saturacao entre 10h e 16h (depois da correcao)")
s3 = consulta_min(1, 0, S - 1, 2, 5)
print("  Segment tree responde:", s3)
print("  Conferindo na mao:    min de", spo2[2:6], "=", min(spo2[2:6]))
