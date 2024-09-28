cargo run &
APP_PID=$!

while inotifywait -r -e modify src/*.rs; do
  if ps -p $APP_PID > /dev/null; then
    kill $APP_PID
  fi

  cargo run &
  APP_PID=$!
done