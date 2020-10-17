FROM rustlang/rust:nightly-slim as builder

# mkdir required to install openjdk on slim images
RUN mkdir -p /usr/share/man/man1 && \
    apt-get update && \
    apt-get install -y \
        cmake g++ git libpcre3-dev libpq-dev libssl-dev \
        libzmq3-dev make openjdk-11-jdk pkg-config unzip wget && \
    apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /tmp

RUN wget https://freefr.dl.sourceforge.net/project/swig/swig/swig-4.0.1/swig-4.0.1.tar.gz && \
    tar xf swig-*.tar.gz && \
    cd swig-* && \
    ./configure && make -j12 && \
    make install && \
    rm -rf /tmp/*

ENV ANDROID_SDK_ROOT=/root/sdk
ENV NDK_HOME=$ANDROID_SDK_ROOT/ndk/20.1.5948944

RUN wget https://dl.google.com/android/repository/commandlinetools-linux-6609375_latest.zip && \
    unzip commandlinetools-linux-*_latest.zip && \
    mkdir $ANDROID_SDK_ROOT && \
    mv tools $ANDROID_SDK_ROOT/ && \
    rm -rf /tmp/*

RUN yes | $ANDROID_SDK_ROOT/tools/bin/sdkmanager --sdk_root=$ANDROID_SDK_ROOT \
        "platform-tools" \
        "build-tools;29.0.3" \
        "platforms;android-29" \
        "ndk;20.1.5948944" \
        "cmake;3.10.2.4988404" \
        "extras;google;m2repository" \
        "extras;android;m2repository"

RUN echo '[target.aarch64-linux-android]\n\
ar = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"\n\
linker = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"\n\n\
[target.x86_64-linux-android]\n\
ar = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar"\n\
linker = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android26-clang"\n\n\
[target.armv7-linux-androideabi]\n\
ar = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar"\n\
linker = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang"\n\n\
[target.i686-linux-android]\n\
ar = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar"\n\
linker = "/root/sdk/ndk/20.1.5948944/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang"' > $CARGO_HOME/config

RUN rustup target add \
        aarch64-linux-android \
        x86_64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android

COPY ffi /rgb-node/ffi
COPY Cargo.lock Cargo.toml LICENSE README.md /rgb-node/
COPY src /rgb-node/src

ENV JAVA_HOME="/usr/lib/jvm/java-11-openjdk-amd64/"

WORKDIR /rgb-node/ffi/android/library

RUN ./gradlew build

