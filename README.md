# 🦀 QFerris — быстрый и легковесный API-клиент на Rust

[![License: MIT](https://img.shields.io/badge/license-Apache%202-yellow)](https://opensource.org/license/apache-2-0)

**QFerris** — это современный OpenSource-аналог Postman/Insomnia, созданный на Rust для разработчиков, которые ценят **производительность, минимализм и контроль**.  

Почему QFerris?  
- 🚀 **В 3-5 раз меньше RAM**, чем у Postman (благодаря Rust и Tauri).  
- 🔌 Поддержка **HTTP, GRPC, GraphQL** (и WebSocket в планах).  
- 🛠️ **Плагины на WASM** — расширяйте функционал без ущерба безопасности.  
- 📁 **Локальное хранение** (SQLite) + Git-интеграция.  
- 🦀 **Написан на Rust** — никакого Electron!   

## ⚡ Быстрый старт  
Установка (требуется Rust ≥1.70):  
```bash
git clone https://github.com/just-squad/qferris
cd qferris
cargo run --release
```

## 🌟 Особенности  
- **Интуитивный GUI** с тёмной/светлой темой.  
- **Импорт из Postman/Insomnia** (в разработке).  
- **Переменные окружения** (`{{base_url}}/{{token}}`).  
- **GRPC через .proto** (автогенерация кода).  

## 🛠️ Технологии  
- **GUI**: `tauri` + `Leptos`
- **HTTP**: `reqwest`  
- **GRPC**: `tonic` + `prost`  
- **База данных**: `sqlx` + SQLite  
- **Плагины**: `wasmtime`  

## 🚧 Roadmap  
- [x] Базовый HTTP-клиент (MVP)  
- [ ] GRPC-поддержка  
- [ ] Система плагинов (WASM)  
- [ ] Облачная синхронизация  

Полный план: [ROADMAP.md](ROADMAP.md)  

## 🤝 Как помочь проекту  
Мы приветствуем контрибьютеров! Вот как можно присоединиться:  
1. **Тестирование**: Попробуйте собрать FerrisQ и [сообщите о багах](https://github.com/just-squad/qferris/issues).  
2. **Разработка**: Возьмите задачу из [Good First Issues](https://github.com/just-squad/qferris/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).  
3. **Документация**: Помогите улучшить docs или перевести README.  

## 📜 Лицензия  
MIT © 2025 [JustSquad]  

---  
*QFerris не аффилирован с Postman или Insomnia. Это community-driven проект.*  
