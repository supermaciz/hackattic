package main

import (
	"bytes"
	"encoding/json"
	"io"
	"log"
	"net/http"
)

type ProblemSet struct {
	Token string `json:"token"`
}

type Solution struct {
	Secret string `json:"secret"`
}

func getWebsocketToken(token string) string {
	resp, err := http.Get("https://hackattic.com/challenges/websocket_chit_chat/problem?access_token=" + token)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()

	var problem ProblemSet
	err = json.NewDecoder(resp.Body).Decode(&problem)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("getWebsocketToken() %s %+v\n", resp.Status, problem)
	return problem.Token
}

func submitSolution(token string, secret string) {
	data := map[string]string{"secret": secret}
	jsonData, _ := json.Marshal(data)

	resp, err := http.Post(
		"https://hackattic.com/challenges/websocket_chit_chat/solve?access_token="+token,
		"application/json",
		bytes.NewBuffer(jsonData),
	)
	if err != nil {
		log.Fatal(err)
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("submitSolution() %s %s\n", resp.Status, body)
}
