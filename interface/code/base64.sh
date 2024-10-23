#!/bin/sh
base64 -d ./updater/files_to_use/util_sig.txt > ./updater/files_to_use/util_sig_conv.txt
base64 -d ./updater/files_to_use/vendor_sig.txt > ./updater/files_to_use/vendor_sig_conv.txt
base64 -d ./updater/files_to_use/update64.txt > ./updater/files_to_use/update_bin.ino.hex
