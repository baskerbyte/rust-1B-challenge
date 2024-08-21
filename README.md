<img width=100% src="https://capsule-render.vercel.app/api?type=waving&color=179aad&height=120&section=header"/>

[![Typing SVG](https://readme-typing-svg.herokuapp.com/?color=0c6875&size=40&center=true&vCenter=true&width=1000&lines=Olá,+seja+bem+vindo(a)!;Ao+nosso+projeto!+:%29)](https://git.io/typing-svg)

<div align="center" />

<br>

# Sobre o Projeto - Desafio

  
#### O projeto envolve o desenvolvimento de programas capazes de processar arquivos com diferentes magnitudes de dados:
 1 - Milhão de linhas, com números randomicos entre 0 e 1 milhão
 <br>1 - Bilhão de linhas, com números randomicos entre 0 e 1 bilhão
<br>1 - Trilhão de linhas, com números randomicos entre 0 e 1 trilhão

#### Este desafio foi proposto pelo professor Jackson Gomes de Souza durante a segunda aula da disciplina de Estruturas e Complexidade de Algoritmos.

</div> 

<div align="center" />

<br>

## Integrantes

  
<p align="center" class="github-links">
<a href="https://github.com/Geisbelly">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Geis.png" width="210px" alt="Geisbelly"></a>
    <img src="https://github.com/Geisbelly-vic/Curso-de-Figma-2024-02/blob/main/Imagens/Integrante/Rectangle%20375.png" width="80px">
<a href="https://github.com/baskerbyte">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Luis.png" width="210px" alt="Luis Fernando"></a>
    <img src="https://github.com/Geisbelly-vic/Curso-de-Figma-2024-02/blob/main/Imagens/Integrante/Rectangle%20375.png" width="80px">
<a href="https://github.com/MariAntonia-010">
    <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/Mary.png" width="210px" alt="Maria Antônia"></a>
</p>

<br>

  
## Resultados




### Divisão em 2 Partes
</div>
<div align="left" />

> ### Resumos
> #### Parte 1
> Programa para 1 milhão e bilhão;
> <br>Linguagem: Rust;
> <br>Metodo: Multi-threads, paralelismo, batches+lock;
> <br>Divisão: Escrita e leitura, separados.
> 
> <br>Lógica:
> <br>Para a escrita no arquivo, foram utilizadas várias threads para permitir o paralelismo. Cada thread é encarregada de gerar números em batches. Após a conclusão de cada batch, a thread adquire um lock no mutex (para garantir que não haja sobrescrição concorrente) e escreve os dados no arquivo.
>
> Para otimizar a busca em um arquivo, foram criadas várias threads, cada uma responsável por ler um segmento do arquivo. Cada thread verifica se o número desejado está presente em sua parte designada, permitindo uma busca paralela eficiente.
> #### Parte 2
> Programa para 1 trilhão;
> <br>Linguagem: Python;
> <br>Metodo: Multi-tarefas, funções sincronas e assincronas, lotes(blocos), eventos de monitoramento;
> <br>Divisão: Escrita, leitura e limpeza, mesclados.
>
>  <br>Lógica:
> <br> O programa é estruturado em um conjunto definido de tarefas que, trabalhando em blocos, escreve cada número em uma linha. Quando o número de linhas no arquivo atinge um limite global, a função síncrona de busca é ativada, interrompendo as tarefas paralelas. Esta função procura o número desejado dentro do arquivo, sinaliza um evento caso o número seja encontrado (o que resulta na finalização de todas as atividades), e em seguida limpa o arquivo para evitar que o tamanho atinja + de 1TB. Após a limpeza, as atividades são retomadas. O processo continua até que se atinja a quantidade pré-definida de linhas ou até que o número seja localizado.
</div>

---


<div align="center" />
<br>
  
## Parte 1
### Analise para 1 milhão
<div style="display: flex; align-items: flex-start;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1mCPU.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1mRAm.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1mDisco.jpeg">
</div>

#### Tempo de execução total: 0.09 segundos
<br>

### Analise para 1 bilhão
<div style="display: flex; align-items: flex-start;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1bCPU.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1bRAm.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1bDisco.jpeg">
</div>

#### Tempo de execução total: 1.98 segundos

<br>

## Parte 2
### Analise para 1 trilhão
<div style="display: flex; align-items: flex-start;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1tCPU.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1tRAm.jpeg" style="margin-right: 50px;">
  <img src="https://github.com/baskerbyte/rust-1B-challenge/blob/dev/fotos/1tDisco.jpeg">
</div>

#### Tempo de execução total: 5.46 segundos

</div>

<img width=100% src="https://capsule-render.vercel.app/api?type=waving&color=179aad&height=120&section=footer"/>
