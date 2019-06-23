package handle

import (
	"fmt"
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
)

//HandleMsg ch
func HandleMsg(telegram *tgbotapi.BotAPI, updatechan tgbotapi.UpdatesChannel) {
	for update := range updatechan {
		go func() {
			if message := update.Message; message != nil {
				fmt.println(message)
			}
		}()
	}
}
