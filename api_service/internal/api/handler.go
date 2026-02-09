package api

import (
	"encoding/json"
	"log"
	"net/http"
	"price-crawler-api/internal/services"
	"price-crawler-api/internal/services/database"
	"price-crawler-api/internal/services/queue"
)

type Handler struct {
	store *database.Store
	rabbit *queue.RabbitMQ
	userService *services.UserService
}

func NewHandler(s *database.Store, r *queue.RabbitMQ, u *services.UserService) *Handler {
	return &Handler{store: s, rabbit: r, userService: u}
}

func (h *Handler) GetAllProduct(w http.ResponseWriter, r *http.Request) {
	products, err := h.store.GetAll()
	if err != nil {
		http.Error(w, "ERRO: não foi possível coletar produtos", http.StatusNotFound)
		return
	}

	w.Header().Set("Content-type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(products)
}

func (h *Handler) CreateUser(w http.ResponseWriter, r *http.Request) {
	var body services.UserServiceRequestBody
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		http.Error(w, "Erro: corpo da requisição não suportado", http.StatusBadRequest)
		return
	}

	err := h.userService.Create(body.Email, body.PasswordPlain)
	if err != nil {
		log.Printf("ERROR: erro ao criar usuário: %s", err)
		http.Error(w, "Erro: não foi possível criar usuário", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w)
}