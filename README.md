# Heap_sort

Ordenação de uma vetor de números com o método Heap Sort.

OBS: Esse algoritmo roda na metade do tempo da melhor versão do Insertion_sort e do Merge_sort!!!

Para executar o algoritmos, basta digitar o seguinte comando:

cargo run numero_experimentos tamanho_vetor >> nome_arquivo.txt

numero_experimento é um número inteior que representa a quantidade de vezes que geraremos um vetor aleatório e faremos sua ordenação in-place. Faremos parse dessa String para usize.

tamanho_vetor é um número inteiro que representa a quantidade números aleatórios que serão gerados e armazenados no vetor, o qual faremos sua ordenação in-place. Deve ser possível fazer parse para usize.

Exemplos:

cargo run --release 100 5000 >> resultados.txt

Escreve o seguinte output no arquivo:

//---------------------------

Function: Heap_sort

Número de experimentos: 100

Tamanho da lista de números: 5000

Tempo total: 22.791159ms

//--------------------------

