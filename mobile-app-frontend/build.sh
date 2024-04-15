rm ./dist/vw-app-*.wasm
rm ./build/mobile-android.7z
~/.cargo/bin/trunk build --release
# ~/.cargo/bin/trunk build
# ~/.cargo/bin/wasm-opt vw-app-*.wasm -Oz -o vw-opt.wasm
sed -i "s@import init from '/@import init from '\./@g" "./dist/index.html"
sed -i 's@<link rel="preload" href="\/vw-app-@<link rel="preload" href="vw-app-@g' "./dist/index.html"
sed -i 's@<link rel="modulepreload" href="\/vw-app-@<link rel="modulepreload" href="vw-app-@g' "./dist/index.html"
sed -i "s@init('\/@init('@g" "./dist/index.html"
sed -i 's@src="\/@src="@g' "./dist/index.html"
sed -i 's@href="\/static\/@href="static\/@g' "./dist/index.html"

# 7z a "./backend/public/downloads/mobile_android.7z" -up1q0r2x1y2z1w2 "../frontend/dist/*" -t7z -ms=off -mx7
7z a "./build/mobile-android.7z" "./dist/*" -t7z -mx7
sshpass -f "./.server-password" scp ./build/mobile-android.7z root@195.112.108.82:/var/lib/docker/volumes/vw-mobile-download/_data
cp "./build/mobile-android.7z" "../backend-new/public/download/mobile-android.7z"