package api

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"price-crawler-api/internal/services"
)

type UserHandler struct {
	userService *services.UserService
}

func NewUserHandler(s *services.UserService) *UserHandler{
	return &UserHandler{userService: s}
}

func (u *UserHandler) CreateUser(w http.ResponseWriter, r *http.Request) {
	var body services.UserServiceRequestBody
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		http.Error(w, "Erro: corpo da requisição não suportado", http.StatusBadRequest)
		return
	}

	err := u.userService.Create(body.Email, body.PasswordPlain)
	if err != nil {
		if errors.Is(err, services.ErrInvalidEmail) {
			http.Error(w, "email inválido", http.StatusBadRequest)
			return
		}

		log.Printf("ERROR: erro ao criar usuário: %s", err)
		http.Error(w, "Erro: não foi possível criar usuário", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w)
}