package main

import (
	"fmt"
	"log"
	"os"

	"strings"

	"github.com/joho/godotenv"
	"github.com/mailjet/mailjet-apiv3-go"

	"crypto/md5"
)

func init() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Erro carregando .env")
	}
}

func SendMail(to, nome, as string, seed int64) {
	MJ_KEY := os.Getenv("MJ_API_KEY")
	MJ_SECRET := os.Getenv("MJ_API_SECRET")

	// Cria o cliente MailJet
	mj := mailjet.NewMailjetClient(MJ_KEY, MJ_SECRET)

	messagesInfo := []mailjet.InfoMessagesV31{
		{
			From: &mailjet.RecipientV31{
				Email: "amgsecreto.go@outlook.com",
				Name:  "Amigo Secreto",
			},
			To: &mailjet.RecipientsV31{
				mailjet.RecipientV31{
					Email: to,
					Name:  nome,
				},
			},
			Subject:  "Amigo Secreto",
			HTMLPart: fmt.Sprintf("<h2>%v, seu Amigo Secreto foi sorteado!</h2><h3>E elu é (rufem os tambores):<br><img src=\"https://cdn.statically.io/og/🎉%v🎉.jpg alt=\"%v\" height=200px width=350px></h3><br><br><p><a href=\"https://github.com/franpessoa/amigo-secreto\">Código Fonte</a>  ||  Seed: <strong>%d</strong>", nome, strings.Replace(as, " ", "%20", 1), as, seed),
		},
	}
	messages := mailjet.MessagesV31{Info: messagesInfo}
	res, err := mj.SendMailV31(&messages)

	if err != nil {
		log.Println(fmt.Sprintf("%v\n", res))
		log.Fatal(err)
	}

	fmt.Println("[EMAIL] Email disparado para", fmt.Sprintf("%x", md5.Sum([]byte(to))))
}
