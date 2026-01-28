package models

import (
	"time"
	"github.com/google/uuid"
	"github.com/shopspring/decimal"
)

type price_chapter struct {
	id uuid.UUID
	product uuid.UUID
	value decimal.Decimal
	created_at time.Time
}