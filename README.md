# Amigo secreto

## Índice
 * [Funcionamento do sorteio](#funcionamento-do-sorteio)
 * [Uso](#uso)
    * [Compilação](#compilação)
    * [Como rodar](#rodando)
 * [Licença](#licença)

## Funcionamento do sorteio

A principar estrutura de dados é o participante, definido como 
```rust
// No arquivo src/participante.rs
pub struct Participante {
    pub nome: String,
    pub email: String
}
```

Isso significa que um `Participante` é uma estrutura com dois campos: um nome e um e-mail. Podemos criar um Participante com o nome de "Luís" e email de "luís@exemplo.org" ou um com nome de "Ana" e email de "a.na@gmail.com".
 
De onde vamos tirar os participantes do jogo? De um arquivo CSV, mais ou menos que nem esse

```csv
Nome,Email
Leocrécio,leo@gmail.com
Ana,ana.a@instituição.org
Felipe,filipão@hotmail.com
Sofia,sofia.sobrenome@uol.com.br

```
Aqui temos quatro participantes: o Leocrécio, a Ana, o Felipe e a Sofia, cada um com seus respectivos emails

A partir de uma lista de participantes (vamos chamar de `participantes`, olha que nome criativo), construímos um objeto chamado `Jogo`, nessa parte aqui

```rust
// src/main.rs
let mut jogo = Jogo::novo(participantes.len());
    
for p in participantes {
    jogo.add_participante(p)
}
```

A gente cria um `Jogo`, com o tamanho da lista de participantes, e para cada um dos participantes chamamos o método `add_participante()` que registra aquele participante no sistema. 

Daí a gente chega na função que faz todo o sorteio:
```rust
// src/jogo.rs
pub fn realizar_jogo(&mut self) -> JogoResultado {
        let (mut rng, seed) = gen_rng(self.seed.clone());
        self.participantes.shuffle(&mut rng);

        assert!(self.participantes.len() > 1); // não faz o jogo com menos de dois participantes

        return JogoResultado {
            seed,
            emails: iter_send(self.participantes.clone()),
            participantes: self.participantes.clone(),
        }
    }

```

Primeiro, temos essa linha intrigante aqui:
```rust
let (mut rng, seed) = gen_rng(self.seed.clone());
```
Para que o nosso sorteio seja auditável, seguro e "repetível", em vez fazermos tudo aleatório temos que sortear uma semente.Assim, se precisarmos repetir o sorteio porque, por exemplo, algum email não foi enviado, é só usar a mesma semente que o resultado vai ser o mesmo!

Essa função `gen_rng` analisa se temos uma, e caso não tivermos, cria uma aleatoriamente. Depois disso, usamos `.shuffle(&mut rng)` para "embaralhar" a lista de participantes. Isso porque, em vez de sortear um amigo secreto para cada um, nós só embaralhamos a lista e fazemos o primeiro tirar o segundo, o segundo tirar o terceiro, o terceiro tirar o quarto e por aí vai até que o último tira o primeiro.

Em seguida, vamos verificar que tem mais de um participante, porque amigo secreto funciona assim, né?
Daí vamos retornar o resultado!
```rust
return JogoResultado {
    seed,
    emails: iter_send(self.participantes.clone()),
    participantes: self.participantes.clone(),
}
```

Mas pera...

Não precisa mandar um email bonitinho para cada um?
Acabou, é só isso?

Não é não!

A gente retorna um objeto `JogoResultado`, que quando construído chama `iter_send(...)` - é essa função que manda um email para cada. Ela só é chamada meio escondida.

Mas como email é uma coisa complicadinha, envolvendo *SMTP*, e outras coisas mais técnicas, não vou explicar nesse texto. Se tiver curiosidade, olha no arquivo `src/email.rs`

## Uso
Presume-se que você tenha a linguagem de programação Rust e git instalados

### Compilação
Clone o repositório: 
```bash
$ git clone https://gitlab.com/franpessoa/amigo-secreto
```
Compile:
```bash
$ cargo build --release
```

Opcionalmente, instale como um executável
```bash
$ cargo install --path .
```

### Somente instalar, sem clonar o repositório
 ```bash
$ cargo install --git https://gitlab.com/franpessoa/amigo-secreto
```
Esse comando realiza implicitamente os passos acima

### Rodando
Primeiro, é necessário um arquivo `<nome>.csv`, com o seguinte formato:
```csv
Nome,Email
<participante_1_nome>,<participante_1_email>
<participante_2_nome>,<participante_2_email>
<participante_3_nome>,<participante_3_email>
<participante_4_nome>,<participante_4_email>
...
<participante_x_nome>,<participante_x_email>

```

Em seguida, crie um arquivo de ambiente `.env` com o seguinte formato e as informações de SMTP
```env
SMTP_PASSWORD="<sua-senha-smtp>"
SMTP_SENDER="<o-email-que-vai-enviar>"
SMTP_USERNAME="<seu-usuário-smtp>"
SMTP_RELAY="<seu-servidor-smtp-sem-porta>"
```
Por padrão, utiliza-se a porta 2525. Não há como configurar isso, ainda.

## Para fazer
 * melhor documentação interna de tipos e funções
 * configuração de SMTP

## Licença
Esse programa está sobre a licença código-aberto MIT.

Leia o arquivo `LICENSE` para saber mais