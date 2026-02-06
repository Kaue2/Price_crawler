package dtos

import "github.com/google/uuid"

type LoginRequest struct {
	Email string `json:"email"`
	Password string `json:"password"`
}

type UserResponse struct {
	Id    uuid.UUID `json:"id"`
	Email string `json:"email"`
}