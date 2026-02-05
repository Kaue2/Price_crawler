package main

import (
	"log"
	"net/http"
	"os"
	"price-crawler-api/internal/api"
	"price-crawler-api/internal/database"
	"price-crawler-api/internal/queue"
	"price-crawler-api/internal/utils"

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

	handler := api.NewHandler(store, rabbit)
	log.Println("DEBUG: Handler criado")

	
	router := handler.RegisterRoutes() // mux personalizado
	port := ":8080"
	
	corsHandler := utils.EnableCORS(router)
	log.Println("DEBUG: Cors configurado")

	log.Println("DEBUG: subindo servidor...")

	server := &http.Server{
		Addr: port,
		Handler: corsHandler,
	}

	if err := server.ListenAndServe(); err != nil {
		panic(err)
	}
}