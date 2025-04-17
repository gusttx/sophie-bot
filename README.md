# 🤖 Sophie Bot

Um bot multifuncional para Discord escrito em [Rust](https://www.rust-lang.org/), utilizando a crate [Serenity](https://crates.io/crates/serenity) com seu framework [Poise](https://crates.io/crates/poise).

## ⚠️ Aviso sobre o comando `/onlinefix`

O comando `/onlinefix` obtém links magnéticos de torrents por meio de scraping do site `online-fix.me`, conhecido por distribuir conteúdo pirateado.

**Eu, o desenvolvedor deste bot, não apoio nem incentivo a pirataria de software.** A inclusão desta funcionalidade tem propósitos **estritamente educacionais**, como aprendizado de técnicas de web scraping, caching e manipulação de arquivos `.torrent`.

**O uso deste comando é de sua inteira responsabilidade.** Ao habilitar a feature `onlinefix` e utilizar este comando, você assume os riscos associados ao acesso e download de conteúdo de fontes não oficiais, que podem incluir malware ou violações de direitos autorais.

Este comando viola as [Políticas de Direitos Autorais e Propriedade Intelectual do Discord](https://support.discord.com/hc/en-us/articles/4410339349655-Discord-s-Copyright-IP-Policy).

Para utilizá-lo, a feature `onlinefix` deve ser explicitamente ativada durante a compilação.

## 🛠️ Compilação e Configuração

### Pré-requisitos

* Rust
* Docker
* Docker Compose

### Passos

1.  **Clone o repositório:**
    ```bash
    git clone https://github.com/gusttx/sophie-bot.git
    cd sophie-bot
    ```

2.  **Configure as variáveis de ambiente:**
    Copie o arquivo `.env.example` para `.env`:
    ```bash
    cp .env.example .env
    ```
    Edite o arquivo `.env` e preencha **todas** as variáveis com seus próprios valores.

3.  **Inicie os serviços dependentes (MySQL e Redis) com Docker Compose:**
    ```bash
    docker-compose up -d
    ```
    Isso iniciará os containers do MySQL e Redis em background, usando as senhas e portas definidas no seu arquivo `.env`.

4.  **Compile o bot:**

    * **Sem a feature `onlinefix`:**
        ```bash
        cargo build --release
        ```
    * **Com a feature `onlinefix`:**
        ```bash
        cargo build --release --features onlinefix
        ```

5.  **Configure o bot**
    
    Você pode configurar o bot em `config.toml`. Atualmente, ele só registra comandos nos servidores listados no campo `guild_ids`.

6.  **Execute o bot:**
    ```bash
    ./target/release/sophie-bot
    ```

## 📄 Licença

Este projeto é distribuído sob os termos da Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.