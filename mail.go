package main

import (
	"fmt"
	"log"
	"os"

	"github.com/joho/godotenv"
	"github.com/mailjet/mailjet-apiv3-go"
)

func init() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Erro carregando .env")
	}
}

func SendMail(to, name, as string, seed int64) {
	MJ_KEY := os.Getenv("MJ_API_KEY")
	MJ_SECRET := os.Getenv("MJ_API_SECRET")

	// Cria o cliente MailJet
	mj := mailjet.NewMailjetClient(MJ_KEY, MJ_SECRET)

	messagesInfo := []mailjet.InfoMessagesV31{
		{
			From: &mailjet.RecipientV31{
				Email: "asecreto.go@gmail.com",
				Name:  "🎉 Amigo Secreto 🎉",
			},
			To: &mailjet.RecipientsV31{
				mailjet.RecipientV31{
					Email: to,
					Name:  name,
				},
			},
			Subject:  "Amigo Secreto",
			HTMLPart: fmt.Sprintf("<h2>Amigo Secreto</h2><br><h3>O seu amigo secreto é:<br>	-> %v 🎉🎉🎉</h3><br><br><p><a href=\"https://github.com/franpessoa/amigo-secreto\">Código Fonte</a>  ||  Seed: <strong>%d</strong>", as, seed),
		},
	}
	messages := mailjet.MessagesV31{Info: messagesInfo}
	res, err := mj.SendMailV31(&messages)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Data: %+v\n", res)
}
