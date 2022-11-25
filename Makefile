main: mail.go main.go Lista.csv
	@echo "Compilando binário"
	@go build .

run: amigo-secreto
	./amigo-secreto