// simple-operations.rs
//
// Demonstracao das operacoes basicas de uma Segment Tree.
// Tres casos de uso pra mostrar que a mesma estrutura resolve
// problemas bem diferentes trocando so o "merge" e o elemento neutro.
//
//   Use case 1 - EDUCACAO: maior nota entre dois alunos         (MAX)
//   Use case 2 - FINANCAS: soma de vendas num intervalo          (SOMA)
//   Use case 3 - SAUDE:    menor saturacao de oxigenio no turno  (MIN)
//
// Como rodar:
//   rustc simple-operations.rs && ./simple-operations
//
// Como rodar os testes:
//   rustc --test simple-operations.rs -o simple-operations-test && ./simple-operations-test


use std::cmp::{max, min};


// ==========================================================
// USE CASE 1: NOTAS DA TURMA (operacao de MAXIMO)
// ==========================================================
//
// Cenario: uma turma de 8 alunos. A professora quer responder:
//   1) Qual a MAIOR nota entre os alunos X e Y?
//   2) Um aluno refez a prova, como atualizar a resposta?


// ----------------------------------------------------------
// build_max - monta a arvore a partir do array de notas
// ----------------------------------------------------------
// chamada inicial:  build_max(&notas, &mut tree_notas, 1, 0, M - 1)
//   no     = indice do nó atual dentro de tree_notas[] (comeca em 1 = raiz)
//   inicio = primeiro indice do array que esse nó cobre
//   fim    = ultimo indice do array que esse nó cobre
fn build_max(notas: &[i32], tree_notas: &mut [i32], no: usize, inicio: usize, fim: usize) {

    // se o intervalo tem um unico elemento, esse nó e uma FOLHA.
    // folhas guardam o valor direto do array.
    if inicio == fim {
        tree_notas[no] = notas[inicio];
        return;
    }

    // se tem mais de um elemento, divide em duas metades
    let meio = (inicio + fim) / 2;

    // monta o filho esquerdo (cobre [inicio, meio]) na posicao 2*no
    build_max(notas, tree_notas, 2 * no, inicio, meio);

    // monta o filho direito (cobre [meio+1, fim]) na posicao 2*no + 1
    build_max(notas, tree_notas, 2 * no + 1, meio + 1, fim);

    // depois que os filhos estao prontos, esse nó guarda o MAIOR deles.
    // ou seja: cada nó interno = max do pedaco do array que ele cobre.
    tree_notas[no] = max(tree_notas[2 * no], tree_notas[2 * no + 1]);
}


// ----------------------------------------------------------
// consulta_max - retorna a maior nota em notas[l..r]
// ----------------------------------------------------------
fn consulta_max(tree_notas: &[i32], no: usize, inicio: usize, fim: usize, l: usize, r: usize) -> i32 {

    // CASO 1: o intervalo desse nó esta totalmente FORA do pedido.
    // retorna -1 (neutro: nao altera o max com notas >= 0).
    if r < inicio || fim < l {
        return -1;
    }

    // CASO 2: o intervalo desse nó esta totalmente DENTRO do pedido.
    // a resposta ja esta pronta em tree_notas[no] - nem precisa descer!
    // esse e o truque que faz a consulta ser O(log n) em vez de O(n).
    if l <= inicio && fim <= r {
        return tree_notas[no];
    }

    // CASO 3: sobreposicao parcial - desce nos dois filhos e combina
    let meio = (inicio + fim) / 2;
    let maior_esq = consulta_max(tree_notas, 2 * no, inicio, meio, l, r);
    let maior_dir = consulta_max(tree_notas, 2 * no + 1, meio + 1, fim, l, r);
    max(maior_esq, maior_dir)
}


// ----------------------------------------------------------
// atualiza_max - muda a nota de um aluno
// ----------------------------------------------------------
fn atualiza_max(notas: &mut [i32], tree_notas: &mut [i32], no: usize, inicio: usize, fim: usize, posicao: usize, novo_valor: i32) {

    // chegamos na folha da posicao que queremos mudar? atualiza e volta.
    if inicio == fim {
        tree_notas[no] = novo_valor;
        notas[inicio] = novo_valor;   // mantem o array original sincronizado
        return;
    }

    let meio = (inicio + fim) / 2;

    // desce SO pelo lado que contem a posicao (o outro nao mudou).
    // por isso a atualizacao e O(log n): so o caminho folha->raiz e visitado.
    if posicao <= meio {
        atualiza_max(notas, tree_notas, 2 * no, inicio, meio, posicao, novo_valor);
    } else {
        atualiza_max(notas, tree_notas, 2 * no + 1, meio + 1, fim, posicao, novo_valor);
    }

    // ao voltar da recursao, um dos filhos mudou.
    // entao recalcula esse nó pegando o max dos dois filhos.
    tree_notas[no] = max(tree_notas[2 * no], tree_notas[2 * no + 1]);
}


// ----------------------------------------------------------
// imprime_arvore - desenha a arvore em niveis (so pro caso 1)
// ----------------------------------------------------------
// Mostra a arvore por linhas, do topo pras folhas, com cada valor
// centralizado. Serve pra enxergar que cada nivel resume o array
// em blocos maiores - a raiz resume tudo, as folhas sao o array.
// (Assume M potencia de 2 - caso nosso com M = 8.)
fn imprime_arvore(tree_notas: &[i32], m: usize) {
    // pra M = 8 da 4 niveis (0 a 3) - equivalente ao bit_length do Python
    let niveis = (usize::BITS - (m as usize).leading_zeros()) as usize;
    let largura = m * 3;             // cada folha ocupa ~3 chars no rodape

    println!("Arvore por niveis (cada no = max do pedaco que cobre):");
    for nivel in 0..niveis {
        let nos = 1usize << nivel;                // 1, 2, 4, 8 nos por nivel
        let slot = largura / nos;                 // largura de cada no na linha
        let mut linha = String::from("  ");
        for i in 0..nos {
            let valor = tree_notas[nos + i];      // indices: 1 / 2,3 / 4..7 / 8..15
            let texto = valor.to_string();
            // centraliza o numero dentro do slot (equivalente a str.center do Python)
            let pad = slot.saturating_sub(texto.len());
            let left = pad / 2;
            let right = pad - left;
            linha.push_str(&" ".repeat(left));
            linha.push_str(&texto);
            linha.push_str(&" ".repeat(right));
        }
        println!("{}", linha);
    }
    println!();
}


// ==========================================================
// USE CASE 2: VENDAS DIARIAS (operacao de SOMA)
// ==========================================================
//
// Mesmo raciocinio, outro problema:
//   1) Qual o total de vendas entre o dia X e o dia Y?
//   2) Uma venda foi corrigida, como atualizar a resposta?
//
// A estrutura da arvore e IGUAL a do use case 1. So mudam duas coisas:
//   - o merge: em vez de max dos filhos, a gente SOMA os filhos
//   - o elemento neutro: em vez de -1, usa 0 (neutro da soma)
//
// Essa e a maior sacada da segment tree: o "esqueleto" e sempre o
// mesmo. Trocando o merge e o neutro resolve min, max, soma, gcd, xor, etc.


// ----------------------------------------------------------
// build - monta a arvore de soma
// ----------------------------------------------------------
fn build(vendas: &[i32], tree: &mut [i32], no: usize, inicio: usize, fim: usize) {

    if inicio == fim {
        tree[no] = vendas[inicio];
        return;
    }

    let meio = (inicio + fim) / 2;
    build(vendas, tree, 2 * no, inicio, meio);
    build(vendas, tree, 2 * no + 1, meio + 1, fim);

    // *** UNICA DIFERENCA pro build_max ***
    // em vez de max, soma os dois filhos
    tree[no] = tree[2 * no] + tree[2 * no + 1];
}


// ----------------------------------------------------------
// consulta - soma no intervalo [l, r]
// ----------------------------------------------------------
fn consulta(tree: &[i32], no: usize, inicio: usize, fim: usize, l: usize, r: usize) -> i32 {

    // CASO 1: fora do pedido. retorna 0 (neutro da soma).
    if r < inicio || fim < l {
        return 0;
    }

    // CASO 2: totalmente dentro - resposta pronta
    if l <= inicio && fim <= r {
        return tree[no];
    }

    // CASO 3: parcial - desce e combina
    let meio = (inicio + fim) / 2;
    let soma_esq = consulta(tree, 2 * no, inicio, meio, l, r);
    let soma_dir = consulta(tree, 2 * no + 1, meio + 1, fim, l, r);

    // *** UNICA DIFERENCA pra consulta_max ***
    // soma em vez de max
    soma_esq + soma_dir
}


// ----------------------------------------------------------
// atualiza - muda o valor de uma posicao
// ----------------------------------------------------------
fn atualiza(vendas: &mut [i32], tree: &mut [i32], no: usize, inicio: usize, fim: usize, posicao: usize, novo_valor: i32) {

    if inicio == fim {
        tree[no] = novo_valor;
        vendas[inicio] = novo_valor;
        return;
    }

    let meio = (inicio + fim) / 2;
    if posicao <= meio {
        atualiza(vendas, tree, 2 * no, inicio, meio, posicao, novo_valor);
    } else {
        atualiza(vendas, tree, 2 * no + 1, meio + 1, fim, posicao, novo_valor);
    }

    // *** UNICA DIFERENCA pro atualiza_max ***
    // soma em vez de max
    tree[no] = tree[2 * no] + tree[2 * no + 1];
}


// ==========================================================
// USE CASE 3: SATURACAO DE OXIGENIO DO PACIENTE (operacao de MINIMO)
// ==========================================================
//
// Um paciente internado teve a saturacao de oxigenio (SpO2) medida
// de 2 em 2 horas. Saturacao BAIXA e perigosa - por isso o sistema
// precisa responder:
//   1) Qual a MENOR saturacao num intervalo de horas?
//   2) Uma medicao foi corrigida, como atualizar?
//
// Agora a operacao e MIN - a primeira que NAO e max nem soma.
// A estrutura da arvore continua exatamente a mesma. Mudam duas
// coisas, como sempre:
//   - o merge: em vez de max (ou soma), MIN dos filhos
//   - o elemento neutro: em vez de -1 ou 0, usa "infinito"
//     (neutro do min - ninguem e maior que ele, entao nao atrapalha)

// INF = infinito. E o neutro do min: min(qualquer coisa, INF) = qualquer coisa.
const INF: i32 = i32::MAX;


// ----------------------------------------------------------
// build_min - monta a arvore de minimo
// ----------------------------------------------------------
fn build_min(spo2: &[i32], tree_spo2: &mut [i32], no: usize, inicio: usize, fim: usize) {

    if inicio == fim {
        tree_spo2[no] = spo2[inicio];
        return;
    }

    let meio = (inicio + fim) / 2;
    build_min(spo2, tree_spo2, 2 * no, inicio, meio);
    build_min(spo2, tree_spo2, 2 * no + 1, meio + 1, fim);

    // *** UNICA DIFERENCA pro build_max / build ***
    // min em vez de max (ou soma)
    tree_spo2[no] = min(tree_spo2[2 * no], tree_spo2[2 * no + 1]);
}


// ----------------------------------------------------------
// consulta_min - menor valor em spo2[l..r]
// ----------------------------------------------------------
fn consulta_min(tree_spo2: &[i32], no: usize, inicio: usize, fim: usize, l: usize, r: usize) -> i32 {

    // CASO 1: fora do pedido. retorna INF (neutro do min).
    if r < inicio || fim < l {
        return INF;
    }

    // CASO 2: totalmente dentro - resposta pronta
    if l <= inicio && fim <= r {
        return tree_spo2[no];
    }

    // CASO 3: parcial - desce e combina
    let meio = (inicio + fim) / 2;
    let menor_esq = consulta_min(tree_spo2, 2 * no, inicio, meio, l, r);
    let menor_dir = consulta_min(tree_spo2, 2 * no + 1, meio + 1, fim, l, r);

    // *** UNICA DIFERENCA pra consulta_max / consulta ***
    min(menor_esq, menor_dir)
}


// ----------------------------------------------------------
// atualiza_min - muda uma medicao
// ----------------------------------------------------------
fn atualiza_min(spo2: &mut [i32], tree_spo2: &mut [i32], no: usize, inicio: usize, fim: usize, posicao: usize, novo_valor: i32) {

    if inicio == fim {
        tree_spo2[no] = novo_valor;
        spo2[inicio] = novo_valor;
        return;
    }

    let meio = (inicio + fim) / 2;
    if posicao <= meio {
        atualiza_min(spo2, tree_spo2, 2 * no, inicio, meio, posicao, novo_valor);
    } else {
        atualiza_min(spo2, tree_spo2, 2 * no + 1, meio + 1, fim, posicao, novo_valor);
    }

    // *** UNICA DIFERENCA pro atualiza_max / atualiza ***
    tree_spo2[no] = min(tree_spo2[2 * no], tree_spo2[2 * no + 1]);
}


// ==========================================================
// main - executa as demonstracoes dos tres casos de uso
// ==========================================================
fn main() {

    // ------------------------------------------------------
    // Demonstracao: casos de uso das notas
    // ------------------------------------------------------

    // notas de cada um dos 8 alunos (indices 0 a 7)
    let mut notas: Vec<i32> = vec![7, 3, 9, 5, 8, 2, 6, 4];
    let m = notas.len();

    // a arvore fica num vetor de tamanho 4*M (tamanho seguro que sempre cabe).
    // comeca com -1 porque nenhuma nota vale menos que isso - serve como
    // "ainda nao tem nada aqui" e nao atrapalha as comparacoes de max.
    let mut tree_notas: Vec<i32> = vec![-1; 4 * m];

    println!("==============================================");
    println!("Use case 1: notas da turma (MAX)");
    println!("==============================================");
    println!("Notas dos 8 alunos: {:?}", notas);

    build_max(&notas, &mut tree_notas, 1, 0, m - 1);
    println!("Arvore montada. Maior nota da turma = {}", tree_notas[1]);
    println!();
    imprime_arvore(&tree_notas, m);


    // Caso 1: maior nota entre os alunos 3 e 6
    // (aluno 3 = indice 2, aluno 6 = indice 5)
    println!("Caso 1 - maior nota entre os alunos 3 e 6");
    let n1 = consulta_max(&tree_notas, 1, 0, m - 1, 2, 5);
    println!("  Segment tree responde: {}", n1);
    let fatia = &notas[2..6];
    println!("  Conferindo na mao:    max de {:?} = {}", fatia, fatia.iter().max().unwrap());
    println!();


    // Caso 2: aluno 5 refez a prova e tirou 10
    println!("Caso 2 - aluno 5 refez a prova: nova nota = 10");
    atualiza_max(&mut notas, &mut tree_notas, 1, 0, m - 1, 4, 10);   // posicao 4 = aluno 5
    println!("  Notas apos atualizacao: {:?}", notas);
    println!("  Nova maior nota da turma: {}", tree_notas[1]);
    println!();


    // Caso 3: mesma pergunta do caso 1, com a nota nova
    println!("Caso 3 - maior nota entre os alunos 3 e 6 (agora)");
    let n3 = consulta_max(&tree_notas, 1, 0, m - 1, 2, 5);
    println!("  Segment tree responde: {}", n3);
    let fatia = &notas[2..6];
    println!("  Conferindo na mao:    max de {:?} = {}", fatia, fatia.iter().max().unwrap());
    println!();


    // Caso 4: maior nota entre os alunos 6 e 8
    println!("Caso 4 - maior nota entre os alunos 6 e 8");
    let n4 = consulta_max(&tree_notas, 1, 0, m - 1, 5, 7);
    println!("  Segment tree responde: {}", n4);
    let fatia = &notas[5..8];
    println!("  Conferindo na mao:    max de {:?} = {}", fatia, fatia.iter().max().unwrap());



    // ------------------------------------------------------
    // Demonstracao: casos de uso das vendas
    // ------------------------------------------------------

    // vendas de cada um dos 8 dias
    let mut vendas: Vec<i32> = vec![4, 1, 3, 5, 2, 6, 1, 2];
    let n = vendas.len();

    // tree de soma comeca com 0 (neutro da soma)
    let mut tree: Vec<i32> = vec![0; 4 * n];

    println!();
    println!();
    println!("==============================================");
    println!("Use case 2: vendas diarias (SOMA)");
    println!("==============================================");
    println!("Vendas dos 8 dias: {:?}", vendas);

    build(&vendas, &mut tree, 1, 0, n - 1);
    println!("Arvore montada. Soma total = {}", tree[1]);
    println!();


    // Caso 1: soma das vendas do dia 3 ao dia 6
    println!("Caso 1 - soma das vendas do dia 3 ao dia 6");
    let v1 = consulta(&tree, 1, 0, n - 1, 2, 5);
    println!("  Segment tree responde: {}", v1);
    let soma_manual: i32 = vendas[2..6].iter().sum();
    println!("  Conferindo na mao:     {} + {} + {} + {} = {}",
        vendas[2], vendas[3], vendas[4], vendas[5], soma_manual);
    println!();


    // Caso 2: corrigindo o dia 4 de 5 para 7
    println!("Caso 2 - corrigindo o dia 4: de 5 para 7");
    atualiza(&mut vendas, &mut tree, 1, 0, n - 1, 3, 7);
    println!("  Vendas apos correcao: {:?}", vendas);
    println!("  Nova soma total:      {}", tree[1]);
    println!();


    // Caso 3: mesma pergunta com o valor corrigido
    println!("Caso 3 - soma do dia 3 ao dia 6 (depois da correcao)");
    let v3 = consulta(&tree, 1, 0, n - 1, 2, 5);
    println!("  Segment tree responde: {}", v3);
    let soma_manual: i32 = vendas[2..6].iter().sum();
    println!("  Conferindo na mao:     {}", soma_manual);


    // ------------------------------------------------------
    // Demonstracao: casos de uso da saturacao
    // ------------------------------------------------------

    // saturacoes a cada 2 horas (06h, 08h, 10h, 12h, 14h, 16h, 18h, 20h)
    let mut spo2: Vec<i32> = vec![98, 97, 92, 94, 96, 99, 95, 93];
    let s = spo2.len();

    let mut tree_spo2: Vec<i32> = vec![INF; 4 * s];

    println!();
    println!();
    println!("==============================================");
    println!("Use case 3: saturacao de oxigenio (MIN)");
    println!("==============================================");
    println!("Medicoes de SpO2 % (06h, 08h, 10h, 12h, 14h, 16h, 18h, 20h):");
    println!("  {:?}", spo2);

    build_min(&spo2, &mut tree_spo2, 1, 0, s - 1);
    println!("Arvore montada. Menor saturacao do dia = {} %", tree_spo2[1]);
    println!();


    // Caso 1: menor saturacao entre 10h e 16h (indices 2 a 5)
    println!("Caso 1 - menor saturacao entre 10h e 16h");
    let s1 = consulta_min(&tree_spo2, 1, 0, s - 1, 2, 5);
    println!("  Segment tree responde: {}", s1);
    let fatia = &spo2[2..6];
    println!("  Conferindo na mao:    min de {:?} = {}", fatia, fatia.iter().min().unwrap());
    println!();


    // Caso 2: medicao das 10h estava errada, o valor certo era 90
    println!("Caso 2 - corrigindo medicao das 10h: de 92 para 90");
    atualiza_min(&mut spo2, &mut tree_spo2, 1, 0, s - 1, 2, 90);
    println!("  SpO2 apos correcao:           {:?}", spo2);
    println!("  Menor saturacao do dia agora: {}", tree_spo2[1]);
    println!();


    // Caso 3: repete a pergunta com o valor corrigido
    println!("Caso 3 - menor saturacao entre 10h e 16h (depois da correcao)");
    let s3 = consulta_min(&tree_spo2, 1, 0, s - 1, 2, 5);
    println!("  Segment tree responde: {}", s3);
    let fatia = &spo2[2..6];
    println!("  Conferindo na mao:    min de {:?} = {}", fatia, fatia.iter().min().unwrap());
}


// ==========================================================
// TESTES UNITARIOS
// ==========================================================
//
// rustc --test simple-operations.rs -o simple-operations-test && ./simple-operations-test

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------------------------------------
    // MAX (notas)
    // ----------------------------------------------------------

    fn montar_tree_max() -> (Vec<i32>, Vec<i32>) {
        let notas = vec![7, 3, 9, 5, 8, 2, 6, 4];
        let m = notas.len();
        let mut tree = vec![-1; 4 * m];
        build_max(&notas, &mut tree, 1, 0, m - 1);
        (notas, tree)
    }

    #[test]
    fn max_consulta_turma_toda() {
        let (notas, tree) = montar_tree_max();
        let m = notas.len();
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 0, m - 1), 9);
    }

    #[test]
    fn max_consulta_alunos_3_a_6() {
        let (notas, tree) = montar_tree_max();
        let m = notas.len();
        // indices 2..5 = valores [9, 5, 8, 2] -> max = 9
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 2, 5), 9);
    }

    #[test]
    fn max_consulta_alunos_6_a_8() {
        let (notas, tree) = montar_tree_max();
        let m = notas.len();
        // indices 5..7 = valores [2, 6, 4] -> max = 6
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 5, 7), 6);
    }

    #[test]
    fn max_consulta_elemento_unico() {
        let (notas, tree) = montar_tree_max();
        let m = notas.len();
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 0, 0), 7);
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 2, 2), 9);
    }

    #[test]
    fn max_atualizacao_nota_aluno5() {
        let (mut notas, mut tree) = montar_tree_max();
        let m = notas.len();
        // aluno 5 (indice 4) refez prova: nota 10
        atualiza_max(&mut notas, &mut tree, 1, 0, m - 1, 4, 10);
        assert_eq!(consulta_max(&tree, 1, 0, m - 1, 2, 5), 10);
        assert_eq!(tree[1], 10); // nova raiz
    }

    // ----------------------------------------------------------
    // SOMA (vendas)
    // ----------------------------------------------------------

    fn montar_tree_soma() -> (Vec<i32>, Vec<i32>) {
        let vendas = vec![4, 1, 3, 5, 2, 6, 1, 2];
        let n = vendas.len();
        let mut tree = vec![0; 4 * n];
        build(&vendas, &mut tree, 1, 0, n - 1);
        (vendas, tree)
    }

    #[test]
    fn soma_consulta_total() {
        let (vendas, tree) = montar_tree_soma();
        let n = vendas.len();
        // 4+1+3+5+2+6+1+2 = 24
        assert_eq!(consulta(&tree, 1, 0, n - 1, 0, n - 1), 24);
    }

    #[test]
    fn soma_consulta_dias_3_a_6() {
        let (vendas, tree) = montar_tree_soma();
        let n = vendas.len();
        // indices 2..5 = valores [3, 5, 2, 6] -> soma = 16
        assert_eq!(consulta(&tree, 1, 0, n - 1, 2, 5), 16);
    }

    #[test]
    fn soma_atualizacao_dia4() {
        let (mut vendas, mut tree) = montar_tree_soma();
        let n = vendas.len();
        // dia 4 (indice 3) corrigido de 5 para 7
        atualiza(&mut vendas, &mut tree, 1, 0, n - 1, 3, 7);
        // [3, 7, 2, 6] -> soma = 18
        assert_eq!(consulta(&tree, 1, 0, n - 1, 2, 5), 18);
        // total: 24 - 5 + 7 = 26
        assert_eq!(tree[1], 26);
    }

    // ----------------------------------------------------------
    // MIN (spo2)
    // ----------------------------------------------------------

    fn montar_tree_min() -> (Vec<i32>, Vec<i32>) {
        let spo2 = vec![98, 97, 92, 94, 96, 99, 95, 93];
        let s = spo2.len();
        let mut tree = vec![INF; 4 * s];
        build_min(&spo2, &mut tree, 1, 0, s - 1);
        (spo2, tree)
    }

    #[test]
    fn min_consulta_dia_todo() {
        let (spo2, tree) = montar_tree_min();
        let s = spo2.len();
        assert_eq!(consulta_min(&tree, 1, 0, s - 1, 0, s - 1), 92);
    }

    #[test]
    fn min_consulta_10h_a_16h() {
        let (spo2, tree) = montar_tree_min();
        let s = spo2.len();
        // indices 2..5 = valores [92, 94, 96, 99] -> min = 92
        assert_eq!(consulta_min(&tree, 1, 0, s - 1, 2, 5), 92);
    }

    #[test]
    fn min_consulta_elemento_unico() {
        let (spo2, tree) = montar_tree_min();
        let s = spo2.len();
        assert_eq!(consulta_min(&tree, 1, 0, s - 1, 0, 0), 98);
        assert_eq!(consulta_min(&tree, 1, 0, s - 1, 2, 2), 92);
    }

    #[test]
    fn min_atualizacao_medicao_10h() {
        let (mut spo2, mut tree) = montar_tree_min();
        let s = spo2.len();
        // medicao das 10h (indice 2) corrigida de 92 para 90
        atualiza_min(&mut spo2, &mut tree, 1, 0, s - 1, 2, 90);
        assert_eq!(consulta_min(&tree, 1, 0, s - 1, 2, 5), 90);
        assert_eq!(tree[1], 90); // novo min geral
    }
}
