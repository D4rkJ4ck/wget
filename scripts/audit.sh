#!/bin/bash

check_download() {
  if [ -f "$1" ]; then
    echo -e "File downloaded successfully: $1\n"
  else
    echo "File download failed."
    cd ../..
    exit 1
  fi
}

cargo b

cd target/debug || exit 1

# Test 1
echo "COMAND: ./wget https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg"
./wget https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
check_download EMtmPFLWkAA8CIS.jpg

# Test 2
read -rp "Enter URL to download: " DOWNLOAD_URL

echo "COMAND: ./wget $DOWNLOAD_URL"
./wget "$DOWNLOAD_URL"
check_download "$(basename "$DOWNLOAD_URL")"

# Test 3
echo "COMAND: ./wget https://golang.org/dl/go1.16.3.linux-amd64.tar.gz"
./wget https://golang.org/dl/go1.16.3.linux-amd64.tar.gz
check_download go1.16.3.linux-amd64.tar.gz

# Test 4
echo "COMAND: ./wget https://assets.01-edu.org/wgetDataSamples/Sample.zip"
./wget https://assets.01-edu.org/wgetDataSamples/Sample.zip
check_download Sample.zip

# Test 5
echo "COMAND: ./wget -O=test_20MB.zip https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget -O=test_20MB.zip https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download test_20MB.zip

# Test 6
echo "COMAND: ./wget -O=test_20MB.zip -P=~/Downloads/ https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget -O=test_20MB.zip -P=~/Downloads/ https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download test_20MB.zip

# Test 7
echo "COMAND: ./wget --rate-limit=300k https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget --rate-limit=300k https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download 20MB.zip

# Test 8
echo "COMAND: ./wget --rate-limit=700k https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget --rate-limit=700k https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download 20MB.zip

# Test 9
echo "COMAND: ./wget --rate-limit=2M https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget --rate-limit=2M https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download 20MB.zip

# Test 10
if [ ! -f "download.txt" ]; then
  touch download.txt
fi

{
echo https://assets.01-edu.org/wgetDataSamples/Image_20MB.zip
echo https://assets.01-edu.org/wgetDataSamples/20MB.zip
echo https://assets.01-edu.org/wgetDataSamples/Image_10MB.zip
} > download.txt

echo "COMAND: ./wget -i=download.txt"
./wget -i=download.txt
check_download Image_20MB.zip
check_download 20MB.zip
check_download Image_10MB.zip

# Test 11
echo "COMAND: ./wget -B https://assets.01-edu.org/wgetDataSamples/20MB.zip"
./wget -B https://assets.01-edu.org/wgetDataSamples/20MB.zip
check_download 20MB.zip
echo "COMMAND: cat wget-log"
cat wget-log

# Test 12
echo "COMAND: ./wget --mirror --convert-links http://corndog.io/"
./wget --mirror --convert-links http://corndog.io/

echo "COMMAND: ls corndog.io"
ls corndog.io
echo

# Test 13
echo "COMAND: ./wget --mirror https://oct82.com/"
./wget --mirror https://oct82.com/

echo "COMMAND: ls oct82.com"
ls oct82.com
echo

# Test 14
echo "COMAND: ./wget --mirror --reject=gif https://oct82.com/"
./wget --mirror --reject=gif https://oct82.com/

echo "COMMAND: ls oct82.com"
ls oct82.com
echo

# Test 15
echo "COMAND: ./wget --mirror https://tryapp.com/"
./wget --mirror https://tryapp.com/

echo "COMMAND: ls tryapp.com"
ls tryapp.com
echo

# Test 16
echo "COMAND: ./wget --mirror -X=/img https://tryapp.com/"
./wget --mirror -X=/img https://tryapp.com/

echo "COMMAND: ls tryapp.com"
ls tryapp.com
echo

# Test 17
echo "COMAND: ./wget --mirror https://theuselessweb.com/"
./wget --mirror https://theuselessweb.com/
echo

# Test 18
read -rp "Enter URL to download: " DOWNLOAD_URL

echo "COMAND: ./wget --mirror $DOWNLOAD_URL"
./wget --mirror "$DOWNLOAD_URL"
open "$DOWNLOAD_URL"/index.html &
