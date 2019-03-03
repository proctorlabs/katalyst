#!/bin/bash
mkdir -p reports
docker-compose build
docker-compose up --abort-on-container-exit