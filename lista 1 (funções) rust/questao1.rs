// Crie uma função chamada somaQuadrados que receba dois números inteiros como parâmetros
// e retorne a soma dos quadrados desses dois números. Por exemplo, a soma dos quadrados de 3 e 4 é 9 + 16 = 25.

fn somaQuadrados(parcela1: i32, parcela2: i32) -> i32 {
    parcela1 * parcela1 + parcela2 * parcela2
}

fn main() {
    assert!(somaQuadrados(3, 4) == 25);
    assert!(somaQuadrados(0, 0) == 0);
    assert!(somaQuadrados(-3, 4) == 25);
    assert!(somaQuadrados(5, -5) == 50);
    assert!(somaQuadrados(1, 1) == 2);

    println!("Todos os testes passaram!");
}
