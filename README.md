# Project Overview
This sample repository is an experimental Rust project that aims to provide an API to track the geolocation of IoT devices across the globe.

## Usage
### Install Rust 
Below is a one-liner, but for a more detailed explaination see here https://www.rust-lang.org/tools/install


```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

### Install Postgres
For those blessed with the common-sense to be using Linux, use your package manage. For Arch:
```bash
pacman -Syy postgres
initdb -D /var/lib/postgres/data
systemctl start postgres && systemctl enable postgres
```

If you need more than that, for Arch users, https://wiki.archlinux.org/title/PostgreSQL, otherwise follow your distributions instructions for installation.

### Install gcloud CLI (Google Cloud Platform)
``` bash
curl -O https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-cli-459.0.0-linux-x86_64.tar.gz
tar -xf google-cloud-cli-459.0.0-linux-x86_64.tar.gz
./google-cloud-sdk/install.sh

gcloud init
```

### Install Docker
Again, follow your distibutions instructions, but for those on Arch:
```bash
pacman -Syy docker docker-buildx
systemctl start docker && systemctl enable docker
```

To use docker on the CLI without sudo:
Add your [username] to the docker group
```bash
gpasswd -a [username] docker
```

# Deployment Instructions
Note: these instructions assume access to GCP and are purely temporary dev instructions. Obviously this doesn't belong in the README, so will be redacted in future.
## Pre-requisites
 - Docker
 - Google Cloud CLI

```
gcloud auth configure-docker us-central1-docker.pkg.dev

docker build -t us-central1-docker.pkg.dev/vocal-cyclist-410811/iot-tracker-repository/iot-tracker-app:latest .
docker push us-central1-docker.pkg.dev/vocal-cyclist-410811/iot-tracker-repository/iot-tracker-app:latest

gcloud run deploy iot-tracker-app \
--allow-unauthenticated \
--service-account=182557199593-compute@developer.gserviceaccount.com \
--max-instances=1 \
--image=us-central1-docker.pkg.dev/vocal-cyclist-410811/iot-tracker-repository/iot-tracker-app:latest \
--region=us-central1 \
--port=8080 \
--set-env-vars "DATABASE_URL=postgres://iot-tracker-user:nodle@35.238.127.198:5432/iot-tracker" \
--set-env-vars "HOST=35.208.210.171" \
--add-cloudsql-instances=vocal-cyclist-410811:us-central1:iot-tracker-sql
```

