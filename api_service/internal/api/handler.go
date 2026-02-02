package api

import (
	"encoding/json"
	"net/http"
	"price-crawler-api/internal/database"
	"price-crawler-api/internal/queue"
)

type Handler struct {
	store *database.Store
	rabbit *queue.RabbitMQ
}

func NewHandler(s *database.Store, r *queue.RabbitMQ) *Handler {
	return &Handler{store: s, rabbit: r}
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