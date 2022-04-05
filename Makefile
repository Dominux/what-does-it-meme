run:
	docker-compose -f docker-compose.test.yml up --build --force-recreate

run_server:
	docker-compose -f docker-compose.test.yml build && \
	docker-compose -f docker-compose.test.yml run server
