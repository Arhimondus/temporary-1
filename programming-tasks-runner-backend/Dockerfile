ARG DEBIAN_FRONTEND=noninteractive

FROM debian:bookworm-slim

RUN apt-get update
RUN apt-get install -y git
RUN apt-get install -y wget
RUN apt-get install -y curl
RUN apt-get install -y unzip

# C++
RUN apt-get install -y build-essential

# Brainfuck
WORKDIR /opt
RUN git clone --depth 1 https://github.com/skeeto/bf-x86
WORKDIR /opt/bf-x86
RUN gcc -o bfc bf-x86.c

# Crystal
RUN curl -fsSL https://crystal-lang.org/install.sh | bash

# Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# D
RUN curl -fsS https://dlang.org/install.sh | bash -s ldc

# JavaScript/TypeScript
RUN curl -fsSL https://deno.land/install.sh | sh

# V
WORKDIR /opt
RUN git clone --depth 1 https://github.com/vlang/v
WORKDIR /opt/v
RUN make

# Java
WORKDIR /tmp
RUN wget https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz
RUN tar xvf openjdk-17.0.2_linux-x64_bin.tar.gz
RUN mv jdk-17*/ /opt/jdk17

# Kotlin
WORKDIR /tmp
RUN apt-get install -y unzip
RUN wget https://github.com/JetBrains/kotlin/releases/download/v1.6.20/kotlin-compiler-1.6.20.zip
RUN unzip kotlin-compiler*
RUN mv kotlinc /opt/kotlinc


# C#
RUN wget https://packages.microsoft.com/config/debian/10/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
RUN dpkg -i packages-microsoft-prod.deb
RUN rm packages-microsoft-prod.deb
RUN apt-get update
RUN apt-get install -y apt-transport-https
RUN apt-get update
RUN apt-get install -y dotnet-sdk-6.0

# Profile
COPY .profile /root/.profile
COPY show-versions.sh /root/show-versions.sh
RUN chmod +x /root/show-versions.sh
RUN ls -l /root/show-versions.sh
# RUN /root/show-versions.sh
COPY languages-linux.toml /var/app/languages-linux.toml
COPY target/release/lernu-runner /var/app/lernu-runner
RUN chmod +x /var/app/lernu-runner
RUN . /root/.profile

ENV JAVA_HOME="/opt/jdk17"
ENV DENO_INSTALL="/root/.deno"
ENV DMD="ldmd2"
ENV DC="ldc2"
ENV LIBRARY_PATH="/root/dlang/ldc-1.29.0/lib${LIBRARY_PATH:+:}${LIBRARY_PATH:-}"
ENV LD_LIBRARY_PATH="/root/dlang/ldc-1.29.0/lib${LD_LIBRARY_PATH:+:}${LD_LIBRARY_PATH:-}"
ENV PATH="/opt/colorized-logs:/opt/kotlinc/bin:/root/dlang/ldc-1.29.0/bin:/opt/bf-x86:/opt/v:$JAVA_HOME/bin:$DENO_INSTALL/bin:$HOME/.cargo/bin:$HOME/.deno/bin:$PATH"

WORKDIR /var/app
CMD ["/var/app/lernu-runner"]