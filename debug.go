package main

import "fmt"

func Debug(pessoa Pessoa) {
	fmt.Printf("%v tirou %v", pessoa.Nome, pessoa.AmigoSecreto)
}
