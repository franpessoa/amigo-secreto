CREATE TABLE jogos (
  id_jogo INT PRIMARY KEY AUTO_INCREMENT,
  nome VARCHAR(255) NOT NULL,
  data VARCHAR(255) NOT NULL DEFAULT (CURDATE()),
  max INT NOT NULL,
  senha VARCHAR(255) NOT NULL
);

CREATE TABLE jogadores (
  id_jogador INT PRIMARY KEY AUTO_INCREMENT,
  nome VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  id_jogo INT NOT NULL
);

INSERT INTO jogos(Nome, Senha, Max) VALUES ("Teste 2", "teste2", 10);
INSERT INTO jogadores(Nome, Email, Jogo_ID) VALUES ("Francisco 3", "franciscomspessoa@gmail.com", 1);
SELECT * FROM jogadores INNER JOIN jogos ON jogos.Id = jogadores.Jogo_ID AND jogos.Id = 1;