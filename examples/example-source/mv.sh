./build.sh
rm -rf ./Payload ./en.example-source-unpacked
mv package.bunpak file.zip && unzip file.zip -d . && rm -f file.zip package.bunpak
cp -R ./Payload en.example-source-unpacked
rm -rf ./Payload
