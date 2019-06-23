#!/bin/bash
cd ~/telegramBot
export GOPATH=~/telegramBot
export GO111MODULE="on"
rm ./bin/bot
git pull
cd src
go install
cd ..
ps aux | grep bot | grep -v grep | awk '{print $2}' | xargs kill
nohup ./bin/bot > ~/bot.log 2>&1 &