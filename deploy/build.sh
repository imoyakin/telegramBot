ps aux | grep telegramBot | grep -v grep | awk '{print $2}' | xargs kill
nohup ./telegramBot > ~/ret.log 2>&1 &