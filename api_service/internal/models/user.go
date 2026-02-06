package models

import (
	"time"

	"github.com/google/uuid"
)

type User struct {
	Id uuid.UUID
	Email string
	Password_hash string
	Created_at time.Time
}