test:
	cp .test.env .env &&\
	docker-compose -f docker-compose.test.yml down && \
	docker-compose -f docker-compose.test.yml build && \
	docker-compose -f docker-compose.test.yml run server

dev:
	cp .dev.env .env &&\
	docker-compose -f docker-compose.dev.yml down && \
	docker-compose -f docker-compose.dev.yml build && \
	docker-compose -f docker-compose.dev.yml run server
