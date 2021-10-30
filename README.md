# Pets

This is my simple game. It is a GraphQL back end programmed in rust with juniper.

## Running

It is designed to run in Docker (5.1MB image size). Use `docker run --rm -p 8000:8000 ryanknu/pets` to run it. When it is running, the GraphQL endpoint will be accessible via `/api`.

## Structure

- `main` is the rust entrypoint.
- `auth` is the code that handles http authentication.
- `types` is where all types and BASE resolvers for those types are, ie `crate::types::Trainer::trainer()` simply proxies to another module, but _every_ GraphQL connection in this application can be found in this file.
- `query` or `mutation` - these are root-level GraphQL resolvers, and call other modules.
- `$model` - this is the _actual_ resolver for a model exists e.g. `crate::trainer::trainer()` is what actually does database operations.
- `$x_to_$y` - this is the resolver for a link, so `pet_to_trainer` is expected to follow the link in the Query `{ Pet { Trainer { name }}}`.

## Game Rules

You create a trainer with `createTrainer`. You will be logged in. In case you are not logged in, you can call `authorizeTrainer` with your username and password. Your account will be created with 0 cash and no pets.

Upon logging in, you will receive `cash` based on your pets levels and how long you were gone.

Upon creating an account, you will receive 3 adoptable pets, that you can adopt and name calling `adoptPet`. This should cost 0 cash for the first pet, 1000 cash for the second, and 10000 cash for the third.

You can feed your pets by calling `feedPet`. This will cost 50 cash.
