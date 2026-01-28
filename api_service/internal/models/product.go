package models

import (
	"time"

	"github.com/google/uuid"
)

type Product struct {
	id uuid.UUID
	url string
	store string
	last_checked_at time.Time
	created_at time.Time
}