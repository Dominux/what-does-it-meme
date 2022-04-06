test_all:
	docker-compose -f docker-compose.test.yml up --build --force-recreate

test:
	docker-compose -f docker-compose.test.yml build && \
	docker-compose -f docker-compose.test.yml run server
