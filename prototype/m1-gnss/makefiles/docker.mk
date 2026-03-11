# Docker操作コマンド（今後対応）

.PHONY: docker-build docker-run docker-shell

# イメージビルド
docker-build:
	docker build -t m1-gnss-dev -f Dockerfile.dev .

# コンテナ起動（USB対応、バックエンド）
# --privileged: USBデバイスアクセス用
docker-run:
	docker run --rm -it \
		-v $(PWD)/backend:/workspace/backend \
		-v /dev:/dev \
		--privileged \
		-p 8080:8080 \
		m1-gnss-dev \
		bash -c "cd /workspace/backend && RUST_LOG=debug cargo run"

# コンテナシェル
docker-shell:
	docker run --rm -it \
		-v $(PWD)/backend:/workspace/backend \
		-v /dev:/dev \
		--privileged \
		-p 8080:8080 \
		m1-gnss-dev \
		bash

# TODO: docker-compose.yml を作成してバックエンド+フロントエンド構成に対応
