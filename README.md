# Proyecto Cliente-Servidor TCP en Rust

Este es un proyecto de red básico desarrollado en Rust, que implementa un sistema cliente-servidor utilizando el protocolo TCP. El servidor escucha conexiones en un puerto específico y los clientes pueden conectarse para enviar mensajes.

## ✨ Objetivo

Aprender y practicar el uso de **sockets TCP** en Rust, entendiendo cómo funcionan las conexiones entre cliente y servidor a bajo nivel.

## 🚀 Características

- Servidor TCP que escucha en `x puerto`
- Acepta múltiples conexiones entrantes
- El cliente puede conectarse al servidor y enviar mensajes
- Comunicación punto a punto (peer-to-peer simple)
- Código modular (posibilidad de separar cliente y servidor en archivos distintos)

## 📂 Estructura del proyecto

```bash
tcp_chat/
├── Cargo.toml
├── src/
│   ├── main.rs        # Punto de entrada principal
│   ├── server.rs      # Lógica del servidor (puede estar incluida o separada)
│   └── client.rs      # Lógica del cliente (opcional)
