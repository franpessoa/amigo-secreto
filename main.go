package main

import (
	"encoding/csv"
	"fmt"
	"io"
	"log"
	"math/rand"
	"os"
	"time"
)

type Pessoa struct {
	Nome         string
	Mail         string
	AmigoSecreto string
}

func readPessoas() []Pessoa {
	// Lista de pessoas
	pessoas := []Pessoa{}

	// Ler o arquivo
	f, err := os.Open("Lista.csv")

	if err != nil {
		log.Fatal(err)
	}

	// Fecha o arquivo no fim da função
	defer f.Close()

	// Lê o CSV
	reader := csv.NewReader(f)
	for {
		linha, err := reader.Read()

		// Verifica se o arquivo acabou
		if err == io.EOF {
			break
		}

		// Verfica se deu erro
		if err != nil {
			log.Fatal(err)
		}

		// Adiciona linha à lista
		pessoas = append(pessoas, Pessoa{linha[0], linha[1], ""})

		fmt.Println("[REGISTRO] Pessoa registrada")
	}

	fmt.Println("[REGISTRO] Lista fechada")

	return pessoas
}

// From:
// https://stackoverflow.com/questions/12264789/shuffle-array-in-go - go implementation
// https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle - algorithm

func shuffleList(lista []Pessoa) []Pessoa {
	i := len(lista) - 1
	for i > 0 {
		j := rand.Intn(i + 1)
		lista[i], lista[j] = lista[j], lista[i]

		fmt.Println("[SORTEIO] Swap de posições")
		i -= 1
	}

	fmt.Println("[SORTEIO] Lista aleatorizada")

	return lista
}

func main() {
	seed := time.Now().UnixNano()
	rand.Seed(seed)

	fmt.Println("[SORTEIO] Randomizador preparado")

	file, err := os.Create("seed.key")

	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()
	file.WriteString(fmt.Sprintf("%v", seed))

	fmt.Println("[SORTEIO] Chave registrada")

	lista := shuffleList(readPessoas())
	lista = shuffleList(lista)

	fmt.Println("[EMAIL] Envio iniciado")

	debug := true

	for i := range lista {
		if i < len(lista)-1 {
			lista[i].AmigoSecreto = lista[i+1].Nome
		} else {
			lista[i].AmigoSecreto = lista[0].Nome
		}

		if debug {
			Debug(lista[i])
		} else {
			SendMail(lista[i].Mail, lista[i].Nome, lista[i].AmigoSecreto, seed)
		}
	}

	fmt.Println("[EMAIL] Fim dos emails")
	fmt.Println("[EMAIL] Fim do programa")
}
