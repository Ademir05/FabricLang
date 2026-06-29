# 🚀 Fabric Language

**Fabric** es un lenguaje de programación diseñado en Rust, con sintaxis configurable a través de archivos TOML y un sistema de tipos dinámico fuerte. Este proyecto incluye un Lexer, Parser e Intérprete capaces de manejar operaciones complejas, estructuras de control y tipos de datos numéricos avanzados.

---

## 🛠️ Requisitos previos: Instalar Rust en Windows

Para ejecutar Fabric, necesitas instalar el ecosistema de Rust en tu sistema Windows:

1. **Descargar Rustup**: Ve a [rustup.rs](https://rustup.rs/) y descarga el instalador `rustup-init.exe`.
2. **Visual Studio Build Tools**: Rust requiere las herramientas de compilación de C++. Si el instalador lo solicita, selecciona "Instalar herramientas de compilación de C++ de Visual Studio" (usualmente el paquete "Desktop development with C++").
3. **Configuración**:
   * Ejecuta el instalador y presiona `1` para la instalación por defecto.
   * Reinicia tu terminal (PowerShell o CMD) y escribe:
     ```powershell
     rustc --version
     ```

---

## ⚙️ Instalación del Proyecto

1. Abre una terminal en la carpeta raíz donde se encuentra el proyecto.
2. Compila el código para generar el ejecutable optimizado:
   ```powershell
   cargo build --release

