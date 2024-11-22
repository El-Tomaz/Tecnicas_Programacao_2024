/*

Crie uma função chamada encontraPessoa que receba um vetor de structs Pessoa, seu tamanho, e o nome de uma pessoa. A função deve retornar o índice da pessoa no vetor, ou -1 se a pessoa não for encontrada.

*/

#include <iostream>
#include <cassert>

using namespace std;

struct Pessoa{
	string nome;
	int idade;
	float altura;
};

int encontraPessoa(Pessoa *pessoas, int n_pessoas, string nome){
	for(int i = 0; i < n_pessoas; i++){
		if(pessoas[i].nome == nome){
			return i;
		}
	}
	return -1;
	
}


int main() {
    Pessoa pessoas[] = {
        {"Joao", 25, 1.75},
        {"Maria", 30, 1.65},
        {"Pedro", 22, 1.80}
    };

    assert(encontraPessoa(pessoas, 3, "Maria") == 1);
    assert(encontraPessoa(pessoas, 3, "Pedro") == 2);
    assert(encontraPessoa(pessoas, 3, "Ana") == -1);

    cout << "Todos os testes passaram!" << endl;
    return 0;
}