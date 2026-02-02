package api

import "net/http"

func (h *Handler) RegisterRoutes() *http.ServeMux {
	mux := http.NewServeMux()

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "ERRO: método não permitido", http.StatusMethodNotAllowed)
		}

		h.GetAllProduct(w, r)
	})

	return mux
}