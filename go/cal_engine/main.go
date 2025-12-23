package main
import (
	"log"
	"github.com/mrxrsd/gojacego"
)

func main() {
	engine, _ := gojacego.NewCalculationEngine()

	vars := map[string]interface{}{
		"price": 29.99,
		"quantity": 5,
	}

	total, _ := engine.Calculate("price * quantity", vars)
	log.Printf("Total: %.2f", total)
}