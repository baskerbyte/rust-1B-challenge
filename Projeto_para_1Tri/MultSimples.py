import time
import asyncio
from random import randint
import aiofiles

start_time = time.time()

# Variáveis
arquivo_compactado = "Trabalho-4/Arquivo.txt"
ex = 12  # Alterar - expoente do tamanho do arquivo
qt_tasks = 10  # Alterar - quantidade de tarefas para otimizar
linha_limite = 100000000  # Limite para limpar o arquivo

# Cálculos e variáveis globais
num_chaves = 10 ** ex
nume = num_chaves  # Número de busca
print(f"Objetivo: {nume}")

# Evento para sinalizar se o número foi encontrado
encontrado_evento = asyncio.Event()

# Contador de linhas global
contador_linhas = 0
contador_linhas_lock = asyncio.Lock()

# Função para limpar o arquivo
async def limpar_arquivo():
    async with aiofiles.open(arquivo_compactado, 'w') as arquivo:
        # Abrir o arquivo em modo de escrita ('w') limpa seu conteúdo
        pass

# Função para gerar uma parte do arquivo com números sequenciais e procurar o número
async def gerar_parte_arquivo(nome, inicio, fim):
    global contador_linhas
    async with aiofiles.open(arquivo_compactado, 'a') as arquivoEscrever:
        bloco_tamanho = 100000  # Tamanho do bloco de 100.000
        for i in range(inicio, fim + 1, bloco_tamanho):
            # Se o número já foi encontrado, interrompa a execução
            if encontrado_evento.is_set():
                return

            bloco_inicio = i
            bloco_fim = min(i + bloco_tamanho - 1, fim)
            
            # Verificar se o número desejado está dentro deste bloco
            if bloco_inicio <= nume <= bloco_fim:
                # Escrever apenas o bloco onde o número desejado está
                for j in range(bloco_inicio, bloco_fim + 1):
                    await arquivoEscrever.write(f"{j}\n")

                    if j == nume:
                        
                        print("Número localizado")
                        await arquivoEscrever.close()
                        
                        # Sinaliza que o número foi encontrado
                        encontrado_evento.set()
                        break

                    # Atualizar o contador de linhas globalmente
                    async with contador_linhas_lock:
                        contador_linhas += 1

                # Limpar o arquivo se o número total de linhas atingiu o limite
                async with contador_linhas_lock:
                    if contador_linhas >= linha_limite:
                        print(f"Limpando arquivo após {contador_linhas} linhas.")
                        await limpar_arquivo()
                        contador_linhas = 0

                # Sair do loop após encontrar o número e processar o bloco
                break

async def tarefa_gerar(nome, intervalo):
    inicio = nome * intervalo + 1
    fim = (nome + 1) * intervalo
    if inicio <= nume <= fim:
        await gerar_parte_arquivo(nome, inicio, fim)

async def main():
    # Limpar o arquivo antes de iniciar a geração
    await limpar_arquivo()
    
    # Dividir a geração do arquivo em tarefas
    intervalo = num_chaves // qt_tasks
    tarefas_gerar = [
        asyncio.create_task(tarefa_gerar(nome, intervalo))
        for nome in range(qt_tasks)
    ]
    
    # Aguarda a conclusão da geração do arquivo
    await asyncio.gather(*tarefas_gerar)

    if encontrado_evento.is_set():
        print(f"Número {nume} encontrado.")
    else:
        print(f"Número {nume} não encontrado.")

asyncio.run(main())

end_time = time.time()
print(f"Tempo de execução total: {end_time - start_time} segundos")
