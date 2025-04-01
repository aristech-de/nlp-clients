import 'dotenv/config'

import { NlpClient } from '@aristech-org/nlp-client'
// util is only used to print deep objects in the console
import util from 'util'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new NlpClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})

// Get matching content from the vector database
const response = await client.getContent({
  prompt: process.argv[2] || 'What are the lottery numbers?',
  threshold: 0.5,
  numResults: 3,
  projectId: process.env.PROJECT_ID,
})
console.log(util.inspect(response, { depth: null, colors: true }))

// Update content inside the vector database
/*
const updateResponse = await client.updateContent({
  intent: [{
    id: '123',
    topic: 'lottery-numbers',
    projectId: process.env.PROJECT_ID,
    inputs: [
      { uuid: '82a2133e-038e-4c83-aa26-de47ed386c55', input: 'What are the lottery numbers?' },
      { uuid: '2cc467ca-dc1a-4d33-92ff-8f73bb941fb9', input: 'Can you tell me the numbers from saturday?' },
    ],
    outputChat: [
      'The latest lottery numbers are {{lotto_numbers}}'
    ]
  }],
})
*/

// Remove content from the vector database
/*
const removeResponse = await client.removeContent({
  id: ['82a2133e-038e-4c83-aa26-de47ed386c55'],
  projectId: process.env.PROJECT_ID,
})
*/