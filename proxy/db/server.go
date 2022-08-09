package db

import (
	"database/sql"
	_ "github.com/go-sql-driver/mysql"
)

type DBService struct {
	Db *sql.DB
}

func Init(password string, url string) (*DBService, error) {
	dsn := "root:" + password + "@tcp(" + url + ")/game"
	db, err := sql.Open("mysql", dsn)
	if err != nil {
		return nil, err
	}
	err = db.Ping()
	if err != nil {
		return nil, err
	}

	return &DBService{Db: db}, nil
}
