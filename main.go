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

func init() {
	seed := time.Now().UnixNano()
	file, err := os.Create("Seed.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()
	file.WriteString(fmt.Sprintf("Amigo Secreto v1\n Sorteado às %v\nChave %v", time.Now(), seed))

	rand.Seed(seed)
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
	}

	return pessoas
}

// From:
// https://stackoverflow.com/questions/12264789/shuffle-array-in-go - go implementation
// https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle - algorithm

func shuffleList(lista []Pessoa) []Pessoa {
	for i := range lista {
		j := rand.Intn(i + 1)
		lista[i], lista[j] = lista[j], lista[i]
	}

	return lista
}

func main() {
	fmt.Println("++ Amigo Secreto")
	lista := shuffleList(readPessoas())

	for i := range lista {
		if i < len(lista)-1 {
			lista[i].AmigoSecreto = lista[i+1].Nome
		} else {
			lista[i].AmigoSecreto = lista[0].Nome
		}

		SendMail(lista[i].Mail, lista[i].Nome, lista[i].AmigoSecreto)
	}

}
