# 🐕 Money Master

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)
![Status](https://img.shields.io/badge/status-active-success?style=for-the-badge)

**Um chatbot financeiro inteligente e open-source para controle financeiro pessoal via WhatsApp**

*Mascote: Um cachorro sorridente 🐕*

[Funcionalidades](#-funcionalidades) • [Arquitetura](#-arquitetura) • [Instalação](#-instalação) • [Configuração](#-configuração) • [Uso](#-uso) • [Produção](#-produção)

</div>

---

## 📋 Sobre o Projeto

O **Money Master** é uma solução de c[odigo aberto para controle financeiro pessoal que utiliza inteligência artificial para processar mensagens em linguagem natural através do WhatsApp. Desenvolvido em Rust com arquitetura de microsserviços, oferece uma experiência intuitiva e poderosa para gerenciar suas finanças.

### ✨ Destaques

- 🤖 **Processamento de Linguagem Natural**: Interaja com o chatbot usando linguagem natural
- 💰 **Controle Financeiro Completo**: Registre despesas e receitas de forma detalhada
- 🔍 **Busca Inteligente**: Encontre e analise seus registros financeiros com comandos simples
- 🔒 **Multi-tenant**: Suporta múltiplos usuários com bancos de dados isolados
- ⚡ **Alta Performance**: Desenvolvido em Rust para máxima eficiência
- 📊 **Relatórios e Análises**: Gere relatórios e análises dos seus gastos

---

## 🚀 Funcionalidades

### 1. Registro de Entradas (Receitas e Despesas)
- Registre múltiplas entradas em uma única mensagem
- Suporte a valores, descrições, categorias e observações
- Data automática (hoje) ou personalizada
- Processamento inteligente de linguagem natural

**Exemplo:**
```
Supermercado 133,35 Gasolina 200,00 Camisa nova 50,00 Presente para mãe 35,00 no último sábado para o aniversário dela
```

### 2. Busca e Listagem
- Busque entradas por data, período, categoria, valor ou descrição
- Calcule totais e somas
- Liste entradas com filtros avançados
- Gere relatórios e análises com poder da IA

**Exemplos:**
- "Liste minhas receitas desta semana"
- "Quanto gastei no mês passado?"
- "Mostre minhas despesas da categoria alimentação dos últimos 2 meses"

### 3. Exclusão de Entradas
- Exclua uma ou múltiplas entradas
- Busque e exclua usando descrições naturais

**Exemplo:**
```
Excluir presente da mãe do último sábado
Excluir entrada do supermercado de 133,35
```

### 4. Desfazer Operações
- Desfaça a última operação imediatamente após o registro
- Comando simples: "undo" ou "desfazer"

### 5. Ajuda e Informações
- Pergunte para ober informações sobre o sistema
- Dicas de uso e exemplos
- Guia de funcionalidades

---

## 🏗️ Arquitetura

O Money Master é construído com uma arquitetura de microsserviços modular:

```
moneymaster/
├── moma-api/          # API REST (Actix-Web) - Recebe webhooks do WhatsApp
├── moma-worker/       # Worker assíncrono - Processa mensagens via AMQP
├── moma-routine/      # Motor de rotinas - Processa comandos e rotinas
├── moma-core/         # Domínio principal - Entidades e lógica de negócio
├── moma-auth/         # Autenticação e multi-tenant
├── moma-integration/  # Integrações externas (WhatsApp, Email, IA, AMQP)
├── moma-shared/       # Código compartilhado e configurações
├── moma-log/          # Sistema de logs
└── moma-migrator/     # Migrações de banco de dados
```

### Stack Tecnológico

- **Linguagem**: Rust (Edition 2021)
- **Framework Web**: Actix-Web 4.8
- **ORM**: Sea-ORM 0.12 (SQLite)
- **Runtime Assíncrono**: Tokio
- **IA/ML**: OpenAI API
- **Message Broker**: AMQP (RabbitMQ)
- **Email**: Lettre
- **Serialização**: Serde

### Fluxo de Mensagens

1. **WhatsApp Webhook** → `moma-api` recebe a mensagem
2. **AMQP Queue** → Mensagem é enfileirada
3. **Worker** → `moma-worker` consome a mensagem
4. **Routine Selector** → IA seleciona a rotina apropriada
5. **Processamento** → Rotina processa o comando
6. **Resposta** → Envia resposta ao usuário

---

## 📦 Instalação

### Pré-requisitos

- **Rust**: Versão 1.70 ou superior
- **Cargo**: Gerenciador de pacotes do Rust
- **SQLite**: Banco de dados (gerenciado automaticamente)
- **Serviços Externos** (configuráveis):
  - OpenAI API (para processamento de IA)
  - WhatsApp API (WPP Connect)
  - AMQP/RabbitMQ (message broker)

### Clonando o Repositório

```bash
git clone https://github.com/seu-usuario/moneymaster.git
cd moneymaster
```

### Compilando o Projeto

```bash
# Build de desenvolvimento
cargo build

# Build de release (otimizado)
cargo build --release
```

---

## ⚙️ Configuração

### 1. Arquivo de Ambiente

Crie um arquivo `.env` na raiz do projeto com as seguintes variáveis:

```env
# OpenAI / IA
AI_AUTH_TOKEN=your_openai_api_key
AI_MODEL_ID=gpt-3.5-turbo

# WhatsApp
WHATSAPP_WPP_TOKEN=your_wpp_token
WHATSAPP_WPP_API_URL=https://api.wpp.com
WHATSAPP_WPP_SESSION=your_session

# AMQP (Rabbit MQ)
AMQP_URI=amqp://user:password@localhost:5672
AMQP_EXCHANGE_NAME=moma_exchange
AMQP_WHATSAPP_QUEUE_NAME=whatsapp_queue
AMQP_WHATSAPP_QUEUE_ROUTING_KEY=whatsapp.message

# Email (opcional)
EMAIL_AUTH_TOKEN=your_email_token
EMAIL_MAILSENDER_API_URL=https://api.email.com
EMAIL_FROM_ADDRESS=noreply@moneymaster.com
```

### 2. Configurando o Banco de Dados

O sistema cria automaticamente os bancos de dados SQLite na pasta `db/`:

- `db/auth.db` - Autenticação e tenants
- `db/routine.db` - Dados de execução das rotinas
- `db/core/{tenant_id}.db` - Dados de cada tenant

### 3. Executando Migrações

```bash
# Execute o migrador para criar as tabelas
cargo run --bin moma-migrator
```

---

## 🚀 Uso

### Iniciando os Serviços

#### 1. API (Webhook do WhatsApp)

Instale o servidor de whatsapp e ative uma sessão.
https://github.com/wppconnect-team/wppconnect-server

```bash
cd moma-api
cargo run
```

A API estará disponível em `http://0.0.0.0:8081`

**Endpoints:**
- `POST /WppWhatsApp` - Webhook do WhatsApp
- `GET /status` - Status da API

#### 2. Worker (Processamento de Mensagens)

```bash
cd moma-worker
cargo run
```

O worker ficará ativo processando mensagens da fila AMQP.

### Exemplos de Comandos no WhatsApp

```
👤 Usuário: Olá!
🤖 Money Master: Olá! Bem-vindo ao Money Master! Como posso ajudá-lo hoje?

👤 Usuário: Gastei 50 reais no supermercado hoje
🤖 Money Master: ✅ Entrada registrada: Supermercado - R$ 50,00

👤 Usuário: Quanto gastei este mês?
🤖 Money Master: 📊 Total de despesas deste mês: R$ 1.250,00

👤 Usuário: Liste minhas despesas da última semana e agrupe por categoria
🤖 Money Master: 📋 Despesas da última semana:
                • Supermercado - R$ 50,00
                • Gasolina - R$ 200,00
                • Restaurante - R$ 85,50
```

---

## 🚀 Produção
 Money master é um projeto de uso gratuito aberto a comunidade. Ficou curioso? Quer experimentar? Chame Money Master pelo numero 19 91004-5115

## 🧪 Testes

Execute os testes do projeto:

```bash
# Todos os testes
cargo test

# Testes de um módulo específico
cargo test --package moma-core
cargo test --package moma-routine
```

---

## 📝 Estrutura de Módulos

| Módulo | Descrição |
|--------|-----------|
| `moma-api` | API REST para receber webhooks |
| `moma-worker` | Worker assíncrono para processar mensagens |
| `moma-routine` | Motor de rotinas e processamento de comandos |
| `moma-core` | Domínio principal (categorias, entradas, configurações) |
| `moma-auth` | Sistema de autenticação e multi-tenant |
| `moma-integration` | Integrações (WhatsApp, Email, IA, AMQP) |
| `moma-shared` | Código compartilhado e recursos |
| `moma-log` | Sistema de logging |
| `moma-migrator` | Ferramenta de migração de banco de dados |

---

## 🔒 Segurança

- ✅ Credenciais armazenadas em variáveis de ambiente (`.env`)
- ✅ Bancos de dados isolados por tenant
- ✅ Arquivo `.env` não versionado (veja `.gitignore`)
- ✅ Validação de mensagens do WhatsApp

**⚠️ Importante**: Nunca commite arquivos `.env` ou bancos de dados `.db` no repositório!

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer um Fork do projeto
2. Criar uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abrir um Pull Request

### Guia de Contribuição

- Siga as convenções de código Rust
- Adicione testes para novas funcionalidades
- Atualize a documentação quando necessário
- Mantenha commits claros e descritivos

---

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

## 👨‍💻 Autor

**Rafael Zornita**
- GitHub: [@rafaelzornita](https://github.com/rafaelzornita)
- Email: rafaelzornita@gmail.com

---

## 🙏 Agradecimentos

- Comunidade Rust
- Desenvolvedores das bibliotecas utilizadas
- Todos os contribuidores do projeto

---

## 📚 Recursos Adicionais

- [Documentação Rust](https://doc.rust-lang.org/)
- [Actix-Web Docs](https://actix.rs/)
- [Sea-ORM Docs](https://www.sea-ql.org/SeaORM/)
- [OpenAI API Docs](https://platform.openai.com/docs)

---

<div align="center">

**Desenvolvido com ❤️ usando Rust**

⭐ Se este projeto foi útil para você, considere dar uma estrela!

</div>
