# ** Quick environment set-up **

** DEPENDENCIES ** 
|- Rust
    |- sqlx
----------
|- Postgres
----------

* Installation

    |- Rust:

    Linux:

        |- install curl via pacakge manager (ex. apt install curl or pacman -S curl)

        |- run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    Windows:

        |- install curl via: https://curl.se/windows/ (x64 for amd/intel || arm64 for rasberry pi)

        |- run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

        (check if working: rustc --version ; cargo --version)
    
    Rust LSP for VS code

        |- rust-analyzer
    
    |- PostgreSQL

    Linux:
        
        |- install: postgresql postgresql-contrib via package manager (ex. apt install postgresql postgresql-contrib)
        
        |- sudo systemctl start postgresql

    Windows

        |- download installer from: https://www.postgresql.org/download/windows/

        |- Run installer

            |- you can add the gui panel but the postgres commands to be listed later in "DB Setup" will be CLI commands

        |- Follow Installer Instructions

* DB Setup

    |- run in terminal:

        |- psql (this should log you in as user if not follow log in method described during installation)

        |- create database webprintingapi_db; (!Important: database must be called webprintingapi_db)

* Rust Setup

        |- clone Web-Print_API: git clone https://github.com/zac4312/Web-Print_API.git

        |- cd to project directory

        |- add file ./.env

        |- edit .env file, add: DATABASE_URL=postgres://user:password@localhost:5432/webprintingapi_db

            (if above dont work try: DATABASE_URL=postgres://postgres@localhost/webprintingapi_db)

        |- In terminal run: cargo build

        |- wait till it finishes building

        |- run: cargo add sqlx

        |- run: cargo sqlx migrate run

        |- run: psql -d webprintingapi_db -f ./docs/test_seed.sql
    
        |- run: cargo sqlx prepare

        |- run: cargo run
    
            (back end should now be running)

* JS Integration

        |- clone: https://github.com/zac4312/Web-Print_JS-Integration.git
        
(*Note: any changes to the code will not be possible to push, for push request please message me for access)
    




    
