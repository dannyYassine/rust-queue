# rust-queue

## Install rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
sudo apt-get update
sudo apt install build-essential -y
```

* restart rust analyzer server

## Database

```postgres
CREATE TABLE jobs (
    id SERIAL PRIMARY KEY,
    payload TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    model_type TEXT NOT NULL
);
```

## vs code extensions

* rust-lang.rust-analyzer (should also install dustypomerleau.rust-syntax and 1YiB.rust-bundle)
* vadimcn.vscode-lldb (for debugging)
