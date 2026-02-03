FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Evitar prompts interactivos
ENV DEBIAN_FRONTEND=noninteractive

# Instalar Node.js y JDK
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs openjdk-17-jdk-headless wget unzip \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Configuración de Android SDK
ENV ANDROID_HOME=/opt/android-sdk
ENV PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/build-tools/34.0.0

RUN mkdir -p $ANDROID_HOME/cmdline-tools \
    && wget -q https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip -O /tmp/tools.zip \
    && unzip -q /tmp/tools.zip -d $ANDROID_HOME/cmdline-tools \
    && mv $ANDROID_HOME/cmdline-tools/cmdline-tools $ANDROID_HOME/cmdline-tools/latest \
    && rm /tmp/tools.zip

# Aceptar licencias e instalar componentes necesarios
# Instalamos la plataforma 34 y build-tools 34 que son estándar para Tauri 2 actualmente
RUN yes | sdkmanager --licenses \
    && sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0" "ndk;25.2.9519653"

# Instalar tauri-cli globalmente para facilitar comandos dentro del contenedor
RUN cargo install tauri-cli --version "^2.0.0"

# Exponer puertos para tauri dev (1420)
EXPOSE 1420
