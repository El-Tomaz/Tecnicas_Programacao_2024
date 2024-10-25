#include <iostream>
#include <cassert>

using namespace std;

int somaNaturais(int numero_natural){
    int soma = 0;

    for(int i = 0; i <= numero_natural; i++){  

        soma = soma + i;
   }

   return soma;
}

int main() {
    // Testes unitários
    assert(somaNaturais(5) == 15);  // 1 + 2 + 3 + 4 + 5 = 15
    assert(somaNaturais(1) == 1);   // 1 = 1
    assert(somaNaturais(10) == 55); // 1 + 2 + ... + 10 = 55
    assert(somaNaturais(0) == 0);   // Caso com n = 0
    assert(somaNaturais(3) == 6);   // 1 + 2 + 3 = 6

    cout << "Todos os testes passaram!" << endl;

    return 0;
}