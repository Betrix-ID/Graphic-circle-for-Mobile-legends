#!/system/bin/sh
# Checking ID shell
if [ "$(id -u)" -ne 0 ] && [ "$(id -u)" -ne 2000 ]; then
	printf "[ Eror |@Yeye_nat(Yeye)]\n"
   exit 1
fi
#Chking cpu.abi
     if [ ! -f /sdcard/Multi/lib/target ]; then
	    architecture=$(getprop ro.product.cpu.abi)
	  if [ "$architecture" = "arm64-v8a" ]; then
		cp /sdcard/Multi/lib/target/release/arm64 /sdcard/Multi/systlc
  fi
    fi
#smart notifications
shell() {
    sor="$1"
    cmd notification post -I /sdcard/Multi/IFS.png -S bigtext -t '♨️ Unlimited Graphic Mobile legends' 'Tag' "$sor" > /dev/null 2>&1
}
	
# Style display Terminal
set +x
    echo
    echo "     ☆================================☆"
    echo
    echo "       ~ Description. Graphic circle...... "
    echo
    echo "       - Author                 :  @UnixeID"
    echo "       - Point                    :  2.0 "
    echo "       - Release               :  22 - Mei - 2025"
    echo "       - Name Shell         :  Graphic circle"
    echo
    echo "    |_______________________________________|"
    echo "    \______________________________________/"
    echo
    echo "    Graphic circle Custem. "
    echo
    sleep 2
     rm -rf /data/local/tmp/systlc
     cp /sdcard/Multi/systlc /data/local/tmp
     chmod +x /data/local/tmp/systlc
     if [ "$1" = "-O" ]; then
          shell "Applying Graphic circle 1-2 seconds..."
          /data/local/tmp/systlc -O
     elif [ "$1" = "-L" ]; then
          shell "Applying Graphic circle 1-2 seconds..."
          /data/local/tmp/systlc -L
     elif [ "$1" = "-Q" ]; then
          shell "Applying Graphic circle 1-2 seconds..."
          /data/local/tmp/systlc -Q
     elif [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
           /data/local/tmp/systlc --help
        else
          printf "Failed to apply requested profile. Unknown option: %s\n" "$1"
      fi
set +x

