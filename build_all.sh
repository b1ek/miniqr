#!/bin/sh
docker buildx build -t blekii/sail:latest . --platform linux/amd64,linux/arm64,darwin/amd64,linux/i386,linux/arm64v8,linux/arm64v7
