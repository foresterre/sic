export NASM_MACOS='https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/macosx/nasm-2.15.05-macosx.zip'
export NASM_MAC_PATH='nasm-2.15.05'

echo "Downloading from: $NASM_MACOS"
echo "Files path (in zip): $NASM_MAC_PATH"

curl -o nasm.zip $NASM_MACOS && \
  unzip nasm.zip && \
  cd $NASM_MAC_PATH && \
  export PATH=`pwd`:$PATH && \
  echo $PATH && \
  nasm --version