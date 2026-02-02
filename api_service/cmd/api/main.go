package main

import (
	"log"
	"os"
	"price-crawler-api/internal/database"
	"price-crawler-api/internal/queue"

	"github.com/joho/godotenv"
)

func main() {
	//pwd, _ := os.Getwd()
	//log.Printf("WARNING: caminho para o .env: %s\n", pwd)

	if err := godotenv.Load("../.env"); err != nil {
		log.Fatalf("ERROR: não foi possível carregar .env")
	}

	postgre_conn := os.Getenv("DATABASE_URL")
	rabbit_conn := os.Getenv("MQ_CONNECTION")
	log.Printf("DEBUG: Conexão com Postgre em: %s\n", postgre_conn)
	log.Printf("DEBUG: Conexão com Rabbit em: %s\n", rabbit_conn)

	store, err := database.NewStore(postgre_conn)

	if err != nil {
		log.Fatalf("ERROR: falha ao conectar com Postgre: %s\n", err)
	}
	defer store.Close() // Agenda destruição para o final da função

	rabbit, err := queue.NewRabbitMQConnection(rabbit_conn)
	if err != nil {
		log.Fatalf("ERROR: falha ao conectar com RabbitMQ: %s\n", err)
	}
	defer rabbit.Close()

	log.Println("DEBUG: conexão com postgre estabelecida")
	log.Println("DEBUG: conexão com rabbit estabelecida")

	products, err := store.GetAll()
	if err != nil {
		log.Fatalf("ERROR: falha ao buscar por todos os produtos: %s\n", err)
	}

	log.Printf("DEBUG: todos os produtos encontrados: \n\n%v\n", products)
}