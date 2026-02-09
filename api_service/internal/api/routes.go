package api

import "net/http"

func (h *Handler) RegisterRoutes() *http.ServeMux {
	mux := http.NewServeMux()

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "ERRO: método não permitido", http.StatusMethodNotAllowed)
			return 
		}

		h.GetAllProduct(w, r)
	})

	mux.HandleFunc("/create-user", func (w http.ResponseWriter, r *http.Request)  {
		if r.Method != http.MethodPost {
			http.Error(w, "Erro: método não permitido", http.StatusMethodNotAllowed)
			return
		}

		h.CreateUser(w, r)
	})

	return mux
}