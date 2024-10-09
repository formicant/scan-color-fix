#! /bin/bash

mode="${1:-Gray}"
dpi="${2:-600}"

bus=$(lsusb | grep -Po '(?<=Bus )\d{3}(?= Device \d{3}: ID [0-9a-f:]+ Pantum Ltd\. M6500W-series)')

device=$(lsusb | grep -Po '(?<=Bus \d{3} Device )\d{3}(?=: ID [0-9a-f:]+ Pantum Ltd\. M6500W-series)')

scanner="pantum6500:libusb:$bus:$device"
echo "Scanner: $scanner"

print_option() {
  option=" [$1] $2 "
  if [ "$3" == "$2" ]
  then
    option="\e[7m$option\e[27m" # inverse
  fi
  echo -en " $option "
}

print_options() {
  if [ "$1" == true ]
  then
    echo -en "\e[3A" # move up 3 lines
  fi
  echo -n "   mode: "
  print_option "G" "Gray" "$mode"
  print_option "C" "Color" "$mode"
  echo
  echo -n "    dpi: "
  print_option "3" "300" "$dpi"
  print_option "6" "600" "$dpi"
  print_option "0" "1200" "$dpi"
  echo
  echo "           [Q] Quit   [space|enter] Scan"
}

scan_image() {
  now=$(date +"%y%m%d-%H%M%S")
  
  if [ "$mode" == "Color" ]
  then
    tempfile=$(mktemp /tmp/scan-color-fix-XXXX.png)
    scanimage -d "$scanner" --resolution=$dpi --mode=$mode --format=png -p -o "$tempfile"
    scan-color-fix "$tempfile" scan-$now.png
    rm "$tempfile"
  else
    scanimage -d "$scanner" --resolution=$dpi --mode=$mode --format=png -p -o scan-$now.png
  fi
  
  echo -en "\r\e[2K" # clear current line with progress
  for i in {1..5} # clear 5 lines Pantum driver writes
  do
    echo -en "\e[1A\e[2K" # move up and clear line
  done
}

print_options false

while true
do
  read -rsn1 input
  case $input in
    "q" | "Q")
      break ;;
    "g" | "G")
      mode="Gray";;
    "c" | "C")
      mode="Color" ;;
    "3")
      dpi="300" ;;
    "6")
      dpi="600" ;;
    "0")
      dpi="1200" ;;
    "")
      scan_image ;;
  esac
  print_options true
  
done
