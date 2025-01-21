// Встановлюємо базову URL-адресу API для взаємодії з сервером
const apiBase = 'http://127.0.0.1:8080';

// Додаємо обробник події "click" для кнопки входу
document.getElementById('login')?.addEventListener('click', async () => {
    // Отримуємо значення полів username і password, очищуючи їх від зайвих пробілів
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value.trim();

    // Якщо хоча б одне поле порожнє, виводимо повідомлення та зупиняємо виконання
    if (!username || !password) {
        alert('Please fill in both fields.');
        return;
    }

    try {
        // Надсилаємо POST-запит на сервер для авторизації
        const response = await fetch(`${apiBase}/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' }, // Вказуємо формат даних
            body: JSON.stringify({ username, password }), // Передаємо дані користувача
        });

        // Отримуємо відповідь від сервера у форматі JSON
        const result = await response.json();

        // Якщо сервер повернув успіх, зберігаємо username в localStorage та перенаправляємо на чат
        if (result.success) {
            localStorage.setItem('username', username);
            window.location.href = 'chat.html';
        } else {
            // Інакше показуємо повідомлення про помилку
            alert(result.message);
        }
    } catch (error) {
        // Логування помилок та виведення повідомлення у разі збою запиту
        console.error('Login error:', error);
        alert('Failed to login. Please try again.');
    }
});

// Додаємо обробник події "click" для кнопки реєстрації
document.getElementById('register')?.addEventListener('click', async () => {
    // Отримуємо значення полів username і password
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value.trim();

    // Перевіряємо, чи всі поля заповнені
    if (!username || !password) {
        alert('Please fill in both fields.');
        return;
    }

    try {
        // Надсилаємо POST-запит на сервер для реєстрації
        const response = await fetch(`${apiBase}/register`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' }, // Вказуємо формат даних
            body: JSON.stringify({ username, password }), // Передаємо дані користувача
        });

        // Отримуємо відповідь від сервера
        const result = await response.json();
        alert(result.message); // Виводимо повідомлення від сервера
    } catch (error) {
        // Логування помилок та виведення повідомлення у разі збою запиту
        console.error('Register error:', error);
        alert('Failed to register. Please try again.');
    }
});

// Перевіряємо, чи користувач знаходиться на сторінці чату
if (window.location.pathname.endsWith('chat.html')) {
    // Зчитуємо ім'я користувача з localStorage
    const username = localStorage.getItem('username');

    // Якщо ім'я користувача не знайдено, перенаправляємо на сторінку входу
    if (!username) {
        window.location.href = 'index.html';
    }

    // Виводимо ім'я користувача на сторінці
    document.getElementById('username-display').textContent = `Logged in as: ${username}`;

    // Ініціалізуємо змінні для елементів DOM
    const messages = document.getElementById('messages');
    const input = document.getElementById('input');
    const ws = new WebSocket('ws://127.0.0.1:8080/ws'); // Створюємо WebSocket-з'єднання

    // Подія, що виконується при відкритті WebSocket-з'єднання
    ws.onopen = async () => {
        console.log('WebSocket connection established.');
        try {
            // Завантажуємо збережені повідомлення через API
            const response = await fetch(`${apiBase}/messages`);
            const savedMessages = await response.json();

            // Додаємо завантажені повідомлення до блоку повідомлень
            for (const [user, msg] of savedMessages) {
                const messageDiv = document.createElement('div');
                messageDiv.className = 'message';
                messageDiv.textContent = `${user}: ${msg}`;
                messages.appendChild(messageDiv);
            }

            // Прокручуємо список повідомлень до самого низу
            messages.scrollTop = messages.scrollHeight;
        } catch (error) {
            console.error('Failed to load messages:', error);
        }
    };

    // Логування при закритті WebSocket-з'єднання
    ws.onclose = () => console.log('WebSocket connection closed.');

    // Логування помилок WebSocket
    ws.onerror = (err) => console.error('WebSocket error:', err);

    // Обробка отриманих через WebSocket повідомлень
    ws.onmessage = (event) => {
        const messageDiv = document.createElement('div');
        messageDiv.className = 'message';
        messageDiv.textContent = event.data; // Додаємо нове повідомлення
        messages.appendChild(messageDiv);

        // Прокручуємо список повідомлень до самого низу
        messages.scrollTop = messages.scrollHeight;
    };

    // Обробка події натискання клавіші Enter для відправлення повідомлення
    input.addEventListener('keypress', (e) => {
        if (e.key === 'Enter' && input.value) {
            ws.send(`${username}: ${input.value}`); // Надсилаємо повідомлення через WebSocket
            input.value = ''; // Очищаємо поле введення
        }
    });

    // Обробка кнопки виходу з облікового запису
    document.getElementById('logout').addEventListener('click', () => {
        localStorage.removeItem('username'); // Видаляємо ім'я користувача з localStorage
        ws.close(); // Закриваємо WebSocket-з'єднання
        window.location.href = 'index.html'; // Повертаємося на сторінку входу
    });
}
