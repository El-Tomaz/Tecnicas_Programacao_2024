/*
Crie uma função chamada mediaGeometrica que receba dois números do tipo float
como parâmetros e retorne a média geométrica desses números. A fórmula para
calcular a média geométrica de dois números é multiplicar os dois números e
depois calcular a raiz quadrada do resultado.
*/

fn media_geometrica(operando1: f32, operando2: f32) -> f32 {
    (operando1 * operando2).sqrt()
}

fn main() {
    const TOLERANCIA: f32 = 0.0001;
    assert!((media_geometrica(4.0, 9.0) - 6.0).abs() < TOLERANCIA);
    assert!((media_geometrica(1.0, 1.0) - 1.0).abs() < TOLERANCIA);
    assert!((media_geometrica(0.25, 0.75) - 0.4330).abs() < TOLERANCIA);
    assert!((media_geometrica(2.0, 8.0) - 4.0).abs() < TOLERANCIA);

    println!("Todos os testes passaram!");
}
