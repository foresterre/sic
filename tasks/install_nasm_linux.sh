sudo apt install alien
sudo apt install curl
curl -o "nasm-2.15.05-0.fc31.x86_64.rpm" "https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/linux/nasm-2.15.05-0.fc31.x86_64.rpm"
sudo alien -i "nasm-2.15.05-0.fc31.x86_64.rpm"