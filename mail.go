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
		log.Fatal("Error loading .env file")
	}
}

func SendMail(to, name, as string) {
	MJ_KEY := os.Getenv("MJ_API_KEY")
	MJ_SECRET := os.Getenv("MJ_API_SECRET")

	// Cria o cliente MailJet
	mj := mailjet.NewMailjetClient(MJ_KEY, MJ_SECRET)

	messagesInfo := []mailjet.InfoMessagesV31{
		{
			From: &mailjet.RecipientV31{
				Email: "asecreto.go@gmail.com",
				Name:  "Francisco",
			},
			To: &mailjet.RecipientsV31{
				mailjet.RecipientV31{
					Email: to,
					Name:  name,
				},
			},
			Subject:  "Amigo Secreto",
			HTMLPart: fmt.Sprintf("<h2>Amigo Secreto</h2><br><h3>O seu amigo secreto é:<br>	-> %v 🎉🎉🎉</h3>", as),
		},
	}
	messages := mailjet.MessagesV31{Info: messagesInfo}
	res, err := mj.SendMailV31(&messages)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Data: %+v\n", res)
}
