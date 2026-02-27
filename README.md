# Nutri-R Ecosystem

Ecosistema híbrido de nutrición: Frontend (Leptos/WASM), Escritorio (Tauri), Android (Tauri Mobile) y Servidor de Automatización (Axum).

## 🚀 Desarrollo Local (Windows)

### 1. Requisitos
- **Rust** (Stable)
- **Node.js**
- **Trunk** (`cargo install trunk`)
- **Tauri CLI** (`cargo install tauri-cli`)

### 2. Levantar el Entorno de Escritorio
Si recibes el error `'.' is not recognized` al correr `cargo tauri dev`, es porque Windows no reconoce el prefijo `./`. 

**Comando Correcto:**
```powershell
# Primero instala dependencias de frontend
npm install

# Inicia Tauri (Asegúrate que trunk esté en tu PATH)
cargo tauri dev
```

> **Nota:** Si el error persiste, abre `src-tauri/nutri-app/tauri.conf.json` y cambia `beforeDevCommand` de `./trunk serve` a `trunk serve`.

### 3. Levantar el Servidor de Automatización (API + Cron)
Para que el cron se ejecute y la app de Android pueda sincronizar, debes iniciar el servidor en segundo plano o en otra terminal:

```powershell
cd src-tauri/nutri-server
cargo run
```
El servidor escuchará en `http://127.0.0.1:3001`.

## 📱 Android Build
### Requisitos
Asegúrate de tener instalado el **Android SDK** y **NDK**.

### Comandos de Compilación
```bash
# Debug APK (Instalable directamente para pruebas)
cargo tauri android build --apk --debug

# Release APK (Sin firmar)
cargo tauri android build --apk
```
**Ruta del APK:** `src-tauri/nutri-app/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk`

## 🛠️ Configuración del Cron
Puedes cambiar la frecuencia de generación de planes desde la configuración de la app o enviando un JSON a:
`GET/POST http://localhost:3001/api/config`

**Ejemplo de Cron:** `"0 0 0 * * MON"` (Todos los lunes a medianoche).
