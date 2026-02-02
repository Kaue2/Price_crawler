package models

import (
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/shopspring/decimal"
)

type price_chapter struct {
	Id uuid.UUID
	Product uuid.UUID
	Value decimal.Decimal
	Created_at time.Time
}

func (p price_chapter) String() string {
	return fmt.Sprintf("DEBUG: Price Chapter: %s: \nProduct: %s \nValue: %v \nCreated At: %s\n",
					p.Id, p.Product, p.Value, p.Created_at)
}