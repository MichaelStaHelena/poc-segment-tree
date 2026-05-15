// loan-contracts.rs
//
// Segment Tree onde cada no guarda um CONTRATO (struct com varios atributos),
// nao um escalar. A mesma estrutura responde varias perguntas diferentes
// trocando so o merge e o elemento neutro - exatamente como simple-operations.rs.
//
// Demonstra os 5 casos com sentido semantico sobre uma carteira de 8 contratos:
//
//   Use case 1 - MAIS URGENTE       (min por dias)   -> contrato a cobrar primeiro
//   Use case 2 - MAIS FOLGADO       (max por dias)   -> contrato com mais prazo
//   Use case 3 - MENOR SALDO        (min por valor)  -> menor divida individual
//   Use case 4 - MAIOR EXPOSICAO    (max por valor)  -> maior divida individual
//   Use case 5 - EXPOSICAO TOTAL    (sum por valor)  -> R$ total da carteira
//
// Por que NAO existe "sum por dias"?
//   Vencimentos rodam em paralelo, nao em serie. Se A vence em 7 dias e B em 30,
//   A e pago no dia 7 DENTRO do prazo do B - voce nao espera 37 dias por nada.
//   Somar prazos individuais nao gera nada com unidade real, entao essa
//   combinacao foi excluida de proposito. Ver operations.md para detalhes.
//
// Como rodar:
//   rustc loan-contracts.rs && ./loan-contracts
//
// Como rodar os testes:
//   rustc --test loan-contracts.rs -o loan-contracts-test && ./loan-contracts-test


// ==========================================================
// MODELO DO DADO: ContratoEmprestimo
// ==========================================================

#[derive(Clone)]
struct ContratoEmprestimo {
    contrato_id:     u32,
    devedor:         String,
    valor:           f64,   // saldo devedor em aberto (R$)
    dias_para_pagar: i32,   // dias restantes ate o vencimento
}

impl ContratoEmprestimo {
    fn new(contrato_id: u32, devedor: &str, valor: f64, dias_para_pagar: i32) -> Self {
        ContratoEmprestimo {
            contrato_id,
            devedor: devedor.to_string(),
            valor,
            dias_para_pagar,
        }
    }

    fn display(&self) {
        println!(
            "Contrato #{} | {:<10} | R$ {:>10.2} | {} dias restantes",
            self.contrato_id, self.devedor, self.valor, self.dias_para_pagar
        );
    }
}

fn init_contratos() -> Vec<ContratoEmprestimo> {
    vec![
        ContratoEmprestimo::new(1, "Alice",  5_000.00, 30),
        ContratoEmprestimo::new(2, "Bob",   12_000.00,  7),
        ContratoEmprestimo::new(3, "Carol",  3_200.00, 45),
        ContratoEmprestimo::new(4, "David",  8_500.00, 14),
        ContratoEmprestimo::new(5, "Eve",    2_100.00,  2),
        ContratoEmprestimo::new(6, "Frank",  9_900.00, 21),
        ContratoEmprestimo::new(7, "Grace",  6_700.00, 60),
        ContratoEmprestimo::new(8, "Hank",   4_400.00,  9),
    ]
}

fn imprime_carteira(carteira: &[ContratoEmprestimo]) {
    println!("Carteira:");
    for c in carteira {
        print!("  ");
        c.display();
    }
    println!();
}


// ==========================================================
// USE CASE 1: contrato MAIS URGENTE (min por dias)
// ==========================================================
//
// Merge: o contrato com MENOS dias_para_pagar vence.
// Neutro: contrato sentinela com dias_para_pagar = i32::MAX (nunca ganha o min).

fn neutro_urg() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", 0.0, i32::MAX)
}

fn merge_urg(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.dias_para_pagar <= b.dias_para_pagar { a.clone() } else { b.clone() }
}

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
    tree[no] = merge_urg(&e, &d);
}

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


// ==========================================================
// USE CASE 2: contrato MAIS FOLGADO (max por dias)
// ==========================================================
//
// Merge: o contrato com MAIS dias_para_pagar vence.
// Neutro: contrato sentinela com dias_para_pagar = i32::MIN.

fn neutro_folga() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", 0.0, i32::MIN)
}

fn merge_folga(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.dias_para_pagar >= b.dias_para_pagar { a.clone() } else { b.clone() }
}

fn build_folga(
    contratos: &[ContratoEmprestimo], tree: &mut [ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        tree[no] = contratos[inicio].clone();
        return;
    }
    let meio = (inicio + fim) / 2;
    build_folga(contratos, tree, 2 * no, inicio, meio);
    build_folga(contratos, tree, 2 * no + 1, meio + 1, fim);
    let e = tree[2 * no].clone();
    let d = tree[2 * no + 1].clone();
    tree[no] = merge_folga(&e, &d);
}

fn consulta_folga(
    tree: &[ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize, l: usize, r: usize,
) -> ContratoEmprestimo {
    if r < inicio || fim < l { return neutro_folga(); }
    if l <= inicio && fim <= r { return tree[no].clone(); }
    let meio = (inicio + fim) / 2;
    let e = consulta_folga(tree, 2 * no, inicio, meio, l, r);
    let d = consulta_folga(tree, 2 * no + 1, meio + 1, fim, l, r);
    merge_folga(&e, &d)
}


// ==========================================================
// USE CASE 3: MENOR SALDO devedor (min por valor)
// ==========================================================
//
// Merge: o contrato com MENOS valor vence.
// Neutro: contrato sentinela com valor = f64::INFINITY.

fn neutro_menor() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", f64::INFINITY, 0)
}

fn merge_menor(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.valor <= b.valor { a.clone() } else { b.clone() }
}

fn build_menor(
    contratos: &[ContratoEmprestimo], tree: &mut [ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        tree[no] = contratos[inicio].clone();
        return;
    }
    let meio = (inicio + fim) / 2;
    build_menor(contratos, tree, 2 * no, inicio, meio);
    build_menor(contratos, tree, 2 * no + 1, meio + 1, fim);
    let e = tree[2 * no].clone();
    let d = tree[2 * no + 1].clone();
    tree[no] = merge_menor(&e, &d);
}

fn consulta_menor(
    tree: &[ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize, l: usize, r: usize,
) -> ContratoEmprestimo {
    if r < inicio || fim < l { return neutro_menor(); }
    if l <= inicio && fim <= r { return tree[no].clone(); }
    let meio = (inicio + fim) / 2;
    let e = consulta_menor(tree, 2 * no, inicio, meio, l, r);
    let d = consulta_menor(tree, 2 * no + 1, meio + 1, fim, l, r);
    merge_menor(&e, &d)
}


// ==========================================================
// USE CASE 4: MAIOR EXPOSICAO individual (max por valor)
// ==========================================================
//
// Merge: o contrato com MAIS valor vence.
// Neutro: contrato sentinela com valor = f64::NEG_INFINITY.

fn neutro_maior() -> ContratoEmprestimo {
    ContratoEmprestimo::new(0, "-", f64::NEG_INFINITY, 0)
}

fn merge_maior(a: &ContratoEmprestimo, b: &ContratoEmprestimo) -> ContratoEmprestimo {
    if a.valor >= b.valor { a.clone() } else { b.clone() }
}

fn build_maior(
    contratos: &[ContratoEmprestimo], tree: &mut [ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        tree[no] = contratos[inicio].clone();
        return;
    }
    let meio = (inicio + fim) / 2;
    build_maior(contratos, tree, 2 * no, inicio, meio);
    build_maior(contratos, tree, 2 * no + 1, meio + 1, fim);
    let e = tree[2 * no].clone();
    let d = tree[2 * no + 1].clone();
    tree[no] = merge_maior(&e, &d);
}

fn consulta_maior(
    tree: &[ContratoEmprestimo],
    no: usize, inicio: usize, fim: usize, l: usize, r: usize,
) -> ContratoEmprestimo {
    if r < inicio || fim < l { return neutro_maior(); }
    if l <= inicio && fim <= r { return tree[no].clone(); }
    let meio = (inicio + fim) / 2;
    let e = consulta_maior(tree, 2 * no, inicio, meio, l, r);
    let d = consulta_maior(tree, 2 * no + 1, meio + 1, fim, l, r);
    merge_maior(&e, &d)
}


// ==========================================================
// USE CASE 5: EXPOSICAO TOTAL (sum por valor)
// ==========================================================
//
// Merge: soma os valores.
// Neutro: 0.0 (elemento neutro da soma).
//
// Aqui o tree NAO guarda contratos - guarda escalares (f64, R$ acumulado),
// porque o agregado da soma e um numero, nao um contrato. As folhas extraem
// so o campo `valor` do contrato; os nos internos somam.

fn build_soma(
    contratos: &[ContratoEmprestimo], tree: &mut [f64],
    no: usize, inicio: usize, fim: usize,
) {
    if inicio == fim {
        // *** folha guarda so o campo `valor` ***
        tree[no] = contratos[inicio].valor;
        return;
    }
    let meio = (inicio + fim) / 2;
    build_soma(contratos, tree, 2 * no, inicio, meio);
    build_soma(contratos, tree, 2 * no + 1, meio + 1, fim);
    // *** merge: soma os filhos ***
    tree[no] = tree[2 * no] + tree[2 * no + 1];
}

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

fn atualiza_soma(
    contratos: &mut [ContratoEmprestimo], tree: &mut [f64],
    no: usize, inicio: usize, fim: usize, posicao: usize, novo: ContratoEmprestimo,
) {
    if inicio == fim {
        tree[no] = novo.valor;
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


// ==========================================================
// main: executa os 5 demos
// ==========================================================
fn main() {

    // ------------------------------------------------------
    // Use case 1: MAIS URGENTE (min por dias)
    // ------------------------------------------------------
    let mut contratos = init_contratos();
    let c = contratos.len();
    let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_urg()).collect();

    println!("==============================================");
    println!("Use case 1: contrato MAIS URGENTE (min por dias)");
    println!("==============================================");
    imprime_carteira(&contratos);

    build_urg(&contratos, &mut tree, 1, 0, c - 1);
    print!("Mais urgente da carteira toda: ");
    tree[1].display();
    println!();

    println!("Caso 1 - mais urgente entre contratos 1 e 4 (indices 0..3)");
    let r = consulta_urg(&tree, 1, 0, c - 1, 0, 3);
    print!("  Segment tree responde: "); r.display();
    let m = contratos[0..4].iter().min_by_key(|x| x.dias_para_pagar).unwrap();
    print!("  Conferindo na mao:     "); m.display();
    println!();

    println!("Caso 2 - mais urgente entre contratos 5 e 8 (indices 4..7)");
    let r = consulta_urg(&tree, 1, 0, c - 1, 4, 7);
    print!("  Segment tree responde: "); r.display();
    let m = contratos[4..8].iter().min_by_key(|x| x.dias_para_pagar).unwrap();
    print!("  Conferindo na mao:     "); m.display();
    println!();

    println!("Caso 3 - Eve renegociou: novo prazo = 90 dias");
    atualiza_urg(&mut contratos, &mut tree, 1, 0, c - 1, 4,
                 ContratoEmprestimo::new(5, "Eve", 1_500.00, 90));
    let r = consulta_urg(&tree, 1, 0, c - 1, 0, 7);
    print!("  Mais urgente agora:    "); r.display();
    let m = contratos.iter().min_by_key(|x| x.dias_para_pagar).unwrap();
    print!("  Conferindo na mao:     "); m.display();


    // ------------------------------------------------------
    // Use case 2: MAIS FOLGADO (max por dias)
    // ------------------------------------------------------
    let contratos = init_contratos();
    let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_folga()).collect();

    println!();
    println!();
    println!("==============================================");
    println!("Use case 2: contrato MAIS FOLGADO (max por dias)");
    println!("==============================================");
    imprime_carteira(&contratos);

    build_folga(&contratos, &mut tree, 1, 0, c - 1);
    print!("Mais folgado da carteira toda: ");
    tree[1].display();
    println!();

    println!("Caso 1 - mais folgado entre contratos 1 e 4 (indices 0..3)");
    let r = consulta_folga(&tree, 1, 0, c - 1, 0, 3);
    print!("  Segment tree responde: "); r.display();
    let m = contratos[0..4].iter().max_by_key(|x| x.dias_para_pagar).unwrap();
    print!("  Conferindo na mao:     "); m.display();


    // ------------------------------------------------------
    // Use case 3: MENOR SALDO (min por valor)
    // ------------------------------------------------------
    let contratos = init_contratos();
    let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_menor()).collect();

    println!();
    println!();
    println!("==============================================");
    println!("Use case 3: MENOR SALDO devedor (min por valor)");
    println!("==============================================");
    imprime_carteira(&contratos);

    build_menor(&contratos, &mut tree, 1, 0, c - 1);
    print!("Menor saldo da carteira toda: ");
    tree[1].display();
    println!();

    println!("Caso 1 - menor saldo entre contratos 1 e 4 (indices 0..3)");
    let r = consulta_menor(&tree, 1, 0, c - 1, 0, 3);
    print!("  Segment tree responde: "); r.display();
    // f64 nao implementa Ord; comparamos via partial_cmp.
    let m = contratos[0..4].iter()
        .min_by(|a, b| a.valor.partial_cmp(&b.valor).unwrap()).unwrap();
    print!("  Conferindo na mao:     "); m.display();


    // ------------------------------------------------------
    // Use case 4: MAIOR EXPOSICAO (max por valor)
    // ------------------------------------------------------
    let contratos = init_contratos();
    let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_maior()).collect();

    println!();
    println!();
    println!("==============================================");
    println!("Use case 4: MAIOR EXPOSICAO individual (max por valor)");
    println!("==============================================");
    imprime_carteira(&contratos);

    build_maior(&contratos, &mut tree, 1, 0, c - 1);
    print!("Maior exposicao da carteira toda: ");
    tree[1].display();
    println!();

    println!("Caso 1 - maior exposicao entre contratos 5 e 8 (indices 4..7)");
    let r = consulta_maior(&tree, 1, 0, c - 1, 4, 7);
    print!("  Segment tree responde: "); r.display();
    let m = contratos[4..8].iter()
        .max_by(|a, b| a.valor.partial_cmp(&b.valor).unwrap()).unwrap();
    print!("  Conferindo na mao:     "); m.display();


    // ------------------------------------------------------
    // Use case 5: EXPOSICAO TOTAL (sum por valor)
    // ------------------------------------------------------
    let mut contratos = init_contratos();
    let mut tree: Vec<f64> = vec![0.0; 4 * c];

    println!();
    println!();
    println!("==============================================");
    println!("Use case 5: EXPOSICAO TOTAL (sum por valor)");
    println!("==============================================");
    imprime_carteira(&contratos);

    build_soma(&contratos, &mut tree, 1, 0, c - 1);
    println!("Exposicao total da carteira: R$ {:.2}", tree[1]);
    println!();

    println!("Caso 1 - exposicao total entre contratos 1 e 4 (indices 0..3)");
    let total = consulta_soma(&tree, 1, 0, c - 1, 0, 3);
    let manual: f64 = contratos[0..4].iter().map(|x| x.valor).sum();
    println!("  Segment tree responde: R$ {:.2}", total);
    println!("  Conferindo na mao:     R$ {:.2}", manual);
    println!();

    println!("Caso 2 - Eve quitou parcialmente, novo saldo = R$ 500,00");
    atualiza_soma(&mut contratos, &mut tree, 1, 0, c - 1, 4,
                  ContratoEmprestimo::new(5, "Eve", 500.00, 2));
    let manual: f64 = contratos.iter().map(|x| x.valor).sum();
    println!("  Exposicao total agora: R$ {:.2}", tree[1]);
    println!("  Conferindo na mao:     R$ {:.2}", manual);
}


// ==========================================================
// TESTES UNITARIOS
// ==========================================================
//
// rustc --test loan-contracts.rs -o loan-contracts-test && ./loan-contracts-test

#[cfg(test)]
mod tests {
    use super::*;

    // carteira: Alice(1,5000,30) Bob(2,12000,7) Carol(3,3200,45) David(4,8500,14)
    //           Eve(5,2100,2)   Frank(6,9900,21) Grace(7,6700,60) Hank(8,4400,9)

    // ----------------------------------------------------------
    // Use case 1: MAIS URGENTE (min por dias)
    // ----------------------------------------------------------

    fn montar_tree_urg(contratos: &[ContratoEmprestimo]) -> Vec<ContratoEmprestimo> {
        let c = contratos.len();
        let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_urg()).collect();
        build_urg(contratos, &mut tree, 1, 0, c - 1);
        tree
    }

    #[test]
    fn urg_raiz_e_eve() {
        let contratos = init_contratos();
        let tree = montar_tree_urg(&contratos);
        assert_eq!(tree[1].contrato_id, 5); // Eve - 2 dias
        assert_eq!(tree[1].dias_para_pagar, 2);
    }

    #[test]
    fn urg_query_1_a_4_e_bob() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_urg(&contratos);
        // Alice(30) Bob(7) Carol(45) David(14) -> Bob mais urgente
        let r = consulta_urg(&tree, 1, 0, c - 1, 0, 3);
        assert_eq!(r.contrato_id, 2);
        assert_eq!(r.dias_para_pagar, 7);
    }

    #[test]
    fn urg_query_5_a_8_e_eve() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_urg(&contratos);
        // Eve(2) Frank(21) Grace(60) Hank(9) -> Eve mais urgente
        let r = consulta_urg(&tree, 1, 0, c - 1, 4, 7);
        assert_eq!(r.contrato_id, 5);
    }

    #[test]
    fn urg_apos_renegociacao_eve_bob_assume() {
        let mut contratos = init_contratos();
        let c = contratos.len();
        let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_urg()).collect();
        build_urg(&contratos, &mut tree, 1, 0, c - 1);
        atualiza_urg(&mut contratos, &mut tree, 1, 0, c - 1, 4,
                     ContratoEmprestimo::new(5, "Eve", 1_500.00, 90));
        // Eve agora tem 90 dias; Bob (7) assume como mais urgente
        let r = consulta_urg(&tree, 1, 0, c - 1, 0, 7);
        assert_eq!(r.contrato_id, 2);
        assert_eq!(r.dias_para_pagar, 7);
    }

    // ----------------------------------------------------------
    // Use case 2: MAIS FOLGADO (max por dias)
    // ----------------------------------------------------------

    fn montar_tree_folga(contratos: &[ContratoEmprestimo]) -> Vec<ContratoEmprestimo> {
        let c = contratos.len();
        let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_folga()).collect();
        build_folga(contratos, &mut tree, 1, 0, c - 1);
        tree
    }

    #[test]
    fn folga_raiz_e_grace() {
        let contratos = init_contratos();
        let tree = montar_tree_folga(&contratos);
        assert_eq!(tree[1].contrato_id, 7); // Grace - 60 dias
        assert_eq!(tree[1].dias_para_pagar, 60);
    }

    #[test]
    fn folga_query_1_a_4_e_carol() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_folga(&contratos);
        // Alice(30) Bob(7) Carol(45) David(14) -> Carol mais folgado
        let r = consulta_folga(&tree, 1, 0, c - 1, 0, 3);
        assert_eq!(r.contrato_id, 3);
        assert_eq!(r.dias_para_pagar, 45);
    }

    // ----------------------------------------------------------
    // Use case 3: MENOR SALDO (min por valor)
    // ----------------------------------------------------------

    fn montar_tree_menor(contratos: &[ContratoEmprestimo]) -> Vec<ContratoEmprestimo> {
        let c = contratos.len();
        let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_menor()).collect();
        build_menor(contratos, &mut tree, 1, 0, c - 1);
        tree
    }

    #[test]
    fn menor_raiz_e_eve() {
        let contratos = init_contratos();
        let tree = montar_tree_menor(&contratos);
        assert_eq!(tree[1].contrato_id, 5); // Eve - R$2.100
    }

    #[test]
    fn menor_query_1_a_4_e_carol() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_menor(&contratos);
        // Alice(5000) Bob(12000) Carol(3200) David(8500) -> Carol menor saldo
        let r = consulta_menor(&tree, 1, 0, c - 1, 0, 3);
        assert_eq!(r.contrato_id, 3);
    }

    // ----------------------------------------------------------
    // Use case 4: MAIOR EXPOSICAO (max por valor)
    // ----------------------------------------------------------

    fn montar_tree_maior(contratos: &[ContratoEmprestimo]) -> Vec<ContratoEmprestimo> {
        let c = contratos.len();
        let mut tree: Vec<ContratoEmprestimo> = (0..4 * c).map(|_| neutro_maior()).collect();
        build_maior(contratos, &mut tree, 1, 0, c - 1);
        tree
    }

    #[test]
    fn maior_raiz_e_bob() {
        let contratos = init_contratos();
        let tree = montar_tree_maior(&contratos);
        assert_eq!(tree[1].contrato_id, 2); // Bob - R$12.000
    }

    #[test]
    fn maior_query_5_a_8_e_frank() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_maior(&contratos);
        // Eve(2100) Frank(9900) Grace(6700) Hank(4400) -> Frank maior exposicao
        let r = consulta_maior(&tree, 1, 0, c - 1, 4, 7);
        assert_eq!(r.contrato_id, 6);
    }

    // ----------------------------------------------------------
    // Use case 5: EXPOSICAO TOTAL (sum por valor)
    // ----------------------------------------------------------

    fn montar_tree_soma(contratos: &[ContratoEmprestimo]) -> Vec<f64> {
        let c = contratos.len();
        let mut tree = vec![0.0f64; 4 * c];
        build_soma(contratos, &mut tree, 1, 0, c - 1);
        tree
    }

    #[test]
    fn soma_total_carteira() {
        let contratos = init_contratos();
        let tree = montar_tree_soma(&contratos);
        // 5000+12000+3200+8500+2100+9900+6700+4400 = 51800
        assert!((tree[1] - 51_800.0).abs() < 0.01);
    }

    #[test]
    fn soma_query_1_a_4() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_soma(&contratos);
        // 5000+12000+3200+8500 = 28700
        let total = consulta_soma(&tree, 1, 0, c - 1, 0, 3);
        assert!((total - 28_700.0).abs() < 0.01);
    }

    #[test]
    fn soma_query_5_a_8() {
        let contratos = init_contratos();
        let c = contratos.len();
        let tree = montar_tree_soma(&contratos);
        // 2100+9900+6700+4400 = 23100
        let total = consulta_soma(&tree, 1, 0, c - 1, 4, 7);
        assert!((total - 23_100.0).abs() < 0.01);
    }

    #[test]
    fn soma_apos_quitacao_parcial_eve() {
        let mut contratos = init_contratos();
        let c = contratos.len();
        let mut tree = vec![0.0f64; 4 * c];
        build_soma(&contratos, &mut tree, 1, 0, c - 1);
        // Eve quitou parcialmente: novo saldo R$500
        atualiza_soma(&mut contratos, &mut tree, 1, 0, c - 1, 4,
                      ContratoEmprestimo::new(5, "Eve", 500.00, 2));
        // 51800 - 2100 + 500 = 50200
        assert!((tree[1] - 50_200.0).abs() < 0.01);
    }
}
