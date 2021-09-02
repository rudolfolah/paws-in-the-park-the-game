docker stop $(docker ps | grep 'cosmwasm' | cut -d' ' -f1)
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.11.4
docker volume rm $(docker volume ls | cut -d' ' -f6 | grep '_cache')
