package models

import (
	"fmt"
	"time"

	"github.com/google/uuid"
)

type Product struct {
	Id uuid.UUID
	Url string
	Store string
	Last_checked_at time.Time
	Created_at time.Time
}

func (p Product) String() string {
		return fmt.Sprintf("DEBUG Product: %s \nUrl: %s \nStore: %s \nLast Checked: %s \nCreated At: %s\n",
							p.Id, p.Url, p.Store, p.Last_checked_at, p.Created_at)
}