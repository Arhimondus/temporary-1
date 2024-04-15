# rm ./dist-debug-pack/*.wasm
mkdir ./dist-debug-pack
cp -r ./dist/* ./dist-debug-pack
sed -i "s@import init from '/@import init from '\./@g" "./dist-debug-pack/index.html"
sed -i 's@<link rel="preload" href="\/vw-app-@<link rel="preload" href="vw-app-@g' "./dist-debug-pack/index.html"
sed -i 's@<link rel="modulepreload" href="\/vw-app-@<link rel="modulepreload" href="vw-app-@g' "./dist-debug-pack/index.html"
sed -i "s@init('\/@init('@g" "./dist-debug-pack/index.html"
sed -i 's@src="\/@src="@g' "./dist-debug-pack/index.html"
7z a "../backend-new/public/download/mobile-android.7z" "./dist-debug-pack/*" -t7z -mx7