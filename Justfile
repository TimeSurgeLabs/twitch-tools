set dotenv-load
MODEL_URL := "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx"
CONFIG_URL := "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json"
ESPEAKNG_DATA_URL := "https://github.com/thewh1teagle/piper-rs/releases/download/espeak-ng-files/espeak-ng-data.tar.gz"

default:
  just --list

build: _fetch-resources
  npm run tauri build


build-mac-universal:_fetch-resources
  npm run tauri build --target universal-apple-darwin



dev: _fetch-resources
  npm run tauri dev

test:
  echo $APPLE_SIGNING_IDENTITY

_fetch-resources:
  #!/bin/bash
  # check if resources directory exists
  if [ ! -d "resources" ]; then
    echo "Resources directory not found, creating it"
    mkdir -p resources
  fi

  # check if the resources exist. If they do exist, skip the download
  if [ ! -f "resources/model.onnx" ]; then
    echo "Model file not found, downloading it"
    curl -L -o resources/model.onnx {{MODEL_URL}}
  fi

  if [ ! -f "resources/model.onnx.json" ]; then
    echo "Config file not found, downloading it"
    curl -L -o resources/model.onnx.json {{CONFIG_URL}}
  fi

  if [ ! -d "resources/espeak-ng-data" ]; then
    echo "Espeak-ng data not found, downloading it"
    curl -L -o resources/espeak-ng-data.tar.gz {{ESPEAKNG_DATA_URL}}
    tar -xzf resources/espeak-ng-data.tar.gz -C resources
    rm -rf resources/espeak-ng-data.tar.gz
  fi

  
clean:
  rm -rf resources
  rm -rf src-tauri/target
  rm -rf dist
