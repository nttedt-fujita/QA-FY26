# M3 受入検査DB バックエンド
# Session 36で作成

FROM golang:1.24-alpine AS builder

WORKDIR /app

# 依存関係のダウンロード
COPY go.mod go.sum* ./
RUN go mod download || true

# ソースコードのコピーとビルド
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o /api ./cmd/api

# 実行用イメージ
FROM alpine:3.19

WORKDIR /app
COPY --from=builder /api /app/api

EXPOSE 8080
CMD ["/app/api"]
