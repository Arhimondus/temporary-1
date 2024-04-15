#!/bin/sh

echo "GCC $(gcc -v  2>&1 | grep -m 1 -Po "(?<=version ).*(?= \()" | tr -d '\n')"
echo "BFC $(sha1sum /opt/bf*/bfc | cut -c 1-5)"
echo "CRYSTAL $(crystal --version | grep -m 1 -Po "(?<=Crystal ).*(?= \[)")"
echo "RUST $(rustc -V | grep -m 1 -Po "(?<=rustc ).*(?= \()")"
echo "LDC $(ldc2 --version | grep -m 1 -Po "(?<=the LLVM D compiler \().*(?=\))")"
echo "DENO $(deno -V | grep -m 1 -Po "(?<=deno ).*")"
echo "V $(v -v | grep -m 1 -Po "(?<=V ).*(?= )")"
echo "JAVA $(java -version 2>&1 | grep -m 1 -Po "(?<=openjdk version \").*(?=\" )")"
echo "KOTLIN $(kotlinc -version 2>&1 | grep -m 1 -Po "(?<=kotlinc-jvm ).*(?= \(JRE)")"
echo "DOTNET $(dotnet --version)"