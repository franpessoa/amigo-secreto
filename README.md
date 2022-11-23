# Amigo Secreto

Meu programinha para o amigo secreto da família

**Indíce:**

1. Leitura de Dados

2. Sorteio
   
   1. Formato
   
   2. _Seeding_

3. Envio de E-Mails

4. Conclusão

## Procedimentos

#### Leitura dos dados

Quando nós registramos as pessoas, elas são inseridas em uma tabela no formato (Nome, Email)  

| Nome  | Email                |
| ----- | -------------------- |
| Léo   | leo@exemplo.com      |
| Sofia | sofia@exemplo.com.br |
| Ian   | ian@email.com        |

O arquivo final fica no formato .CSV (valores separados por vírgula), uma forma de guardar dados na forma de um texto

```csv
Léo,leo@exemplo.com
Sofia,sofia@exemplo.com.br
Ian,ian@email.com
```

Após os valores serem extraidos do arquivo, as pessoas são registradas em uma lista de _structs_ como essa (calmae já explico):

```go
type Pessoa struct {
	Nome         string
	Mail         string
	AmigoSecreto string
}
```

É como se as pessoas fossem armazenadas em uma tabelinha de informações como essa:

| Nome | Mail (e-mail)   | Amigo Secreto        |
| ---- | --------------- | -------------------- |
| Léo  | leo@exemplo.com | vazio *por enquanto* |

**Detalhe técnico:** O nome `string` significa que esses valores serão armazenados na forma de texto

#### Sorteio

Vamos supor que depois do último passo obtemos a seguinte lista de pessoas:

| 1   | 2     | 3   |
| --- | ----- | --- |
| Léo | Sofia | Ian |

O amigo secreto é determinado pela ordem dessa lista: o primeiro tira o segundo, o segundo tira o terceiro e o terceiro tira o primeiro.

| Nome      | Tirou     |
| --------- | --------- |
| Léo (1)   | Sofia (2) |
| Sofia (2) | Ian (3)   |
| Ian (3)   | Léo (1)   |

Só que, se usarmos essa ordem sem nada, não teremos sorteado: a ordem vai ser a mesma ordem que os nomes estavam no arquivo, e **não é isso que a gente quer**

Para isso, vamos **embaralhar** essa lista com o algoritmo chamado de [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle). Mas para explicar ele, vamos expandir essa lista:

| 1   | 2     | 3   | 4    | 5       |
| --- | ----- | --- | ---- | ------- |
| Léo | Sofia | Ian | Vera | Gustavo |

Uma lista de **5** posições, certo? Assim, teremos 5 **turnos de embaralhamento**. Em cada um desses turnos estaremos pegando os números:

- $i$  - Número da célula correspondente ao turno, começa no 1, depois vat para 2, _etc._

- $j$ - Número aleatório entre $1$ e $i$. No primeiro turno é 1, no segundo pode ser 1 ou dois, no terceiro pode ser 1, 2 ou 3, _etc._ 

Assim, trocaremos as posições de $i$ por $j$. Se $i = j$, não acontece nada. Se $i \neq j$, suas posições são trocadas

Vamos exemplificar:

**_Rodada 1 - Entre 1 e 1_**

| 1            | 2     | 3   | 4    | 5       |
| ------------ | ----- | --- | ---- | ------- |
| *<u>Léo</u>* | Sofia | Ian | Vera | Gustavo |

Nada acontece



**_Rodada 2 - Entre 1 e 2_**

| 1            | 2              | 3   | 4    | 5       |
| ------------ | -------------- | --- | ---- | ------- |
| <u>_Léo_</u> | <u>_Sofia_</u> | Ian | Vera | Gustavo |

Vamos supor que o número 1 ($j$) tenha sido sorteado. Então vamos trocar a célula 2 ($i$) pela célula 1 ($j$), o que dá em

| 1              | 2            | 3   | 4    | 5       |
| -------------- | ------------ | --- | ---- | ------- |
| <u>*Sofia*</u> | <u>*Léo*</u> | Ian | Vera | Gustavo |



**_Rodada 3 - Entre 1 e 3_**

| 1              | 2            | 3            | 4    | 5       |
| -------------- | ------------ | ------------ | ---- | ------- |
| *<u>Sofia</u>* | *<u>Léo</u>* | <u>*Ian*</u> | Vera | Gustavo |

Vamos supor que o número 2 tenha sido sorteado. Vamos trocar a célula 2 pela 3

| 1              | 2            | 3            | 4    | 5       |
| -------------- | ------------ | ------------ | ---- | ------- |
| *<u>Sofia</u>* | *<u>Ian</u>* | <u>*Léo*</u> | Vera | Gustavo |



**_Rodada 4 - Entre 1 e 4_**

| 1              | 2            | 3            | 4             | 5       |
| -------------- | ------------ | ------------ | ------------- | ------- |
| *<u>Sofia</u>* | *<u>Ian</u>* | <u>*Léo*</u> | <u>*Vera*</u> | Gustavo |

Vamos supor que o número 1 tenha sido sorteado. Vamos trocar a célula 4 pela 1

| 1             | 2            | 3            | 4              | 5       |
| ------------- | ------------ | ------------ | -------------- | ------- |
| *<u>Vera</u>* | *<u>Ian</u>* | <u>*Léo*</u> | <u>*Sofia*</u> | Gustavo |



***Rodada 5 - Entre 1 e 5***

| 1             | 2            | 3            | 4              | 5                |
| ------------- | ------------ | ------------ | -------------- | ---------------- |
| *<u>Vera</u>* | *<u>Ian</u>* | <u>*Léo*</u> | <u>*Sofia*</u> | <u>*Gustavo*</u> |

Vamos supor que o número 5 seja sorteado. Nada acontece



***Resultado Final***

Começamos com:

| 1   | 2     | 3   | 4    | 5       |
| --- | ----- | --- | ---- | ------- |
| Léo | Sofia | Ian | Vera | Gustavo |

Terminamos com:

| 1    | 2   | 3   | 4     | 5       |
| ---- | --- | --- | ----- | ------- |
| Vera | Ian | Léo | Sofia | Gustavo |



***Na prática***

Ná prática, o sorteio é feito por essa função aqui:

```go
func shuffleList(lista []Pessoa) []Pessoa {
	for i := range lista {
		j := rand.Intn(i + 1)
		lista[i], lista[j] = lista[j], lista[i]
	}

	return lista
}
```

Pegamos uma lista `lista`, iteramos sobre ela com o contador `i`, sorteamos um número `j` e trocamos as posições.

Assim **todas as combinações possuem a mesma probabilidade de acontecerem**



***Seeding***

Para que em todos os sorteios sejam sorteados números diferentes, e para que os sorteios sejam replicáveis (se um email não for entregue, por exemplo, podemos realizar o sorteio novamente com o mesmo resultado), utilizamos uma *seed* ("semente"). A *seed* é dada ao aleatorizador no início do programa, e é retirada do horário atual. Ela influencia no sorteio, de modo a nunca obteremos o mesmo resultado. 

Vamos dar um exemplo. Considere esse programa que gera números aleatórios:

`>> 10`

Se rodarmos ele de novo, com a mesma seed, obteremos:

`>> 10`

Se rodarmos ele com outra seed:

`>> 6`

Se rodarmos de novo com a nova seed:

`>> 6`

Entenderam? No começo do programa, existe essa linha de código:

```go
seed := time.Now().UnixNano()
rand.Seed(seed)
```

Com `seed := time.Now().UnixNano()`, estamos gerando um número através do horário, e em seguida utilizando o método `rand.Seed()` para usar ela como a chave do nosso aleatorizador.

Logo após isso, executamos essas instruções:

```go
file, err := os.Create("seed.key")

if err != nil {
	log.Fatal(err)
}

defer file.Close()
file.WriteString(fmt.Sprintf("%v", seed))
```

Parece complexo, mas o que esse bloco faz é registrar essa _seed_ em um arquivo, denominado de `seed.key`. Assim, podemos utilizar ela para refazer o sorteio e atestar sua veracidade, ou simplesmente remandar os e-mail's com o mesmo conteúdo

Tá vendo? Não é só um amigo secreto, é um amigo secreto ***A-U-D-I-T-Á-V-E-L***

#### Envio de e-mail's

Essa parte é mais abstrata: eu não tenho paciência para ficar contando unzinho e zerinho e mandar os e-mails. Por isso, eu um serviço já pronto: o Mailjet

Tem um monte de outros que eu poderia usar, só escolhi esse porque era prático e funcionou no meu caso. 

**Alguns detalhes:**

O email é mandado a partir de um Outlook que eu criei especificamente para o bot: `amgsecreto.go@outlook.com`

No fim, cada mensagem tem a seguinte estrutura:

```html
<h2>{nome}, seu amigo secreto foi sorteado!</h2>
<h3>E elu é (rufem os tambores):</h3>
<img src="https://cdn.statically.io/og/🎉{nome}🎉.jpg" alt={nome} height=200px width=350px>
<br><br>
<p><a href="https://github.com/franpessoa/amigo-secreto">Código Fonte</a>  ||  Seed: <strong>{seed}</strong><p>
```

A primeira linha é um título:

A segunda é um subtítulo, preparação emocional

A terceira é uma imagem com o nome do fidedigno

A quarta são duas linhas em branco

A quinta é uma nota de rodapé com o link para o repositório e a _seed_ utilizada



O resultado final fica assim:

![](http://0x0.st/oUzq.png)

#### Conclusão

Ficou com alguma dúvida? Só mandar para mim que eu respondo feliz! E é tudo código aberto, pode xeretar á vontade

Os arquivos estão dispostos desta forma:

```
amigo-secreto
├── go.mod - Arquivo que indentifica o nome do pacote, a versão do GO e dependências
├── go.sum - Arquivo de lock das dependências
├── mail.go - Programa com a função de mandar emails
├── main.go - Programa raiz, todo o resto acontece aqui
└── README.md - Esse textinho aqui

```


