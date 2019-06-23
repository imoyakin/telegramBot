package main

import (
	"bot/handle"
	"log"
	"net/http"
	"os"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
)

func main() {
	BotAPI := os.Getenv("botAPI")
	addr := os.Getenv("botAddr")

	bot, err := tgbotapi.NewBotAPI(BotAPI)
	if err != nil {
		log.Fatal(err)
	}

	bot.Debug = true

	log.Printf("Authorized on account %s", bot.Self.UserName)
	_, err = bot.SetWebhook(tgbotapi.NewWebhook(addr + bot.Token))
	if err != nil {
		log.Fatal(err)
	}

	info, err := bot.GetWebhookInfo()
	if err != nil {
		log.Fatal(err)
	}

	if info.LastErrorDate != 0 {
		log.Printf("Telegram callback failed:%s", info.LastErrorMessage)
	}

	updates := bot.ListenForWebhook("/" + bot.Token)
	go http.ListenAndServe("0.0.0.0:8443", nil)
	//go http.ListenAndServeTLS("0.0.0.0:8443", "cert.pem", "key.pem", nil)

	go handle.HandleMsg(bot, updates)
	for update := range updates {
		log.Printf("%+v\n", update)
	}
}
