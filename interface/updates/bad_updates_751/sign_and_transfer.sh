openssl dgst -sha3-256 -sign vendor.private.pem -out vendor_sign.txt updatecommands.txt
openssl dgst -sha3-256 -sign utility.private.pem -out util_sign.txt updatecommands.txt
base64 vendor_sign.txt > vendor_sign64.txt
base64 updatecommands.txt > update64.txt
base64 util_sign.txt > util_sign64.txt
sudo cp vendor_sign64.txt /local_updates/
sudo cp util_sign64.txt /local_updates/
sudo cp update64.txt /local_updates/
