package api

import "net/http"

func RegisterRoutes(UserHandler *UserHandler) *http.ServeMux {
	mux := http.NewServeMux()

	mux.HandleFunc("POST /create-user", UserHandler.CreateUser)

	return mux
}