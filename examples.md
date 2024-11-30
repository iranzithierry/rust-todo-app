## CREATE
```curl
curl --request POST \
  --url http://localhost:8080/todos \
  --header 'Content-Type: application/json' \
  --data '{
  "title": "Learning Rust"
}'
```

## GET 
```curl
curl --request GET \
  --url http://localhost:8080/todos
```

## UPDATE
```curl
curl --request PUT \
  --url http://localhost:8080/todos/<UUID> \
  --header 'Content-Type: application/json' \
  --data '{
  "completed": true
}'
```

## DELETE
```curl
curl --request DELETE \
  --url http://localhost:8080/todos/<UUID>
  ```

## RETRIEVE
```curl
curl --request GET \
  --url http://localhost:8080/todos/<UUID>
  ```