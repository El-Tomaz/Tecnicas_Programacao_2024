/*

Crie uma função chamada somaNaturais que receba um número inteiro positivo n como parâmetro
 e retorne a soma de todos os números naturais de 1 até n. 
 A função deve utilizar a estrutura de repetição for para calcular a soma.

*/

fn soma_naturais(n: u32) -> u32 {
    let mut soma_acumulada: u32 = 0;
    for i in 1..(n + 1) {
        soma_acumulada += i;
    }
    soma_acumulada
}

fn main() {
    assert!(soma_naturais(5) == 15); // 1 + 2 + 3 + 4 + 5 = 15
    assert!(soma_naturais(1) == 1); // 1 = 1
    assert!(soma_naturais(10) == 55); // 1 + 2 + ... + 10 = 55
    assert!(soma_naturais(0) == 0); // Caso com n = 0
    assert!(soma_naturais(3) == 6); // 1 + 2 + 3 = 6

    println!("Todos os testes passaram!");
}
