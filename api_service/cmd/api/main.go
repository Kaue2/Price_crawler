package main

import (
	"log"
	"os"
	"price-crawler-api/internal/database"

	"github.com/joho/godotenv"
)

func main() {
	pwd, _ := os.Getwd()
	log.Printf("WARNING: caminho para o .env: %s\n", pwd)

	if err := godotenv.Load("../.env"); err != nil {
		log.Println("WARNING: não foi possível carregar .env")
	}

	postgre_conn := os.Getenv("DATABASE_URL")
	log.Printf("Conexão com Postgre em: %s\n", postgre_conn)

	store, err := database.NewStore(postgre_conn)

	if err != nil {
		log.Printf("ERROR: falha ao conectar com Postgre: %s\n", err)
	}
	defer store.Close() // Agenda destruição para o final da função

	log.Println("DEBUG: conexão com postgre estabelecida")
}