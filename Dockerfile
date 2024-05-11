# Используем официальный образ Rust как базовый для сборки нашего приложения
FROM rust:1.56 as builder

# Создаем новую рабочую директорию для компиляции приложения
WORKDIR /usr/src/myapp

# Копируем файлы проекта в рабочую директорию
COPY . .

# Собираем зависимости, убедимся, что они закешированы
RUN cargo install --path .

# Настраиваем окончательный образ, используя минимальный официальный образ Debian
FROM debian:buster-slim

# Устанавливаем пакеты необходимые для работы стандартной библиотеки Rust
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Копируем исполняемый файл из сборочного контейнера в окончательный образ
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

# Устанавливаем команду, которая будет выполнена при запуске контейнера
CMD ["myapp"]
