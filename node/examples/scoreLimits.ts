import 'dotenv/config'

import { NlpClient } from '@aristech-org/nlp-client'

const auth = process.env.TOKEN && process.env.SECRET ? { token: process.env.TOKEN, secret: process.env.SECRET } : undefined

const client = new NlpClient({
  host: process.env.HOST,
  ssl: Boolean(auth) || Boolean(process.env.ROOT_CERT) || process.env.SSL === 'true',
  rootCert: process.env.ROOT_CERT,
  auth,
})
const response = await client.getScoreLimits({
  projectId: process.env.PROJECT_ID,
  testSentencesUpperLimit: [
    "What are the lottery numbers?",
    "Can you tell me the winning numbers?",
  ],
  testSentencesLowerLimit: [
    "What's the weather like?",
    "I want to speak to a human.",
  ],
})
console.log(response)