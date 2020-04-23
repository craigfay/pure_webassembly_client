# About
* What do bundle-sizes look like when you build your client application entirely with WebAssembly?

# Setup
* Install Docker > version 18.0.0
* Copy, then edit environment variables file: `cp .env.example .env`

# Commands
* Build Docker Image: `docker build . -t webassembly` 
* Start Application: `docker run -it --rm --name webassembly webassembly`

# Notes
* Expect build steps to take > 5 minutes. This is normal.

