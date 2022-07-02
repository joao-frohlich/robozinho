#!/bin/bash

echo "Limpando as pastas de input/output"
rm inputs/*
rm outputs/*

echo "Executando o experimento"

for i in {1..5}
do
    echo "  Experimento $i"
    echo "      Gerador"
    cargo run --example gerador
    echo "      A*"
    cargo run --example robozinho_estrela $i --release > outputs/estrela_$i.txt
    echo "      Guloso"
    cargo run --example robozinho_guloso $i --release > outputs/guloso_$i.txt
    echo "      Dijkstra"
    cargo run --example robozinho_determinista $i --release > outputs/determinista_$i.txt
done