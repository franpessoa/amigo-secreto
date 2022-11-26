package main

import "fmt"

func Debug(pessoa Pessoa) {
	fmt.Printf("%v tirou %v\n", pessoa.Nome, pessoa.AmigoSecreto)
}
