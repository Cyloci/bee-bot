APP_NAME=bee-bot
IMAGE_NAME=cyloci/$(APP_NAME)


docker-build:
	docker build -t $(IMAGE_NAME) .

docker-push:
	docker push $(IMAGE_NAME)

prod-logs:
	kubectl logs -f -l app=$(APP_NAME)

prod-shell:
	kubectl exec -it $$(kubectl get pods | grep "${APP_NAME}" | awk '{print $$1}' | head -n 1) -- /bin/sh

prod-restart:
	kubectl rollout restart deployment $(APP_NAME)

prod-deploy: docker-build docker-push prod-restart
