serve:
    mold -run cargo watch -x run

psql:
    docker-compose exec -it db psql -U postgres -d service_life

tailwind:
    npx tailwindcss -i ./src/input.css -o ./public/output.css --watch
