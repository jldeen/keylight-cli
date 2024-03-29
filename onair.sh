#!/bin/bash
# https://apple.stackexchange.com/questions/424789/can-i-trigger-a-homekit-scene-when-my-laptops-camera-is-turned-on-or-off

exec log stream | grep -E --line-buffered 'UVCAssistant' | # filter for UVCAssistant so BT stream is ignored
grep -E --line-buffered '(stop|start) stream' | # filter log events
  # tee /dev/stderr |                           # output matching events for debugging
  sed -Eu 's/.*(start|stop).*/\1/' | # reduce the log message down to a single word identifying the event/state
  while read -r event; do                     # store that word in the $event variable
    echo "Camera state has changed to: $event"
    if [ "$event" = "start" ]; then
      # echo "Keylight on"
      keylight on
    else
      # echo "Keylight off"
      keylight off
    fi
  done