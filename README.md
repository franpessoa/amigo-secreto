# [WIP] Amigo secreto - TEXTO DESATUALIZADO E INCOMPLETO!!
## Para resolver
 - melhorar a documentação de tipos e funções
 - completamente remover o código asíncrono
 - email de mock para ser ignorado


Esse site do amigo secreto tem duas partes - o algoritmo que cuida do jogo e o site em si. Esse texto vai explicar um pouco de como funciona o algoritmo, o sorteio e como os dados são recebidos e processados

A principar estrutura de dados é o participante, definido como 
```rust
// No arquivo src/participant.rs
pub struct Participante {
    pub nome: String,
    pub email: String
}
```

Isso significa que um `Participante` é uma estrutura com dois campos: um nome e um e-mail. Podemos criar um Participante com o nome de "Luís" e email de "luís@exemplo.org" ou um com nome de "Ana" e email de "a.na@gmail.com".
 
A linha em cima dessa definição tem alguns ~hieróglifos antigos~ macros que definem algumas coisas importantes sobre essa estrutura. Um **macro** é um código que produz mais código, e roda antes de todo o resto. Assim, quando essa estrutura for utilizada, os macros já terão gerado para ela algumas propriedades úteis.

~Os escritos satãnicos~ A linha é a seguinte:
```rust
#[derive(Serialize, Deserialize, Debug)]
```
Eu disse que era confuso! `#[derive( )]` só significa que nós estamos utilizando um macro, portanto não importa muito. O que importa é

 - `Serialize` significa que essa estrutura é serializável, ou seja, pode ser transformada em uma representação de texto
 - `Deserialize` significa que essa estrutura é deserializável, ou seja, pode ser induzida a partir de uma representação de texto
 
Que representação de texto é essa? Existem várias - algumas das mais comuns são YAML, TOML, e, a que é usada aqui, JSON:

```json
{
    "participantes": [
        {
            "nome": "Leocrécio",
            "email": "leo@gmail.com"
        },
        {
            "nome": "Fernanda",
            "email": "fernanda.f@hotmail.com"
        },
        {
            "nome": "Andrew",
            "email": "a@a.org"
        },
    ]
}
```

Isso aqui representa três participantes: o Leocrécio, a Fernanda e o Andrew, cada qual com seu e-mail.

É importante que essa estrutura tenha essas propriedades porque nós não vamos receber esses participantes como uma série de campos certinhos - nós vamos receber de um navegador de alguém. E precisamos ter uma estrutura que os dois lados da conversa entendam.

A parte que cuida de receber e processare os dados é essa função, que engloba quase tudo:
```rust
pub async fn jogo(Json(dados): Json<Jogo>) -> Result<Json<JogoSucesso>, Json<JogoErro>> {
    let (mut rng, seed) = gen_rng(dados.seed);
    let mut participantes = data.participantes;

    participantes.shuffle(&mut rng);

    let mut handles = vec![];
    for (idx, participante) in participantes.iter().enumerate() {
        let selecionado = match participantes.get(idx + 1) {
            Some(p) => &p.nome,
            None => &(&participantes).get(0).unwrap().name
        };

        let email_resultado = send(
            format!("{} <{}>", participante.name, participante.email), 
            selecionado.to_owned()
        );
        
        handles.push(tokio::spawn(email_resultado))
    }

    let resultados = futures::future::join_all(handles).await;
    for j in resultados {
        match j {
            Err(e) => return Err(Json(JogoErro::new(e.to_string()))),
            Ok(_) => continue
        }
    }

    return Ok(Json(JogoSucesso::new(seed, participantes)))
}
```

Ela é bem complexa, então vamos por partes:

```rust
pub async fn
```
declara uma função pública assíncrona. Pública tem a ver com a visibilidade interna desse código, então não vamos nos preocupar com isso. Assíncrona eu explico mais para frente.

```rust
jogo(Json(dados): Json<Jogo>) -> Result<Json<JogoSucesso>, Json<JogoErro>>
```
Isso aqui é a assinatura da função. funciona asssim

```
nome_da_função(parametros_de_entrada) -> tipo_de_saída
```

`jogo()` é o nome dessa função. Ela tem uma entrada `dados`, que vai ser encapsulada em `Json(dados)` para que aconteca a decodificação da representação em Json.

Parâmetros precisam ter um tipo, declarado assim:
```
nome_da_função(parametro_1: tipo_1, parametro_2: tipo_2)
```
Nesse caso, `Json(dados)` tem o tipo `Json<Jogo>`. Os <> significam que Json é um **tipo genérico**, que pode ser formado por vários outros tipos - nesse caso, o tipo `Jogo`

`Jogo` é uma outra estrutura, declarada como
```rust
pub struct Jogo {
    pub participantes: Vec<Participante>,
    pub seed: Option<String>
}
```

Ela tem dois campos: um é uma lista (`Vec<>)` de `Participante` é o outro é, opcionalmente, uma semente. Semente tem a ver com o sorteio, que vêm daqui a pouco.

Agora sabemos que essa função se chama `jogo` é leva um parâmetro `dados` do tipo `Jogo`. Mas o que ela retorna?

```rust
Result<Json<JogoSucesso>, Json<JogoErro>>
```
O `Result<T, E>` significa ou um ou outro. Portanto, se der tudo certo,  retornamos o primeiro - `T`, e se der errado retornamos `E`.
Nesse caso temos que:

 - Se ser tudo certo, retornamos `Json<JogoSucesso>`
 - Se der errado, retornamos `Json<JogoErro>`
 
 Nós já sabemos que o `Json<>` só está aqui para cuidar de transformar essas estruturas em/de texto. `JogoSucesso` e `JogoErro` são outras estruturas que só existem para que representar as possibilidades de saída, então não precisamos falar delas aqui

Próxima linha: vamos falar de sorteio
```rust
let (mut rng, seed) = gen_rng(dados.seed);
```

`gen_rng()` é uma função importante. Ela cuida de preparar o gerador de números aleatórios que vai ser usado no sorteio, fazendo o seguinte:

```rust
pub fn gen_rng(seed: Option<String>) -> (ChaCha20Rng, String) {
    let rng_seed = match seed {
        Some(s) => s,
        None => {
            let seed: String = ChaCha20Rng::from_entropy()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            
            seed
        }
    };

    return (Seeder::from(&rng_seed).make_rng(), rng_seed.clone())
}
```
Lembra que a entrada `Jogo` tinha um parâmetro `seed`? É aqui que ele entra.

Uma semente é um número, texto, qualquer coisa, que vai ser usada para gerar os números aleatórios. Toda vez que você gera um número aleatório, você está também criando outra semente para ele.
**Toda vez que u