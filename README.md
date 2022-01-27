<div align="center">
    <h1>Serverless GraphQL Service Mesh</h1>
    [WIP] This repository provides a reference architecture for a serverless, graphql-based service mesh on Amazon Web Services (AWS).
    <p align="center">
        <a href="#Services">Services</a> •
        <a href="#about">About</a> •
        <a href="#installation">Installation</a> •
        <a href="#resources">Resources</a> •
        <a href="#roadmap">Roadmap</a> •
        <a href="#license">License</a>
    </p>
    <hr />
</div>

## Services


|   Service   |   Tech   |                     Endpoint                     |
|:-----------:|:--------:|:------------------------------------------------:|
| **Router**  | **Rust** |     **https://mesh.ocrateris.cloud/router**      |
| **Gateway** | **Node** | **https://mesh.ocrateris.cloud/gateway/graphql** |
|             |          |                                                  |
|  Customers  |   Rust   |  https://mesh.ocrateris.cloud/service/customers  |
|  Products   |   Rust   |  https://mesh.ocrateris.cloud/service/products   |
|   Reviews   |   Rust   |   https://mesh.ocrateris.cloud/service/reviews   |

## About
*...*
### Design Principles
*...*
### Architecture
*...*

## Installation

### Prerequisites

✅ Create an Amazon Web Services (AWS) account. <br />
✅ Install the AWS command line interface (AWS-CLI). <br />
✅ Install `node` and `pnpm` (package manager). <br />
✅ Install the AWS Cloud Development Kit toolkit (AWS-CDK) . <br />
✅ Bootstrap your AWS account for use with cdk. <br />
✅ Install and configure `docker` and `buildx` plugin. <br />
✅ Install `rust` and related toolchain. <br />

### Get Started

1. Clone this repository & install tooling dependencies.

```bash
git clone git@github.com:medelman17/serverless-graphql-mesh.git
cd serverless-graphql-mesh 
pnpm install
```
2. Update project configuration file with your AWS account number.

```bash
Insert nifty bash one-liner, here.
```

3. Build services and other accoutrement.
```bash
pnpx turbo run service
```

4. Deploy to the cloud. 

```bash
pnpx turbo run deploy
```
5. Profit.
## Resources

### GraphQL

- [**Apollo GraphQL Platform**](https://www.apollographql.com/) - The Apollo Graph Platform unifies GraphQL across your apps and services, unlocking faster delivery for your engineering teams.
- [**Federation**](https://www.apollographql.com/apollo-federation) - Apollo Federation is the industry-standard open architecture for building a distributed graph that scales across teams.
- [**Rust Router**](https://www.apollographql.com/blog/announcement/backend/apollo-router-our-graphql-federation-runtime-in-rust/) - Use Apollo’s graph router to compose a unified graph from multiple subgraphs, determine a query plan, and route requests across your services. The Apollo Router is written in Rust, and it is fast. ⚡️ Early benchmarks show that the Router adds less than 10ms of latency to each operation, and it can process 8x the load of the JavaScript Apollo Gateway. Packaged as a standalone, multi-threaded binary, the Router can use all available CPU cores without needing to run multiple instances on a single machine.

### Cloud Infrastructure
- [**Cloud Development Kit (CDK)**](https://aws.amazon.com/cdk/) - The AWS Cloud Development Kit (AWS CDK) is an open-source software development framework to define your cloud application resources using familiar programming languages.

### Distributed Systems

### Tooling & Stack Bits
- [**Turborepo**](https://turborepo.org/) - Monorepos that make ship happen; turborepo is a high-performance build system for JavaScript and TypeScript codebases. A fresh take on the whole setup. Designed to glue everything together. A toolchain that works for you and not against you. With sensible defaults, but even better escape hatches. Built with the same techniques used by the big guys, but in a way that doesn't require PhD to learn or a staff to maintain.
- [**Docker BuildX**](https://docs.docker.com/buildx/working-with-buildx/) - Docker Buildx is a CLI plugin that extends the docker command with the full support of the features provided by Moby BuildKit builder toolkit. It provides the same user experience as docker build with many new features like creating scoped builder instances and building against multiple nodes concurrently.


## Roadmap

- [ ] Create Roadmap

## License

Source code in this repository is covered by the MIT license.
The default throughout the repository is a license under the MIT license, unless a file header or a license file in a subdirectory specifies another license.
See the LICENSE for the full license text.
