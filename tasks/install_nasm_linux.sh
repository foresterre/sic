export NASM_LINUX='https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/linux/nasm-2.15.05-0.fc31.x86_64.rpm'
export NASM_RPM='nasm-2.15.05-0.fc31.x86_64.rpm'
sudo apt install alien && wget $NASM_LINUX && sudo alien -i $NASM_RPM