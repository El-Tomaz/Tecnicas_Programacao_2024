/*

Crie uma função chamada somaArray que receba um array de inteiros e o tamanho do array como parâmetros,
e retorne a soma de todos os elementos do array. A função deve utilizar um laço for para somar os elementos.

*/

fn soma_array(array: &i32, size:u32) -> i32{
    let mut soma_acumulada:i32 = 0;
    println!("{}",array[2]);
}

fn main(){
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [10, 20, 30];
    let arr3 = [-1, -2, -3, -4, -5];
    let arr4 = [0, 0, 0, 0, 0];

// Testes unitários
assert!(soma_array(&arr1 as &i32, 5) == 15);    /*// 1 + 2 + 3 + 4 + 5 = 15 
assert!(soma_array(arr2, 3) == 60);    // 10 + 20 + 30 = 60
assert!(soma_array(arr3, 5) == -15);   // -1 + -2 + -3 + -4 + -5 = -15
assert!(soma_array(arr4, 5) == 0);     // 0 + 0 + 0 + 0 + 0 = 0
assert!(soma_array(arr1, 0) == 0);     // Caso com array de tamanho 0
*/
}