use warp::ws::{Message, WebSocket};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Структура для отримання даних про користувача під час реєстрації або входу
#[derive(Deserialize)]
pub struct User {
    username: String, // Ім'я користувача
    password: String, // Пароль
}

// Структура для повернення відповіді API
#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,   // Успішність операції
    message: String, // Повідомлення для користувача
}

// Обробник для реєстрації нового користувача
pub async fn handle_register(
    user: User, 
    users: Arc<RwLock<HashMap<String, String>>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = users.write().await;
    // Перевіряємо, чи вже існує користувач з таким ім'ям
    if users.contains_key(&user.username) {
        return Ok(warp::reply::json(&ApiResponse {
            success: false,
            message: "Username already exists".to_string(),
        }));
    }
    // Додаємо нового користувача
    users.insert(user.username.clone(), user.password);
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "Registration successful".to_string(),
    }))
}

// Обробник для авторизації користувача
pub async fn handle_login(
    user: User, 
    users: Arc<RwLock<HashMap<String, String>>>
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = users.read().await;
    // Перевіряємо наявність користувача та збіг пароля
    if let Some(password) = users.get(&user.username) {
        if password == &user.password {
            return Ok(warp::reply::json(&ApiResponse {
                success: true,
                message: "Login successful".to_string(),
            }));
        }
    }
    Ok(warp::reply::json(&ApiResponse {
        success: false,
        message: "Invalid username or password".to_string(),
    }))
}

// Обробник WebSocket-з'єднань для роботи з реальними повідомленнями
pub async fn handle_connection(
    ws: WebSocket, 
    tx: Arc<Mutex<broadcast::Sender<String>>>
) {
    // Розділяємо WebSocket-з'єднання на відправник і приймач
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let mut rx = tx.lock().await.subscribe(); // Підписуємося на канал передачі повідомлень

    // Запускаємо асинхронне завдання для передачі вхідних повідомлень WebSocket
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Відправляємо отримані повідомлення всім підписникам
            if ws_sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Обробляємо вхідні повідомлення від користувача
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_str() {
                    // Формуємо повідомлення у вигляді [Ім'я користувача]: текст
                    let username_message = format!("[{}] {}", "Username", text);
                    // Надсилаємо повідомлення через канал
                    tx.lock()
                        .await
                        .send(username_message)
                        .expect("Failed to broadcast message");
                }
            }
            Err(_) => break, // Завершуємо цикл при помилці
        }
    }
}
