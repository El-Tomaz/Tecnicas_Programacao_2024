/*

Crie uma função chamada isVogal que receba um caractere do tipo char como parâmetro 
e retorne true se o caractere for uma vogal (tanto maiúscula quanto minúscula), e false caso contrário.

*/

fn is_vogal(letra: char) -> bool {
    match letra {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

fn main() {
    // Testes unitários
    assert!(is_vogal('a') == true);
    assert!(is_vogal('E') == true);
    assert!(is_vogal('z') == false);
    assert!(is_vogal('U') == true);
    assert!(is_vogal('b') == false);

    println!("Todos os testes passaram!");
}
