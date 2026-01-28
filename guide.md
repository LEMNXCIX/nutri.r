📘 NutriNode-Rust: Guía de Desarrollo de Agente IA Local
Esta guía te llevará desde cero hasta un producto funcional. Construiremos NutriNode, una aplicación de escritorio que gestiona tu nutrición OMAD/Keto usando Inteligencia Artificial local, persistencia en archivos y notificaciones por correo.

🧠 Introducción a las Tecnologías
Antes de escribir código, entendamos las piezas del rompecabezas:

Tauri v2: El marco de trabajo. A diferencia de Electron (que usa Node.js y Chromium pesado), Tauri usa las librerías nativas del sistema operativo (WebView2 en Windows, WebKit en Linux/Mac) y un backend en Rust. Resultado: Apps de 5MB en lugar de 100MB.

Rust: El lenguaje del backend. Nos da seguridad de memoria, tipado estricto y rendimiento extremo. Manejará los archivos, la IA y el envío de correos.

Leptos: El framework Frontend. Es como React, pero escrito en Rust y compilado a WebAssembly (WASM). No usa Virtual DOM, lo que lo hace increíblemente rápido y reactivo.

Ollama: El motor de IA local. Ejecuta modelos como Llama 3 o Mistral en tu NUC sin enviar datos a la nube.

🛠️ Fase 1: Configuración del Entorno
1. Prerrequisitos
Asegúrate de tener instalado:

Rust & Cargo: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Node.js / Bun: Para herramientas de empaquetado frontend.

Ollama: Instalado y ejecutándose (ollama run mistral-nemo).

2. Instalación de Herramientas de Desarrollo
Necesitamos el CLI de Tauri y Trunk (el empaquetador para WASM).

```
Bash
# Soporte para compilar a WebAssembly
rustup target add wasm32-unknown-unknown

# Herramienta de construcción para Leptos/WASM
cargo install trunk

# CLI de Tauri v2
cargo install tauri-cli --version "^2.0.0"
```

3. Inicialización del Proyecto
No usaremos plantillas por defecto para tener control total.

```
Bash
mkdir nutrinode
cd nutrinode
# Inicializar estructura básica de Rust
cargo init
# Inicializar Tauri en la carpeta
cargo tauri init
```

Prompt de Tauri:

Host: `trunk serve`

Url: `http://localhost:8080`

Frontend: Selecciona "No framework" (lo configuraremos manual con Leptos).

🏗️ Fase 2: Arquitectura y Estructura de Datos
Vamos a usar el patrón File-System Database. No necesitamos SQL para un solo usuario.

Estructura de Carpetas Recomendada
```
Plaintext
nutrinode/
├── src/                    # 🟢 Frontend (Leptos)
│   ├── components/         # Dashboard, Settings, PlanView
│   ├── app.rs              # Componente raíz
│   └── main.rs             # Punto de entrada WASM
├── src-tauri/              # 🦀 Backend (Rust)
│   ├── src/
│   │   ├── models.rs       # Estructuras de datos (JSON)
│   │   ├── ai.rs           # Lógica de Ollama (Double Pass)
│   │   ├── db.rs           # Lectura/Escritura de archivos
│   │   └── lib.rs          # Comandos expuestos
│   └── tauri.conf.json     # Configuración y Permisos
├── index.html              # Entry point para Trunk
└── Trunk.toml              # Configuración del servidor dev
````

Definición de Modelos (src-tauri/src/models.rs)
Concepto: serde (Serializer/Deserializer) es la biblioteca más importante de Rust. Convierte structs de Rust a JSON y viceversa automáticamente.

```
Rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub prompt_maestro: String,
    pub smtp_user: String,
    pub smtp_pass: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanIndex {
    pub id: String,         // Ej: "2026_W05"
    pub fecha: String,
    pub proteinas: Vec<String>, // Metadatos para búsquedas rápidas
    pub enviado: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanDetail {
    pub markdown_content: String,
    // Podrías agregar lista de compras parseada aquí
}
```

🦀 Fase 3: El Backend (Rust)
Aquí ocurre la magia. Vamos a editar src-tauri/Cargo.toml para añadir las dependencias:

```
Ini, TOML
[dependencies]
tauri = { version = "2.0.0", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] } // Cliente HTTP para Ollama
tokio = { version = "1", features = ["full"] }      // Runtime asíncrono
lettre = "0.10"                                     // Envio de emails
chrono = "0.4"                                      // Manejo de fechas
```

3.1. Módulo de Base de Datos (src-tauri/src/db.rs)
Implementamos funciones helpers para leer y escribir JSONs de forma segura.

```
Rust
use std::fs;
use std::path::PathBuf;
use crate::models::{AppConfig, PlanIndex};

pub fn get_data_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle.path().app_data_dir().expect("Error resolviendo AppData")
}

pub fn read_index(path: PathBuf) -> Vec<PlanIndex> {
    if !path.exists() { return vec![]; }
    let content = fs::read_to_string(path).unwrap_or("[]".to_string());
    serde_json::from_str(&content).unwrap_or_default()
}

// ... Implementar write_index y read_config similarmente
```

3.2. Lógica de IA - El "Double Pass" (src-tauri/src/ai.rs)
Concepto Avanzado: Los LLMs son buenos generando texto, pero malos generando estructuras estrictas. El Double Pass consiste en pedirle primero que sea creativo (texto) y luego, en una segunda llamada con un modelo más ligero, que extraiga datos (JSON/Lista).

```
Rust
use reqwest::Client;
use serde_json::json;

pub async fn generate_plan_with_ollama(prompt: String, exclusion: String) -> Result<(String, Vec<String>), String> {
    let client = Client::new();
    
    // PASO 1: Generación Creativa (Mistral Nemo 12B)
    let prompt_final = format!("{}\n\nRESTRICCIÓN: No uses estas recetas: {}", prompt, exclusion);
    
    let res_gen = client.post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "mistral-nemo",
            "prompt": prompt_final,
            "stream": false
        }))
        .send().await.map_err(|e| e.to_string())?;
    
    let json_gen: serde_json::Value = res_gen.json().await.map_err(|e| e.to_string())?;
    let markdown = json_gen["response"].as_str().unwrap_or("").to_string();

    // PASO 2: Extracción Analítica (Llama 3.2 3B o 1B)
    // Le pedimos solo las proteínas separadas por comas
    let prompt_extract = format!("Analiza el siguiente plan y lista solo las proteínas principales separadas por comas (ej: Pollo, Res): \n\n{}", markdown);
    
    let res_ext = client.post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "llama3.2", 
            "prompt": prompt_extract,
            "stream": false
        }))
        .send().await.map_err(|e| e.to_string())?;

    let json_ext: serde_json::Value = res_ext.json().await.map_err(|e| e.to_string())?;
    let raw_proteins = json_ext["response"].as_str().unwrap_or("");
    
    // Limpieza básica del string a Vec<String>
    let proteins: Vec<String> = raw_proteins.split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Ok((markdown, proteins))
}
```

3.3. Exponer Comandos a Tauri (src-tauri/src/lib.rs)
Estos son los puentes que el Frontend llamará.

```
Rust
#[tauri::command]
async fn cmd_generate_week(app: tauri::AppHandle) -> Result<String, String> {
    // 1. Leer Config y Index
    // 2. Construir lista de exclusión (últimas 3 semanas)
    // 3. Llamar a ai::generate_plan_with_ollama
    // 4. Guardar resultado en archivo y actualizar index
    // 5. Retornar "OK" o el ID generado
    Ok("2026_W0X".to_string())
}
```

🟢 Fase 4: El Frontend (Leptos + Tailwind)
4.1. Configuración (src/main.rs)
Configuramos Leptos para que se monte en el <body> del HTML.

```
Rust
use leptos::*;
mod app;

fn main() {
    mount_to_body(|| view! { <app::App/> })
}
```

4.2. El Puente (src/tauri_bridge.rs)
Concepto: wasm_bindgen permite a Rust (compilado a WASM) hablar con JavaScript. Tauri inyecta una API en JS, y aquí creamos los bindings para llamarla con seguridad de tipos desde Rust.

```
Rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Wrapper seguro
pub async fn generate_week() -> Result<String, String> {
    let res = invoke("cmd_generate_week", JsValue::NULL).await;
    // Lógica para convertir JsValue a Result de Rust...
    Ok("Done".to_string()) // Simplificado
}
```

4.3. UI Reactiva (src/app.rs)
Concepto: Leptos usa Signals. A diferencia de React que re-renderiza todo el componente, Leptos solo actualiza el nodo del DOM exacto que cambió. Es "Fine-Grained Reactivity".

```
Rust
use leptos::*;
use crate::tauri_bridge::generate_week;

#[component]
pub fn App() -> impl IntoView {
    // Estado
    let (loading, set_loading) = create_signal(false);
    let (plans, set_plans) = create_signal(vec![]); // Aquí cargarías el index

    // Acción
    let on_generate = move |_| {
        set_loading.set(true);
        spawn_local(async move {
            match generate_week().await {
                Ok(_) => log::info!("Plan generado!"),
                Err(e) => log::error!("Error: {}", e),
            }
            set_loading.set(false);
            // Recargar lista de planes aquí
        });
    };

    view! {
        <div class="min-h-screen bg-gray-900 text-white p-8">
            <h1 class="text-3xl font-bold mb-6 text-green-400">"NutriNode AI"</h1>
            
            <button 
                class="bg-green-600 hover:bg-green-500 px-4 py-2 rounded transition disabled:opacity-50"
                on:click=on_generate
                disabled=loading
            >
                {move || if loading.get() { "Generando..." } else { "Crear Nueva Semana" }}
            </button>

            // Aquí iría el componente <PlanList plans=plans />
        </div>
    }
}
```

🎨 Fase 5: Estilizado con Tailwind
Para que Tailwind funcione con Trunk:

Crea un archivo input.css:

```
CSS
@tailwind base;
@tailwind components;
@tailwind utilities;
```

En Trunk.toml:

```
Ini, TOML
[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "npx tailwindcss -i input.css -o dist/tailwind.css --minify"]
```

En index.html, vincula el CSS generado:

```
HTML
<link rel="stylesheet" href="/tailwind.css">
```

🚀 Fase 6: Ejecución y Construcción
Desarrollo (Hot Reload)
Abre dos terminales:

Frontend: `trunk serve` (Inicia el servidor de archivos y recompila WASM al guardar).

Tauri: `cargo tauri dev` (Abre la ventana nativa y conecta con el servidor de Trunk).

Producción (El Binario Final)
Cuando tu app esté lista y probada:

```
Bash
cargo tauri build
```

Esto generará un instalador (.msi o .exe en Windows, .deb en Linux) en src-tauri/target/release/bundle/. Este instalador contiene todo: tu lógica Rust, tu frontend compilado y optimizado.

🚀 Fase 7: Navegación y Detalles (Avanzado)
Para convertir nuestra app de una sola vista a una aplicación multipágina, usaremos `leptos_router`.

7.1 Configuración
Añade a `Cargo.toml` (frontend). Es crítico que las versiones de `leptos` y `leptos_router` coincidan (ej: `0.8.11`) para evitar errores de compatibilidad de traits:
```toml
leptos = { version = "0.8.11", features = ["csr"] }
leptos_router = { version = "0.8.11" }
```

En `src/app.rs`, importa y configura el Router. Note el uso de `path!` y el prop `fallback`:

```rust
use leptos_router::{components::{Route, Router, Routes}, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar />
            <main class="container mx-auto p-4">
                <Routes fallback=|| "Not Found">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/plan/:id") view=PlanDetail />
                </Routes>
            </main>
        </Router>
    }
}
```
> [!NOTE]
> **Leptos 0.8 Routing**: Ahora es obligatorio usar la macro `path!` para definir rutas y el componente `<Routes>` requiere un prop `fallback`.

7.2 Navbar
Un componente simple para navegar. Crea `src/components/navbar.rs`:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-gray-800 p-4 text-white flex gap-4 border-b border-gray-700">
            <A href="/" class="hover:text-green-400 transition">"Inicio"</A>
            <A href="/settings" class="hover:text-green-400 transition">"Configuración"</A>
        </nav>
    }
}
```

7.3 Vista de Detalle
Para ver el contenido de un plan, creamos una ruta dinámica `/plan/:id`.

1. **El Link**: En tu lista de planes en `Home`, usa el componente `<A>`:
```rust
<ul class="space-y-2">
    {move || plans.get().into_iter().map(|plan| view! {
        <A href=format!("/plan/{}", plan.id)>
            <li class="cursor-pointer hover:bg-gray-800 p-3 rounded border border-gray-700 hover:border-green-500 transition">
                {format!("📅 {} (ID: {})", plan.fecha, plan.id)}
            </li>
        </A>
    }).collect_view()}
</ul>
```

2. **El Componente de Detalle**:
Usamos `use_params_map` para leer el ID de la URL y un `LocalResource` para cargar los datos async.

```rust
use leptos_router::hooks::use_params_map;

#[component]
pub fn PlanDetail() -> impl IntoView {
    let params = use_params_map();
    // Signal derivada que obtiene el ID de la URL reactivamente
    let id = move || params.with(|params| params.get("id").unwrap_or_default());

    // LocalResource es necesario para Tauri porque el bridge de JS no es 'Send'
    let plan_resource = LocalResource::new(move || {
        let id = id(); // Trackeamos el ID reactivo
        async move {
            crate::tauri_bridge::get_plan_content(&id).await
        }
    });

    view! {
        <div class="p-6 max-w-4xl mx-auto">
            <div class="flex justify-between items-center mb-6">
                 <h2 class="text-2xl font-bold text-green-400">"Plan Semanal: " {id()}</h2>
                 <A href="/" class="bg-gray-700 hover:bg-gray-600 px-4 py-2 rounded text-sm transition">"← Volver"</A>
            </div>
           
            <Suspense fallback=move || view! { <div class="animate-pulse">"Cargando detalles..."</div> }>
                {move || match plan_resource.get() {
                    Some(Ok(content)) => view! { 
                        <article class="prose prose-invert max-w-none bg-gray-800 p-6 rounded-lg shadow-lg" inner_html=content></article> 
                    }.into_view(),
                    Some(Err(e)) => view! { 
                        <div class="text-red-400 bg-red-900/20 p-4 rounded">"Error cargando plan: " {e}</div> 
                    }.into_view(),
                    None => view! {}.into_view()
                }}
            </Suspense>
        </div>
    }
}
```

### 🧠 Explicación Técnica de las Correcciones

1.  **Alineación de Dependencias**: Leptos es un ecosistema de rápido movimiento. Si usas `leptos 0.8` con `leptos_router 0.6`, los "traits" internos (como los que permiten renderizar componentes) no coincidirán, lanzando errores crípticos. **Siempre mantén las versiones sincronizadas.**
2.  **`path!` Macro**: En la v0.8, las rutas han sido optimizadas. Ya no se usan strings simples sino la macro `path!`, que pre-procesa la ruta para un matching más rápido.
3.  **`LocalResource` vs `Resource`**: 
    - Un `Resource` estándar espera que el futuro sea `Send` (que pueda moverse entre hilos).
    - Tauri usa `wasm-bindgen` para hablar con JavaScript. Los objetos de JS (`JsValue`) **no son `Send`**.
    - `LocalResource` garantiza que la tarea asíncrona se ejecute siempre en el hilo principal del navegador/WebView, permitiendo usar las APIs de Tauri/JS de forma segura.
4.  **`params.with` y `.get()`**: En Leptos 0.8, se prefiere el acceso por referencia para evitar clones innecesarios, y las señales se invocan como funciones `id()` para obtener su valor.

¡Con esto tienes una SPA (Single Page Application) completa corriendo localmente!

