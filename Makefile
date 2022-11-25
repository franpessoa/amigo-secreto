main: mail.go main.go Lista.csv
	@echo "Compilando binário"
	@go build .
	
	@echo "Rodando"
	@./amigo-secreto