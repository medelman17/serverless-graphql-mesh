
import {ApolloServer} from 'apollo-server-lambda'
import {ApolloGateway} from '@apollo/gateway'


const gateway = new ApolloGateway();

const server = new ApolloServer({gateway});


export const handler = server.createHandler();