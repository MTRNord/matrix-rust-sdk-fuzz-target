#!/bin/bash
docker-compose -f docker-compose.deps.yml up -d postgres
docker-compose up
