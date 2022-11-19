package main

import (
	"fmt"
	"math/rand"
	"time"
	"encoding/csv"
	"os"
	"log"
)

type Pessoa struct {
	Nome string
	Mail string
}

func init() {
	rand.Seed(time.Now().UnixNano())
}

func main() {
	fmt.Println("Amigo Secreto");

	// Ler o arquivo
	file := "Teste.csv"

	content, err := os.ReadFile(file)

	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(string(content))
}
