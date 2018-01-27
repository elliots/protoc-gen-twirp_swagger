package main

import (
	"context"
	"fmt"

	"github.com/elliots/protoc-gen-twirp_swagger/example/clients/go"
)

func main() {
	cfg := swagger.NewConfiguration()
	cfg.BasePath = "http://localhost:8080"
	client := swagger.NewAPIClient(cfg)

	hat, resp, err := client.HaberdasherApi.MakeHat(context.Background(), swagger.ExampleSize{
		Inches: 20,
	})
	if err != nil {
		panic(err)
	}

	fmt.Printf("Got response code: %d hat: %v", resp.StatusCode, hat)
}
