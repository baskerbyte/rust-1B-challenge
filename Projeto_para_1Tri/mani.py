import time
import asyncio
import aiofiles

start_time = time.time()

# Variáveis
arquivo_compactado = "Trabalho-4/Arquivo.txt"
ex = 12  # Alterar - expoente do tamanho do arquivo
qt_tasks = 10  # Alterar - quantidade de tarefas para otimizar
linha_limite = 100000000  # Limite para limpar o arquivo

# Cálculos e variáveis globais
num_chaves = 10 ** ex
nume = 500000000000  # Número de busca
print(f"Objetivo: {nume}")

# Evento para sinalizar se o número foi encontrado
encontrado_evento = asyncio.Event()

# Contador de linhas global
contador_linhas = 0
contador_linhas_lock = asyncio.Lock()

# Função para limpar o arquivo
async def limpar_arquivo():
    async with aiofiles.open(arquivo_compactado, 'w') as arquivo:
        pass  # Abrir o arquivo em modo de escrita ('w') limpa seu conteúdo

# Função para gerar uma parte do arquivo com números sequenciais e procurar o número
async def gerar_parte_arquivo(nome, inicio, fim):
    global contador_linhas
    async with aiofiles.open(arquivo_compactado, 'a') as arquivoEscrever:
        bloco_tamanho = 100000  # Tamanho do bloco de 100.000
        for i in range(inicio, fim + 1, bloco_tamanho):
            if encontrado_evento.is_set():
                return  # Interromper se o número já foi encontrado

            bloco_inicio = i
            bloco_fim = min(i + bloco_tamanho - 1, fim)
            
            if bloco_inicio <= nume <= bloco_fim:
                # Escrever apenas o bloco onde o número desejado está
                for j in range(bloco_inicio, bloco_fim + 1):
                    await arquivoEscrever.write(f"{j}\n")

                    if j == nume:
                        print(f"Número {nume} localizado na linha {contador_linhas + 1}")
                        encontrado_evento.set()  # Sinaliza que o número foi encontrado
                        break

                    # Atualizar o contador de linhas globalmente
                    async with contador_linhas_lock:
                        contador_linhas += 1

                if contador_linhas >= linha_limite:
                    buscar_numero_no_arquivo()
                    print(f"Limpando arquivo após {contador_linhas} linhas.")
                    await limpar_arquivo()
                    contador_linhas = 0
                break

def buscar_numero_no_arquivo():
    with open(arquivo_compactado, 'r') as arquivoLer:
        contador_linhas = 0
        for linha in arquivoLer:
            contador_linhas += 1
            numero_atual = int(linha.strip())
            if numero_atual == nume:
                print(f"Número {nume} encontrado na linha {contador_linhas}.")
                return True
        print(f"Número {nume} não encontrado após {contador_linhas} linhas.")
        return False

async def tarefa_gerar(nome, intervalo):
    inicio = nome * intervalo + 1
    fim = (nome + 1) * intervalo
    if inicio <= nume <= fim:
        await gerar_parte_arquivo(nome, inicio, fim)

async def main():
    await limpar_arquivo()
    intervalo = num_chaves // qt_tasks
    tarefas_gerar = [
        asyncio.create_task(tarefa_gerar(nome, intervalo))
        for nome in range(qt_tasks)
    ]
    await asyncio.gather(*tarefas_gerar)

    if encontrado_evento.is_set():
        print(f"Número {nume} encontrado.")
    else:
        print(f"Número {nume} não encontrado.")

asyncio.run(main())

end_time = time.time()
print(f"Tempo de execução total: {end_time - start_time} segundos")
