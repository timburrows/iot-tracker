# Project Overview
This sample repository is an experimental Rust project that aims to provide an API to track the geolocation of IoT devices across the globe.

The synopsis is quite simple: provide a web API that effficiently processes a large volume of requests that report the geolocation of IoT devices, in 15 minute intervals. The latitude and longitude of device(s) should be recorded in a SQL database, and anomalies should be observed in a data warehouse such as Big Query.

As a Rust novice aspiring to level-up to an intermediate level, my first objective was to research trends in the Rust eco-system to invest my time/effort into learning the right crates and/or tech. As this is as much a learning experience as a project in itself, I will go into some detail into the conclusions I came to throughout my research.

 - Axum: A young contender in a rather small competition of web API frameworks for Rust. Built by the developers behind Tokio, the framework delivers a modest but capable interface to underlying technologies such as tower, hyper and tokio itself, and serves as a thin but powerful layer upon these technologies to give developers a higher-level API to build web services upon. Primarily, the reason I chose Axum was that more mature frameworks such as Rocket and others started to show a decrease in interest and momentum in development.

  - SeaORM: Coming from a C# background, I've been spoilt by the simplicity of Entity Framework and LINQ to SQL. Code-first database modelling and context of the database within the application runtime all provided to you on a silver platter. Well I don't know any better, so naturally I reached out for the Rust lang's plug and play ORM. I had dealt with Diesel previously, and through some brief follow up research found concurrency and performance issues which were of concern. Among others, SeaORM seemed like it had gained the most traction. Once I became a little more familiar with the ORM, I did come to appreciate it, but still I'm fairly unsatisfied. SeaORM didn't have support for all Postgres Types such as Point and others provided by Postgres modules like PostGIS. Despite some complaints, SeaORM has flexible migrations and a neat CLI to maagage them. Entity code-gen was great but on the contrary, it would be nice to see an entity-first alternative, where migrations are generated from entities defined by Rust structs.

  - GCP: I don't have much to say except that there's not much to see here. Barely any support for Rust, a small SDK for some basic things like Pub/Sub, Storage but not much more.

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

